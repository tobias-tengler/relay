/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::vec;

use clap::Args;
use clap::Subcommand;
use common::Diagnostic;
use common::DiagnosticsResult;
use common::FeatureFlag;
use common::Rollout;
use common::RolloutRange;
use log::info;
use lsp_types::CodeActionOrCommand;
use lsp_types::TextEdit;
use lsp_types::Uri;
use relay_compiler::errors::BuildProjectError;
use relay_compiler::errors::Error as CompilerError;
use relay_compiler::errors::Result as CompilerResult;
use relay_transforms::Programs;
use relay_transforms::disallow_required_on_non_null_field;
use relay_transforms::fragment_alias_directive;

#[derive(Subcommand, Debug, Clone)]
pub enum AvailableCodemod {
    /// Marks unaliased conditional fragment spreads as @dangerously_unaliased_fixme
    MarkDangerousConditionalFragmentSpreads(MarkDangerousConditionalFragmentSpreadsArgs),

    /// Removes @required directives from non-null fields within @throwOnFieldError fragments and operations.
    RemoveUnnecessaryRequiredDirectives,

    /// Runs all Relay compiler transforms and fixes all fixable diagnostics
    FixAll,

    /// Rewrites the legacy directive-based fragment-argument syntax
    /// (`@argumentDefinitions` / `@arguments`) into the native GraphQL
    /// fragment-argument syntax. Requires the
    /// `featureFlags.enable_fragment_argument_transform` compiler flag.
    ///
    /// Handled directly in `relay-bin`'s `handle_codemod_command` (it operates
    /// on the source AST + needs the feature-flag check) rather than via
    /// `run_codemod`.
    TransformFragmentArguments,
}

#[derive(Args, Debug, Clone)]
pub struct MarkDangerousConditionalFragmentSpreadsArgs {
    /// Specify a percentage of fragments to codemod. If a number is provided,
    /// the first n percentage of fragments will be codemodded. If a range (`20-30`) is
    /// provided, then fragments between the start and end of the range will be codemodded.
    #[clap(long, short, value_parser=valid_percent, default_value = "100")]
    pub rollout_percentage: FeatureFlag,
}

pub async fn run_codemod(
    programs: CompilerResult<Vec<Arc<Programs>>>,
    root_dir: PathBuf,
    codemod: AvailableCodemod,
) -> Result<(), std::io::Error> {
    match &codemod {
        AvailableCodemod::MarkDangerousConditionalFragmentSpreads(opts) => {
            run_codemod_impl(
                programs.expect("Failed to build programs"),
                root_dir,
                |programs: &Arc<Programs>| {
                    fragment_alias_directive(&programs.source, &opts.rollout_percentage).map(|_| ())
                }, // Codemods don't return anything for OK,
                format!("{codemod:?}").as_str(),
            )
            .await
        }
        AvailableCodemod::RemoveUnnecessaryRequiredDirectives => {
            run_codemod_impl(
                programs.expect("Failed to build programs"),
                root_dir,
                |programs: &Arc<Programs>| disallow_required_on_non_null_field(&programs.reader),
                format!("{codemod:?}").as_str(),
            )
            .await
        }
        AvailableCodemod::TransformFragmentArguments => {
            // Handled directly in `handle_codemod_command` because it operates
            // on the source AST and needs the feature-flag check rather than
            // the compiled `Programs`.
            unreachable!(
                "TransformFragmentArguments is handled before run_codemod in handle_codemod_command"
            )
        }
        AvailableCodemod::FixAll => {
            match programs {
                Ok(_programs) => {
                    // Noop
                    Ok(())
                }
                Err(error) => {
                    let diagnostics = as_diagnostics(error);
                    fix_diagnostics("FixAll", &root_dir, &diagnostics)
                }
            }
        }
    }
}

fn as_diagnostics(error: CompilerError) -> Vec<Diagnostic> {
    match error {
        CompilerError::DiagnosticsError { errors } => errors,
        CompilerError::BuildProjectsErrors { errors } => errors
            .into_iter()
            .flat_map(|e| match e {
                BuildProjectError::ValidationErrors { errors, .. } => errors,
                _ => vec![],
            })
            .collect(),
        _ => vec![],
    }
}

pub async fn run_codemod_impl(
    programs: Vec<Arc<Programs>>,
    root_dir: PathBuf,
    f: impl Fn(&Arc<Programs>) -> DiagnosticsResult<()>,
    codemod: &str,
) -> Result<(), std::io::Error> {
    let diagnostics = programs
        .iter()
        .flat_map(|programs| {
            let result = f(programs);
            match result {
                Ok(_) => vec![],
                Err(e) => e,
            }
        })
        .collect::<Vec<_>>();

    fix_diagnostics(codemod, &root_dir, &diagnostics)
}

pub fn fix_diagnostics(
    codemod: &str,
    root_dir: &Path,
    diagnostics: &[Diagnostic],
) -> Result<(), std::io::Error> {
    let actions = relay_lsp::diagnostics_to_code_actions(root_dir, diagnostics);

    info!(
        "Codemod {:?} ran and found {} changes to make.",
        codemod,
        actions.len()
    );
    apply_actions(actions)?;
    Ok(())
}

fn apply_actions(actions: Vec<CodeActionOrCommand>) -> Result<(), std::io::Error> {
    let mut collected_changes = std::collections::HashMap::new();

    // Collect all the changes into a map of file-to-list-of-changes
    for action in actions {
        if let CodeActionOrCommand::CodeAction(code_action) = action
            && let Some(changes) = code_action.edit.unwrap().changes
        {
            for (file, changes) in changes {
                collected_changes
                    .entry(file)
                    .or_insert_with(Vec::new)
                    .extend(changes);
            }
        }
    }

    // Sort and validate the changes for EVERY file before writing any of them.
    // This avoids leaving the working tree in a half-rewritten state when one
    // file's changes overlap (which aborts the run): either every file is
    // written, or none is.
    let mut sorted_changes: Vec<(Uri, Vec<TextEdit>)> = Vec::new();
    for (file, mut changes) in collected_changes {
        sort_changes(&file, &mut changes)?;
        sorted_changes.push((file, changes));
    }

    for (file, changes) in sorted_changes {
        // Read file into memory and apply changes. Changes are applied in
        // reverse order (highest position first) so that splicing in
        // replacements — which can change the number of lines — does not
        // invalidate the line indices of edits that come earlier in the file.
        let file_contents: String = fs::read_to_string(file.path().as_str())?;
        let mut lines: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();
        for change in changes.iter().rev() {
            let start_line = change.range.start.line as usize;
            let end_line = change.range.end.line as usize;
            let start_character = change.range.start.character as usize;
            let end_character = change.range.end.character as usize;

            // Take the prefix from the start line and the suffix from the end
            // line, splicing the new text (which may itself span multiple
            // lines) in between and dropping any lines fully covered by the
            // range.
            let mut spliced = String::new();
            spliced.push_str(&lines[start_line][..start_character]);
            spliced.push_str(&change.new_text);
            spliced.push_str(&lines[end_line][end_character..]);

            let replacement_lines: Vec<String> =
                spliced.split('\n').map(|s| s.to_string()).collect();
            lines.splice(start_line..=end_line, replacement_lines);
        }

        // Write file back out
        let new_file_contents = lines.join("\n");
        fs::write(file.path().as_str(), new_file_contents)?;

        info!("Applied {} changes to {}", changes.len(), file.path());
    }
    Ok(())
}

fn sort_changes(uri: &Uri, changes: &mut [TextEdit]) -> Result<(), std::io::Error> {
    // Sort the changes by their start position within the file. They are later
    // applied in reverse order so that splicing earlier edits doesn't shift the
    // positions of edits that come before them in the file.
    changes.sort_by_key(|change| change.range.start);

    // Verify none of the changes overlap: a change overlaps its predecessor if
    // it starts before the predecessor ends.
    let mut prev_change: Option<&TextEdit> = None;
    for change in changes.iter() {
        if let Some(prev_change) = prev_change
            && change.range.start < prev_change.range.end
        {
            return Err(std::io::Error::other(format!(
                "Codemod produced changes that overlap: File {}, changes: {:?} vs {:?}",
                uri.path(),
                change,
                prev_change
            )));
        }
        prev_change = Some(change);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use lsp_types::CodeAction;
    use lsp_types::CodeActionOrCommand;
    use lsp_types::Position;
    use lsp_types::Range;
    use lsp_types::TextEdit;
    use lsp_types::Uri;
    use lsp_types::WorkspaceEdit;

    use super::*;

    fn code_action_for(uri: &Uri, edits: Vec<TextEdit>) -> CodeActionOrCommand {
        let mut changes = std::collections::HashMap::new();
        changes.insert(uri.clone(), edits);
        CodeActionOrCommand::CodeAction(CodeAction {
            title: "test".to_string(),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                document_changes: None,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[test]
    fn applies_multi_line_range_edit() {
        let source = "fragment listFragment on Api\n@argumentDefinitions(\n  count: { type: \"Int\" }\n) {\n  id\n}\n";

        let mut temp_path = std::env::temp_dir();
        temp_path.push(format!("relay_codemod_apply_test_{}.graphql", std::process::id()));
        fs::write(&temp_path, source).unwrap();

        let uri = Uri::from_str(&format!("file://{}", temp_path.to_string_lossy())).unwrap();

        // Insert variable definitions after the fragment name (single-line,
        // multi-line replacement text) ...
        let insert = TextEdit {
            range: Range::new(Position::new(0, 21), Position::new(0, 21)),
            new_text: "(\n  $count: Int\n)".to_string(),
        };
        // ... and delete the multi-line `@argumentDefinitions(...)` directive
        // including the preceding newline (range spans lines 0..3).
        let delete = TextEdit {
            range: Range::new(Position::new(0, 28), Position::new(3, 1)),
            new_text: String::new(),
        };

        apply_actions(vec![code_action_for(&uri, vec![insert, delete])]).unwrap();

        let result = fs::read_to_string(&temp_path).unwrap();
        fs::remove_file(&temp_path).ok();

        assert_eq!(
            result,
            "fragment listFragment(\n  $count: Int\n) on Api {\n  id\n}"
        );
    }

    /// End-to-end test through the real pipeline: the fragment-argument
    /// transform emits diagnostics, `diagnostics_to_code_actions` maps their
    /// spans to file line/character ranges, and `apply_actions` rewrites the
    /// file on disk. Covers the multi-line `@argumentDefinitions` block and a
    /// spread carrying a directive before `@arguments` (which must be kept).
    #[test]
    fn transform_fragment_arguments_end_to_end() {
        let source = concat!(
            "fragment fooFragment on Api\n",
            "@argumentDefinitions(\n",
            "  count: { type: \"Int\", defaultValue: 10 }\n",
            ") {\n",
            "  id\n",
            "}\n",
            "\n",
            "query FooQuery {\n",
            "  ...fooFragment @include(if: $cond) @arguments(count: $count)\n",
            "}\n",
        );

        let mut dir = std::env::temp_dir();
        dir.push(format!("relay_codemod_e2e_test_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file_name = "fooFragment.graphql";
        let file_path = dir.join(file_name);
        fs::write(&file_path, source).unwrap();

        let document = graphql_syntax::parse_executable(
            source,
            common::SourceLocationKey::standalone(file_name),
        )
        .unwrap();
        let diagnostics = crate::transform_fragment_arguments(&document.definitions);

        let actions = relay_lsp::diagnostics_to_code_actions(&dir, &diagnostics);
        apply_actions(actions).unwrap();

        let result = fs::read_to_string(&file_path).unwrap();
        fs::remove_dir_all(&dir).ok();

        // `apply_actions` rebuilds the file from `lines().join("\n")`, so the
        // original trailing newline is dropped. The `@include` directive is
        // preserved before the migrated argument list.
        let expected = [
            "fragment fooFragment(",
            "  $count: Int = 10",
            ") on Api",
            " {",
            "  id",
            "}",
            "",
            "query FooQuery {",
            "  ...fooFragment(count: $count) @include(if: $cond) ",
            "}",
        ]
        .join("\n");

        assert_eq!(result, expected);
    }
}

fn valid_percent(s: &str) -> Result<FeatureFlag, String> {
    // If the string is a range of the form "x-y", where x and y are numbers, return the range
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() == 2 {
        let start = parts[0].parse::<u8>().map_err(|_| {
            "Expected the value on the left of the rollout range to be a number".to_string()
        })?;
        let end = parts[1].parse::<u8>().map_err(|_| {
            "Expected the value on the right of the rollout range to be a number".to_string()
        })?;
        if (0..=100).contains(&start) && (0..=100).contains(&end) && start <= end {
            Ok(FeatureFlag::RolloutRange {
                rollout: RolloutRange { start, end },
            })
        } else {
            Err("numbers must be between 0 and 100, inclusive, and the first number must be less than or equal to the second".to_string())
        }
    } else {
        // turn s into a u8
        let s = s.parse::<u8>().map_err(|_| "not a number".to_string())?;
        // check if s is less than 100
        if (0..=100).contains(&s) {
            Ok(FeatureFlag::Rollout {
                rollout: Rollout(Some(s)),
            })
        } else {
            Err("number must be between 0 and 100, inclusive".to_string())
        }
    }
}
