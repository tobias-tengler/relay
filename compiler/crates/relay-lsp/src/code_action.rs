/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod create_name_suggestion;

use std::collections::HashMap;
use std::collections::HashSet;

use common::Location;
use common::Span;
use create_name_suggestion::create_default_name;
use create_name_suggestion::create_default_name_with_index;
use create_name_suggestion::create_impactful_name;
use create_name_suggestion::create_name_wrapper;
use create_name_suggestion::DefinitionNameSuffix;
use docblock_shared::ARGUMENT_DEFINITIONS;
use graphql_ir::ValidationDiagnosticCode;
use graphql_syntax::ExecutableDefinition;
use graphql_syntax::ExecutableDocument;
use intern::Lookup;
use itertools::Itertools;
use lsp_types::request::CodeActionRequest;
use lsp_types::request::Request;
use lsp_types::CodeAction;
use lsp_types::CodeActionOrCommand;
use lsp_types::Diagnostic;
use lsp_types::NumberOrString;
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::TextDocumentPositionParams;
use lsp_types::TextEdit;
use lsp_types::Url;
use lsp_types::WorkspaceEdit;
use resolution_path::IdentParent;
use resolution_path::IdentPath;
use resolution_path::OperationDefinitionPath;
use resolution_path::ResolutionPath;
use resolution_path::ResolvePosition;
use serde_json::Value;

use crate::lsp_runtime_error::LSPRuntimeError;
use crate::lsp_runtime_error::LSPRuntimeResult;
use crate::server::GlobalState;
use crate::utils::is_file_uri_in_dir;

pub(crate) fn on_code_action(
    state: &impl GlobalState,
    params: <CodeActionRequest as Request>::Params,
) -> LSPRuntimeResult<<CodeActionRequest as Request>::Result> {
    let uri = params.text_document.uri.clone();

    if !is_file_uri_in_dir(state.root_dir(), &uri) {
        return Err(LSPRuntimeError::ExpectedError);
    }

    let text_document_position_params = TextDocumentPositionParams {
        text_document: params.text_document,
        position: params.range.start,
    };
    let (document, location) =
        state.extract_executable_document_from_text(&text_document_position_params, 1)?;

    if let Some(diagnostic) = state.get_diagnostic_for_range(&uri, params.range) {
        let code_actions =
            get_code_actions_from_diagnostic(&uri, diagnostic, &document, &location, state);
        if code_actions.is_some() {
            return Ok(code_actions);
        }
    }

    let path = document.resolve((), location.span());
    let definitions = state.resolve_executable_definitions(&uri)?;
    let used_definition_names = get_definition_names(&definitions);

    get_code_actions(path, used_definition_names, uri, params.range)
        .map(|code_actions| Some(code_actions))
        .ok_or(LSPRuntimeError::ExpectedError)
}

fn get_code_actions_from_diagnostic(
    url: &Url,
    diagnostic: Diagnostic,
    document: &ExecutableDocument,
    location: &Location,
    state: &impl GlobalState,
) -> Option<Vec<CodeActionOrCommand>> {
    match diagnostic {
        Diagnostic {
            code:
                Some(NumberOrString::Number(
                    ValidationDiagnosticCode::EXPECTED_OPERATION_VARIABLE_TO_BE_DEFINED
                    | ValidationDiagnosticCode::UNDEFINED_VARIABLE_REFERENCED,
                )),
            data: Some(Value::Array(array_data)),
            ..
        } => {
            let definition = document.find_definition(location.span())?;

            match &array_data[..] {
                [Value::String(variable_name), Value::String(variable_type)] => match definition {
                    ExecutableDefinition::Operation(operation) => {
                        create_operation_variable_code_action(
                            operation,
                            variable_name,
                            variable_type,
                            location,
                            state,
                            url.to_owned(),
                        )
                        .and_then(|code_action| Some(vec![code_action]))
                    }
                    ExecutableDefinition::Fragment(fragment) => {
                        create_fragment_argument_code_action(
                            fragment,
                            variable_name,
                            variable_type,
                            location,
                            state,
                            url.to_owned(),
                        )
                        .and_then(|code_action| Some(vec![code_action]))
                    }
                },
                _ => None,
            }
        }
        Diagnostic {
            data: Some(Value::Array(array_data)),
            ..
        } => Some(
            array_data
                .iter()
                .filter_map(|item| match item {
                    Value::String(suggestion) => Some(create_code_action(
                        "Fix Error",
                        suggestion.to_string(),
                        url,
                        diagnostic.range,
                    )),
                    _ => None,
                })
                .collect_vec(),
        ),
        _ => None,
    }
}

struct FragmentAndOperationNames {
    operation_names: HashSet<String>,
    _fragment_names: HashSet<String>,
}

fn get_definition_names(definitions: &[ExecutableDefinition]) -> FragmentAndOperationNames {
    let mut operation_names = HashSet::new();
    let mut fragment_names = HashSet::new();
    for definition in definitions.iter() {
        match definition {
            ExecutableDefinition::Operation(operation) => {
                if let Some(name) = &operation.name {
                    operation_names.insert(name.value.lookup().to_string());
                }
            }
            ExecutableDefinition::Fragment(fragment) => {
                fragment_names.insert(fragment.name.value.lookup().to_string());
            }
        }
    }

    FragmentAndOperationNames {
        operation_names,
        _fragment_names: fragment_names,
    }
}

fn get_code_actions(
    path: ResolutionPath<'_>,
    used_definition_names: FragmentAndOperationNames,
    url: Url,
    range: Range,
) -> Option<Vec<CodeActionOrCommand>> {
    match path {
        ResolutionPath::Ident(IdentPath {
            inner: _,
            parent:
                IdentParent::OperationDefinitionName(OperationDefinitionPath {
                    inner: operation_definition,
                    parent: _,
                }),
        }) => {
            let suffix = if let Some((_, operation_kind)) = operation_definition.operation {
                DefinitionNameSuffix::from(&operation_kind)
            } else {
                return None;
            };

            let operation_name = if let Some(operation_name) = &operation_definition.name {
                operation_name
            } else {
                return None;
            };

            let code_action_range = get_code_action_range(range, operation_name.span);
            Some(create_code_actions(
                "Rename Operation",
                operation_name.value.lookup(),
                used_definition_names.operation_names,
                suffix,
                &url,
                code_action_range,
            ))
        }
        _ => None,
    }
}

// TODO: Rename
pub trait Bruh {
    fn find_definition(&self, position_span: Span) -> Option<&ExecutableDefinition>;
}

impl Bruh for ExecutableDocument {
    fn find_definition(&self, position_span: Span) -> Option<&ExecutableDefinition> {
        self.definitions
            .iter()
            .find(|definition| definition.contains(position_span))
    }
}

fn create_operation_variable_text_edit(
    operation: &graphql_syntax::OperationDefinition,
    variable_name: &str,
    variable_type: &str,
    location: &Location,
    state: &impl GlobalState,
) -> Option<TextEdit> {
    if operation.variable_definitions.is_none() {
        operation
            .name
            .and_then(|operation_name| {
                state
                    .get_lsp_location(location.with_span(Span {
                        start: operation_name.span.end,
                        end: operation_name.span.end,
                    }))
                    .ok()
            })
            .map(|lsp_location| TextEdit {
                range: lsp_location.range,
                new_text: format!(
                    "(${name}: {type})",
                    name = variable_name,
                    type = variable_type
                ),
            })
    } else {
        operation
            .variable_definitions
            .as_ref()
            .and_then(|variable_definitions| variable_definitions.items.last())
            .and_then(|last_variable| {
                state
                    .get_lsp_location(location.with_span(Span {
                        start: last_variable.span.end,
                        end: last_variable.span.end,
                    }))
                    .ok()
            })
            .map(|lsp_location| TextEdit {
                range: lsp_location.range,
                new_text: format!(
                    ", ${name}: {type}",
                    name = variable_name,
                    type = variable_type
                ),
            })
    }
}

fn create_operation_variable_code_action(
    operation: &graphql_syntax::OperationDefinition,
    variable_name: &str,
    variable_type: &str,
    location: &Location,
    state: &impl GlobalState,
    url: Url,
) -> Option<CodeActionOrCommand> {
    create_operation_variable_text_edit(operation, variable_name, variable_type, location, state)
        .map(|text_edit| {
            let mut changes = HashMap::new();
            changes.insert(url, vec![text_edit]);

            CodeActionOrCommand::CodeAction(CodeAction {
                title: format!("Create operation variable '${}'", variable_name),
                kind: Some(lsp_types::CodeActionKind::QUICKFIX),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    ..Default::default()
                }),
                command: None,
                is_preferred: Some(true),
                ..Default::default()
            })
        })
}

fn create_fragment_argument_text_edit(
    fragment: &graphql_syntax::FragmentDefinition,
    variable_name: &str,
    variable_type: &str,
    location: &Location,
    state: &impl GlobalState,
) -> Option<TextEdit> {
    if let Some(argument_definitions_directive) = fragment
        .directives
        .iter()
        .find(|directive| directive.name.value == ARGUMENT_DEFINITIONS.0)
    {
        argument_definitions_directive
            .arguments
            .as_ref()
            .and_then(|arguments| arguments.items.last())
            .and_then(|argument| {
                state
                    .get_lsp_location(location.with_span(Span {
                        start: argument.span.end,
                        end: argument.span.end,
                    }))
                    .ok()
            })
            .map(|lsp_location| TextEdit {
                range: lsp_location.range,
                new_text: format!(
                    ", {name}: {{ type: \"{type}\" }}",
                    name = variable_name,
                    type = variable_type
                ),
            })
    } else {
        state
            .get_lsp_location(location.with_span(Span {
                start: fragment.type_condition.span.end,
                end: fragment.type_condition.span.end,
            }))
            .ok()
            .map(|lsp_location| TextEdit {
                range: lsp_location.range,
                new_text: format!(
                    " @{directive_name}({name}: {{ type: \"{type}\" }})",
                    directive_name = ARGUMENT_DEFINITIONS.0,
                    name = variable_name,
                    type = variable_type
                ),
            })
    }
}

fn create_fragment_argument_code_action(
    fragment: &graphql_syntax::FragmentDefinition,
    variable_name: &str,
    variable_type: &str,
    location: &Location,
    state: &impl GlobalState,
    url: Url,
) -> Option<CodeActionOrCommand> {
    create_fragment_argument_text_edit(fragment, variable_name, variable_type, location, state).map(
        |text_edit| {
            let mut changes = HashMap::new();
            changes.insert(url, vec![text_edit]);

            CodeActionOrCommand::CodeAction(CodeAction {
                title: format!("Create fragment argument '${}'", variable_name),
                kind: Some(lsp_types::CodeActionKind::QUICKFIX),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    ..Default::default()
                }),
                command: None,
                is_preferred: Some(true),
                ..Default::default()
            })
        },
    )
}

fn create_code_actions(
    title: &str,
    original_name: &str,
    used_names: HashSet<String>,
    suffix: DefinitionNameSuffix,
    url: &Url,
    range: Range,
) -> Vec<CodeActionOrCommand> {
    let mut suggested_names = Vec::with_capacity(4);
    suggested_names.push(create_default_name(url.path(), suffix));
    suggested_names.push(create_default_name_with_index(
        url.path(),
        suffix,
        &used_names,
    ));
    suggested_names.push(create_name_wrapper(original_name, url.path(), suffix));
    suggested_names.push(create_impactful_name(url.path(), suffix));
    suggested_names
        .iter()
        .filter_map(|suggested_name| {
            if let Some(name) = suggested_name {
                if used_names.contains(name) {
                    return None;
                }

                Some(create_code_action(title, name.clone(), url, range))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn get_code_action_range(range: Range, span: Span) -> Range {
    Range {
        start: Position {
            line: range.start.line,
            character: (span.start - 1),
        },
        end: Position {
            line: range.start.line,
            character: (span.end - 1),
        },
    }
}

fn create_code_action(
    title: &str,
    new_name: String,
    url: &Url,
    range: Range,
) -> CodeActionOrCommand {
    let mut changes = HashMap::new();
    let title = format!("{}: '{}'", title, &new_name);
    let text_edit = TextEdit {
        range,
        new_text: new_name,
    };
    changes.insert(url.clone(), vec![text_edit]);

    CodeActionOrCommand::CodeAction(CodeAction {
        title,
        kind: Some(lsp_types::CodeActionKind::QUICKFIX),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            ..Default::default()
        }),
        command: None,
        is_preferred: Some(false),
        ..Default::default()
    })
}

// #[cfg(test)]
// mod tests {
//     use lsp_types::CodeActionOrCommand;
//     use lsp_types::Diagnostic;
//     use lsp_types::Position;
//     use lsp_types::Range;
//     use lsp_types::Url;
//     use serde_json::json;

//     use crate::code_action::get_code_actions_from_diagnostic;

//     #[test]
//     fn test_get_code_actions_from_diagnostics() {
//         let diagnostic = Diagnostic {
//             range: Range {
//                 start: Position {
//                     line: 0,
//                     character: 0,
//                 },
//                 end: Position {
//                     line: 0,
//                     character: 0,
//                 },
//             },
//             message: "Error Message".to_string(),
//             data: Some(json!(vec!["item1", "item2"])),
//             ..Default::default()
//         };
//         let url = Url::parse("file://relay.js").unwrap();
//         let code_actions = get_code_actions_from_diagnostic(&url, diagnostic);

//         assert_eq!(
//             code_actions
//                 .unwrap()
//                 .iter()
//                 .map(|item| {
//                     match item {
//                         CodeActionOrCommand::CodeAction(action) => action.title.clone(),
//                         _ => panic!("unexpected case"),
//                     }
//                 })
//                 .collect::<Vec<String>>(),
//             vec![
//                 "Fix Error: 'item1'".to_string(),
//                 "Fix Error: 'item2'".to_string(),
//             ]
//         );
//     }
// }
