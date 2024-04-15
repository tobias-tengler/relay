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
use common::SourceLocationKey;
use common::Span;
use create_name_suggestion::create_default_name;
use create_name_suggestion::create_default_name_with_index;
use create_name_suggestion::create_impactful_name;
use create_name_suggestion::create_name_wrapper;
use create_name_suggestion::DefinitionNameSuffix;
use graphql_syntax::ExecutableDefinition;
use graphql_syntax::ExecutableDocument;
use intern::string_key::StringKey;
use intern::Lookup;
use lsp_types::request::CodeActionRequest;
use lsp_types::request::Request;
use lsp_types::CodeAction;
use lsp_types::CodeActionOrCommand;
use lsp_types::Diagnostic;
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
use resolution_path::VariableIdentifierParent;
use resolution_path::VariableIdentifierPath;
use serde_json::Value;

use crate::lsp_runtime_error::LSPRuntimeError;
use crate::lsp_runtime_error::LSPRuntimeResult;
use crate::server::GlobalState;
use crate::utils::is_file_uri_in_dir;
use crate::FeatureResolutionInfo;

pub(crate) fn on_code_action(
    state: &impl GlobalState,
    params: <CodeActionRequest as Request>::Params,
) -> LSPRuntimeResult<<CodeActionRequest as Request>::Result> {
    let uri = params.text_document.uri.clone();

    if !is_file_uri_in_dir(state.root_dir(), &uri) {
        return Err(LSPRuntimeError::ExpectedError);
    }

    if let Some(diagnostic) = state.get_diagnostic_for_range(&uri, params.range) {
        let code_actions = get_code_actions_from_diagnostics(&uri, diagnostic);
        if code_actions.is_some() {
            return Ok(code_actions);
        }
    }

    let definitions = state.resolve_executable_definitions(&params.text_document.uri)?;

    let text_document_position_params = TextDocumentPositionParams {
        text_document: params.text_document,
        position: params.range.start,
    };
    let (document, position_span) =
        state.extract_executable_document_from_text(&text_document_position_params, 1)?;

    let path = document.resolve((), position_span);

    let feature_resolution_info = state.resolve_node(&text_document_position_params)?;
    // let project_name = state.extract_project_name_from_url(&uri)?;
    // let schema = state.get_schema(&project_name)?;

    match feature_resolution_info {
        FeatureResolutionInfo::GraphqlNode(node_resolution_info) => {
            // If type_path is empty, type_path.resolve_current_field() will panic.
            if !node_resolution_info.type_path.0.is_empty() {
                // let type_and_field = node_resolution_info
                //     .type_path
                //     .resolve_current_field_argument(&schema);

                // info!("type_and_field: {:?}", type_and_field);
                // if let Some((_parent_type, field)) = type_and_field {
                //     let type_name = schema.get_type_name(field.type_.inner()).to_string();
                //     // TODO resolve enclosing types, not just types immediately under the cursor
                //     return Ok(ResolvedTypesAtLocationResponse {
                //         path_and_schema_name: PathAndSchemaName {
                //             path: vec![type_name],
                //             schema_name,
                //         },
                //     });
                // }
            }
        }
        _ => {}
    };

    let used_definition_names = get_definition_names(&definitions);
    let result = get_code_actions(
        &document,
        path,
        used_definition_names,
        uri,
        params.range,
        position_span,
        state,
    )
    .ok_or(LSPRuntimeError::ExpectedError)?;
    Ok(Some(result))
}

fn get_code_actions_from_diagnostics(
    url: &Url,
    diagnostic: Diagnostic,
) -> Option<Vec<CodeActionOrCommand>> {
    // diagnostic.code

    let code_actions = if let Some(Value::Array(data)) = &diagnostic.data {
        data.iter()
            .filter_map(|item| match item {
                Value::String(suggestion) => Some(create_code_action(
                    "Fix Error",
                    suggestion.to_string(),
                    url,
                    diagnostic.range,
                )),
                _ => None,
            })
            .collect::<_>()
    } else {
        vec![]
    };

    if !code_actions.is_empty() {
        Some(code_actions)
    } else {
        None
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

fn create_operation_variable(
    operation: &graphql_syntax::OperationDefinition,
    variable_name: StringKey,
    location: &Location,
    state: &impl GlobalState,
    url: Url,
) -> Option<CodeActionOrCommand> {
    let variable_type = "Boolean!";
    let text_edit = if operation.variable_definitions.is_none() {
        if let Some(operation_name) = operation.name {
            let end_of_name_location = location.with_span(Span {
                start: operation_name.span.end,
                end: operation_name.span.end,
            });

            let lsp_location = state.get_lsp_location(end_of_name_location).unwrap();

            Some(TextEdit {
                range: lsp_location.range,
                new_text: format!(
                    "(${name}: {type})",
                    name = variable_name.lookup(),
                    type = variable_type
                ),
            })
        } else {
            None
        }
    } else {
        let last_variable = operation
            .variable_definitions
            .as_ref()
            .unwrap()
            .items
            .last()
            .unwrap();

        let end_of_last_variable = location.with_span(Span {
            start: last_variable.span.end,
            end: last_variable.span.end,
        });

        let lsp_location = state.get_lsp_location(end_of_last_variable).unwrap();

        Some(TextEdit {
            range: lsp_location.range,
            new_text: format!(
                ", ${name}: {type}",
                name = variable_name.lookup(),
                type = variable_type
            ),
        })
    };

    match text_edit {
        Some(text_edit) => {
            let mut changes = HashMap::new();
            changes.insert(url, vec![text_edit]);

            Some(CodeActionOrCommand::CodeAction(CodeAction {
                title: format!("Create variable ${}", variable_name),
                kind: Some(lsp_types::CodeActionKind::REFACTOR_EXTRACT),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    ..Default::default()
                }),
                command: None,
                is_preferred: Some(true),
                ..Default::default()
            }))
        }
        None => None,
    }
}

fn get_code_actions(
    document: &ExecutableDocument,
    path: ResolutionPath<'_>,
    used_definition_names: FragmentAndOperationNames,
    url: Url,
    range: Range,
    position_span: Span,
    state: &impl GlobalState,
) -> Option<Vec<CodeActionOrCommand>> {
    match path {
        ResolutionPath::VariableIdentifier(VariableIdentifierPath {
            inner: variable_identifier,
            parent: VariableIdentifierParent::Value(_),
        }) => {
            let definition = document.find_definition(position_span)?;

            let location = Location::new(
                SourceLocationKey::embedded(url.path(), 4),
                Span { start: 0, end: 0 },
            );

            match definition {
                ExecutableDefinition::Operation(operation) => {
                    if !operation.variable_definitions.iter().any(|variable| {
                        variable
                            .items
                            .iter()
                            .any(|item| item.name.name == variable_identifier.name)
                    }) {
                        if let Some(code_action) = create_operation_variable(
                            &operation,
                            variable_identifier.name,
                            &location,
                            state,
                            url,
                        ) {
                            Some(vec![code_action])
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                ExecutableDefinition::Fragment(fragment) => {
                    if fragment.variable_definitions.iter().any(|variable| {
                        variable
                            .items
                            .iter()
                            .any(|item| item.name.name == variable_identifier.name)
                    }) {
                        None
                    } else {
                        // TODO: This is not entirely correct, since it could be a global variable
                        None
                    }
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use lsp_types::CodeActionOrCommand;
    use lsp_types::Diagnostic;
    use lsp_types::Position;
    use lsp_types::Range;
    use lsp_types::Url;
    use serde_json::json;

    use crate::code_action::get_code_actions_from_diagnostics;

    #[test]
    fn test_get_code_actions_from_diagnostics() {
        let diagnostic = Diagnostic {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            message: "Error Message".to_string(),
            data: Some(json!(vec!["item1", "item2"])),
            ..Default::default()
        };
        let url = Url::parse("file://relay.js").unwrap();
        let code_actions = get_code_actions_from_diagnostics(&url, diagnostic);

        assert_eq!(
            code_actions
                .unwrap()
                .iter()
                .map(|item| {
                    match item {
                        CodeActionOrCommand::CodeAction(action) => action.title.clone(),
                        _ => panic!("unexpected case"),
                    }
                })
                .collect::<Vec<String>>(),
            vec![
                "Fix Error: 'item1'".to_string(),
                "Fix Error: 'item2'".to_string(),
            ]
        );
    }
}
