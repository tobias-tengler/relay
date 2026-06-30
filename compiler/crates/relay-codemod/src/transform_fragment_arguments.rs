/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Codemod that rewrites the legacy directive-based fragment-argument syntax
//! (`@argumentDefinitions` on fragment definitions and `@arguments` on fragment
//! spreads) into the native GraphQL fragment-argument syntax.
//!
//! Operates on the *source* AST (`graphql_syntax::ExecutableDefinition`) rather
//! than the IR, because the IR has already lowered `@argumentDefinitions` /
//! `@arguments` away and dropped their source spans. Each rewrite is emitted as
//! a `common::Diagnostic` carrying the replacement text, reusing the existing
//! diagnostic -> code-action -> apply pipeline (see `codemod.rs`).

use common::Diagnostic;
use common::Location;
use common::Span;
use graphql_syntax::ConstantValue;
use graphql_syntax::Directive;
use graphql_syntax::ExecutableDefinition;
use graphql_syntax::FragmentDefinition;
use graphql_syntax::FragmentSpread;
use graphql_syntax::Selection;
use intern::Lookup;
use intern::string_key::Intern;
use relay_transforms::ValidationMessageWithData;

const CODEMOD_NAME: &str = "TransformFragmentArguments";
const ARGUMENT_DEFINITIONS: &str = "argumentDefinitions";
const ARGUMENTS: &str = "arguments";
const TYPE: &str = "type";
const DEFAULT_VALUE: &str = "defaultValue";
const DIRECTIVES: &str = "directives";
const PROVIDER: &str = "provider";
const UNUSED_LOCAL_VARIABLE_DEPRECATED: &str = "unusedLocalVariable_DEPRECATED";

/// Produces one diagnostic per text edit required to migrate every fragment
/// definition and fragment spread in `definitions`.
pub fn transform_fragment_arguments(definitions: &[ExecutableDefinition]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for definition in definitions {
        match definition {
            ExecutableDefinition::Fragment(fragment) => {
                transform_fragment_definition(fragment, &mut diagnostics);
                transform_selections(&fragment.selections.items, fragment.location, &mut diagnostics);
            }
            ExecutableDefinition::Operation(operation) => {
                transform_selections(
                    &operation.selections.items,
                    operation.location,
                    &mut diagnostics,
                );
            }
        }
    }
    diagnostics
}

/// Rewrites a fragment definition's `@argumentDefinitions(...)` directive into
/// fragment variable definitions inserted after the fragment name.
fn transform_fragment_definition(fragment: &FragmentDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let directive = match find_directive(&fragment.directives, ARGUMENT_DEFINITIONS) {
        Some(directive) => directive,
        None => {
            return;
        }
    };

    // `None` means the directive uses a feature (e.g. `provider:`) that has no
    // representation in the bare fragment-variable syntax -> skip the whole
    // fragment, leaving both the directive and any usages untouched.
    let variable_definitions = match build_variable_definitions(directive) {
        Some(variable_definitions) => variable_definitions,
        None => {
            return;
        }
    };

    // (a) Insert the variable definitions immediately after the fragment name.
    let name_end = fragment.name.span.end;
    diagnostics.push(make_fix(
        fragment
            .location
            .with_span(Span::new(name_end, name_end)),
        variable_definitions,
    ));

    // (b) Delete exactly the `@argumentDefinitions(...)` directive span. We do
    // not extend backward to swallow the preceding separator because, without
    // the source text, we can't tell whether the preceding character is
    // whitespace (legal GraphQL allows e.g. `... on Api@argumentDefinitions`),
    // and eating a significant character would corrupt the document. A leftover
    // separator space before the next directive or `{` is harmless.
    diagnostics.push(make_fix(
        fragment
            .location
            .with_span(Span::new(directive.span.start, directive.span.end)),
        String::new(),
    ));
}

/// Recursively rewrites `@arguments(...)` on fragment spreads within
/// `selections`. `location` is the enclosing definition's location, which
/// provides the source file that the (span-only) selection nodes belong to.
fn transform_selections(
    selections: &[Selection],
    location: Location,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for selection in selections {
        match selection {
            Selection::FragmentSpread(spread) => {
                transform_fragment_spread(spread, location, diagnostics);
            }
            Selection::InlineFragment(inline_fragment) => {
                transform_selections(&inline_fragment.selections.items, location, diagnostics);
            }
            Selection::LinkedField(field) => {
                transform_selections(&field.selections.items, location, diagnostics);
            }
            Selection::ScalarField(_) => {}
        }
    }
}

/// Rewrites `...Name @arguments(a: $a)` into `...Name(a: $a)`.
fn transform_fragment_spread(
    spread: &FragmentSpread,
    location: Location,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let directive = match find_directive(&spread.directives, ARGUMENTS) {
        Some(directive) => directive,
        None => {
            return;
        }
    };
    let arguments = match &directive.arguments {
        Some(arguments) => arguments,
        None => {
            return;
        }
    };

    let printed_arguments = arguments
        .items
        .iter()
        .map(|argument| argument.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let name_end = spread.name.span.end;
    let arguments_is_first_directive = spread
        .directives
        .first()
        .is_some_and(|first| first.name.value.lookup() == ARGUMENTS);
    if arguments_is_first_directive {
        // `@arguments` is the first directive, so everything between the spread
        // name and the directive is just whitespace. Replace that whole range
        // (name end -> directive end) with the bare argument list in a single
        // edit, which also swallows any separator/newline so an `@arguments` on
        // its own line doesn't leave a dangling blank line behind.
        diagnostics.push(make_fix(
            location.with_span(Span::new(name_end, directive.span.end)),
            format!("({printed_arguments})"),
        ));
    } else {
        // Other directives sit before `@arguments`
        // (e.g. `...Foo @include(if: $c) @arguments(x: $x)`); preserve them by
        // emitting two independent edits:
        //   (a) insert the bare argument list immediately after the spread name, and
        //   (b) delete exactly the `@arguments(...)` directive span.
        diagnostics.push(make_fix(
            location.with_span(Span::new(name_end, name_end)),
            format!("({printed_arguments})"),
        ));
        diagnostics.push(make_fix(
            location.with_span(Span::new(directive.span.start, directive.span.end)),
            String::new(),
        ));
    }
}

/// Builds the printed fragment variable-definition list for an
/// `@argumentDefinitions(...)` directive, e.g.:
///
/// ```text
/// (
///   $count: Int = 50
///   $cursor: String
/// )
/// ```
///
/// Returns `None` if any variable cannot be represented in the bare syntax
/// (a `provider:` key, an unknown key, or an unexpected value shape), signaling
/// that the whole fragment should be skipped.
fn build_variable_definitions(directive: &Directive) -> Option<String> {
    let arguments = directive.arguments.as_ref()?;
    let mut definitions = Vec::with_capacity(arguments.items.len());

    for argument in &arguments.items {
        let object = match &argument.value {
            graphql_syntax::Value::Constant(ConstantValue::Object(object)) => object,
            _ => {
                return None;
            }
        };

        let mut type_string: Option<String> = None;
        let mut default_value: Option<String> = None;
        let mut directive_strings: Vec<String> = Vec::new();
        let mut unused_local_variable = false;

        for item in &object.items {
            match item.name.value.lookup() {
                TYPE => match &item.value {
                    ConstantValue::String(string_node) => {
                        type_string = Some(string_node.value.to_string());
                    }
                    _ => {
                        return None;
                    }
                },
                DEFAULT_VALUE => {
                    // `defaultValue: null` is equivalent to omitting the default
                    // entirely, so don't emit a redundant `= null`.
                    if !matches!(item.value, ConstantValue::Null(_)) {
                        default_value = Some(item.value.to_string());
                    }
                }
                DIRECTIVES => match &item.value {
                    ConstantValue::List(list) => {
                        for element in &list.items {
                            match element {
                                ConstantValue::String(string_node) => {
                                    directive_strings.push(string_node.value.to_string());
                                }
                                _ => {
                                    return None;
                                }
                            }
                        }
                    }
                    _ => {
                        return None;
                    }
                },
                UNUSED_LOCAL_VARIABLE_DEPRECATED => {
                    unused_local_variable = true;
                }
                // `provider:` (provided variables) and any unknown key have no
                // representation in the bare syntax -> skip the fragment.
                PROVIDER => {
                    return None;
                }
                _ => {
                    return None;
                }
            }
        }

        let type_string = type_string?;
        let mut definition = format!("${}: {}", argument.name.value.lookup(), type_string);
        if let Some(default_value) = default_value {
            definition.push_str(&format!(" = {default_value}"));
        }
        for directive_string in &directive_strings {
            definition.push(' ');
            definition.push_str(directive_string);
        }
        if unused_local_variable {
            definition.push_str(" @unusedLocalVariable_DEPRECATED");
        }
        definitions.push(definition);
    }

    let body = definitions
        .iter()
        .map(|definition| format!("  {definition}"))
        .collect::<Vec<_>>()
        .join("\n");
    Some(format!("(\n{body}\n)"))
}

fn find_directive<'a>(directives: &'a [Directive], name: &str) -> Option<&'a Directive> {
    directives
        .iter()
        .find(|directive| directive.name.value.lookup() == name)
}

fn make_fix(location: Location, fix: String) -> Diagnostic {
    Diagnostic::error_with_data(
        ValidationMessageWithData::CodemodCustomErrorWithFix {
            codemod_name: CODEMOD_NAME.intern(),
            fix,
        },
        location,
    )
}

/// Validates that every *enabled* targeted project has the
/// `enable_fragment_argument_transform` feature flag turned on. `projects`
/// yields `(project_name, enabled, flag_enabled)` tuples. Returns `Err` with a
/// user-facing message naming the flag (and the offending projects) when any
/// enabled project has it disabled, since the rewritten output only parses when
/// the flag is on.
pub fn check_fragment_argument_flag<I>(projects: I) -> Result<(), String>
where
    I: IntoIterator<Item = (String, bool, bool)>,
{
    let disabled_projects: Vec<String> = projects
        .into_iter()
        .filter(|(_, enabled, flag_enabled)| *enabled && !*flag_enabled)
        .map(|(name, _, _)| name)
        .collect();

    if disabled_projects.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "The `TransformFragmentArguments` codemod requires the `featureFlags.enable_fragment_argument_transform` compiler config flag to be enabled. Please enable it for the following project(s) before running: {}.",
            disabled_projects.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use common::SourceLocationKey;
    use graphql_syntax::parse_executable;

    use super::*;

    /// Applies the diagnostics produced by `transform_fragment_arguments`
    /// directly to `source`, mirroring what `apply_actions` does on real files
    /// but without any file I/O. Edits are applied highest-offset-first so
    /// earlier offsets stay valid.
    fn apply(source: &str) -> String {
        let document =
            parse_executable(source, SourceLocationKey::standalone("test.graphql")).unwrap();
        let diagnostics = transform_fragment_arguments(&document.definitions);

        let mut edits: Vec<(usize, usize, String)> = diagnostics
            .iter()
            .map(|diagnostic| {
                let span = diagnostic.location().span();
                let fix = diagnostic
                    .get_data()
                    .first()
                    .map(|data| data.to_string())
                    .unwrap_or_default();
                (span.start as usize, span.end as usize, fix)
            })
            .collect();
        edits.sort_by(|a, b| b.0.cmp(&a.0));

        let mut result = source.to_string();
        for (start, end, fix) in edits {
            result.replace_range(start..end, &fix);
        }
        result
    }

    #[test]
    fn migrates_argument_definitions_and_arguments_and_skips_providers() {
        let source = r#"fragment listFragment on Api
@refetchable(queryName: "listPaginationQuery")
@argumentDefinitions(
  count: { type: "Int", defaultValue: 50 }
  cursor: { type: "String" }
  search: { type: "String" }
) {
  collections {
    id
  }
}

fragment providedFragment on Api
@argumentDefinitions(
  token: { type: "String", provider: "TokenProvider.relayprovider" }
) {
  id
}

query ListQuery {
  ...listFragment @arguments(search: $search)
  ...providedFragment
}
"#;

        // The exact-span delete leaves a harmless separator space before `{`
        // (line ` {`). The ` @arguments(...)` spread rewrite consumes the
        // preceding separator, so no trailing space is left. The
        // `providedFragment` fragment is left untouched because of `provider:`.
        // Built from a line array so the source has no trailing whitespace.
        let expected = [
            "fragment listFragment(",
            "  $count: Int = 50",
            "  $cursor: String",
            "  $search: String",
            ") on Api",
            "@refetchable(queryName: \"listPaginationQuery\")",
            " {",
            "  collections {",
            "    id",
            "  }",
            "}",
            "",
            "fragment providedFragment on Api",
            "@argumentDefinitions(",
            "  token: { type: \"String\", provider: \"TokenProvider.relayprovider\" }",
            ") {",
            "  id",
            "}",
            "",
            "query ListQuery {",
            "  ...listFragment(search: $search)",
            "  ...providedFragment",
            "}",
            "",
        ]
        .join("\n");

        assert_eq!(apply(source), expected);
    }

    #[test]
    fn preserves_directives_before_arguments_on_spread() {
        let source = "query Q {\n  ...foo @include(if: $c) @arguments(x: $x)\n}\n";
        // The `@include` directive sitting before `@arguments` must be kept.
        let expected = "query Q {\n  ...foo(x: $x) @include(if: $c) \n}\n";
        assert_eq!(apply(source), expected);
    }

    /// Number of edits the transform would emit for `source`.
    fn edit_count(source: &str) -> usize {
        let document =
            parse_executable(source, SourceLocationKey::standalone("test.graphql")).unwrap();
        transform_fragment_arguments(&document.definitions).len()
    }

    #[test]
    fn skips_fragment_with_unknown_argument_definition_key() {
        // `bar` is not a recognized key -> the whole fragment is skipped, just
        // like a `provider:` key, producing no edits.
        let source =
            "fragment f on T\n@argumentDefinitions(\n  x: { type: \"Int\", bar: 1 }\n) {\n  id\n}\n";
        assert_eq!(edit_count(source), 0);
        assert_eq!(apply(source), source);
    }

    #[test]
    fn carries_over_variable_directives_and_unused_local() {
        let source = concat!(
            "fragment f on T\n",
            "@argumentDefinitions(\n",
            "  x: { type: \"Int\", directives: [\"@foo(x: 1)\"] }\n",
            "  y: { type: \"Int\", unusedLocalVariable_DEPRECATED: true }\n",
            ") {\n",
            "  id\n",
            "}\n",
        );
        let expected = [
            "fragment f(",
            "  $x: Int @foo(x: 1)",
            "  $y: Int @unusedLocalVariable_DEPRECATED",
            ") on T",
            " {",
            "  id",
            "}",
            "",
        ]
        .join("\n");
        assert_eq!(apply(source), expected);
    }

    #[test]
    fn preserves_complex_types_and_default_values() {
        // Covers: a var with no defaultValue (`$a: String`), a verbatim complex
        // type (`[Int!]!`), and non-scalar default values (list, enum, object)
        // round-tripped via `Display`.
        let source = concat!(
            "fragment f on T\n",
            "@argumentDefinitions(\n",
            "  a: { type: \"String\" }\n",
            "  b: { type: \"[Int!]!\" }\n",
            "  c: { type: \"[String]\", defaultValue: [\"a\", \"b\"] }\n",
            "  d: { type: \"E\", defaultValue: SOME_ENUM }\n",
            "  e: { type: \"In\", defaultValue: { a: 1 } }\n",
            ") {\n",
            "  id\n",
            "}\n",
        );
        let expected = [
            "fragment f(",
            "  $a: String",
            "  $b: [Int!]!",
            "  $c: [String] = [\"a\", \"b\"]",
            "  $d: E = SOME_ENUM",
            "  $e: In = {a: 1}",
            ") on T",
            " {",
            "  id",
            "}",
            "",
        ]
        .join("\n");
        assert_eq!(apply(source), expected);
    }

    #[test]
    fn cleans_up_arguments_on_its_own_line() {
        // `@arguments` on a separate line must not leave a dangling blank line.
        let source = concat!(
            "query Q {\n",
            "  ...foo\n",
            "  @arguments(from: $from, to: $to)\n",
            "  ...bar\n",
            "}\n",
        );
        let expected = [
            "query Q {",
            "  ...foo(from: $from, to: $to)",
            "  ...bar",
            "}",
            "",
        ]
        .join("\n");
        assert_eq!(apply(source), expected);
    }

    #[test]
    fn omits_null_default_value() {
        // `defaultValue: null` must not produce a redundant `= null`.
        let source = concat!(
            "fragment f on T\n",
            "@argumentDefinitions(\n",
            "  a: { type: \"String\", defaultValue: null }\n",
            ") {\n",
            "  id\n",
            "}\n",
        );
        let expected = [
            "fragment f(",
            "  $a: String",
            ") on T",
            " {",
            "  id",
            "}",
            "",
        ]
        .join("\n");
        assert_eq!(apply(source), expected);
    }

    #[test]
    fn no_edits_when_nothing_to_migrate() {
        let source =
            "fragment f on T {\n  id\n}\n\nquery Q {\n  ...f\n  field\n}\n";
        assert_eq!(edit_count(source), 0);
        assert_eq!(apply(source), source);
    }

    #[test]
    fn check_fragment_argument_flag_errors_when_enabled_project_lacks_flag() {
        // Enabled project without the flag -> error naming the flag and project.
        let result = check_fragment_argument_flag(vec![
            ("app".to_string(), true, false),
            ("disabled_app".to_string(), false, false),
        ]);
        let message = result.expect_err("expected the flag guard to error");
        assert!(message.contains("featureFlags.enable_fragment_argument_transform"));
        assert!(message.contains("app"));
        // A disabled project must not be reported.
        assert!(!message.contains("disabled_app"));
    }

    #[test]
    fn check_fragment_argument_flag_ok_when_all_enabled_projects_have_flag() {
        // Enabled project with the flag, plus a disabled project without it.
        let result = check_fragment_argument_flag(vec![
            ("app".to_string(), true, true),
            ("legacy".to_string(), false, false),
        ]);
        assert!(result.is_ok());
    }
}
