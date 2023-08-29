/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Utilities for providing the rename feature

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use common::Location as IRLocation;
use common::SourceLocationKey;
use common::Span;
use extract_graphql::JavaScriptSourceFeature;
use graphql_ir::FragmentDefinitionName;
use graphql_ir::FragmentSpread;
use graphql_ir::Program;
use graphql_ir::Visitor;
use graphql_syntax::parse_executable_with_error_recovery;
use intern::string_key::StringKey;
use lsp_types::request::PrepareRenameRequest;
use lsp_types::request::Rename;
use lsp_types::request::Request;
use lsp_types::request::WillRenameFiles;
use lsp_types::Location;
use lsp_types::PrepareRenameResponse;
use lsp_types::Range;
use lsp_types::TextEdit;
use lsp_types::Url;
use lsp_types::WorkspaceEdit;
use relay_docblock::DocblockIr;
use relay_docblock::On;
use resolution_path::IdentParent;
use resolution_path::IdentPath;
use resolution_path::ResolutionPath;
use resolution_path::ResolvePosition;

use crate::docblock_resolution_info::create_docblock_resolution_info;
use crate::docblock_resolution_info::DocblockResolutionInfo;
use crate::find_field_usages::find_field_locations;
use crate::location::get_file_contents;
use crate::location::transform_relay_location_to_lsp_location;
use crate::utils::is_file_uri_in_dir;
use crate::GlobalState;
use crate::LSPRuntimeError;
use crate::LSPRuntimeResult;

/// Resolve a RenameRequest to a RenameResponse
pub fn on_rename(
    state: &impl GlobalState,
    params: <Rename as Request>::Params,
) -> LSPRuntimeResult<<Rename as Request>::Result> {
    let uri = &params.text_document_position.text_document.uri.clone();
    let text_document_position_params = lsp_types::TextDocumentPositionParams {
        text_document: params.text_document_position.text_document,
        position: params.text_document_position.position,
    };
    let (feature, position_span, source_location_key) =
        state.extract_feature_from_text(&text_document_position_params, 1)?;

    let program = &state.get_program(&state.extract_project_name_from_url(&uri)?)?;
    let root_dir = &state.root_dir();

    let changes = match feature {
        crate::Feature::GraphQLDocument(document) => {
            let node_path = document.resolve((), position_span);

            match node_path {
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_spread_name,
                    parent: IdentParent::FragmentSpreadName(_),
                }) => Ok(rename_fragment(
                    fragment_spread_name.value,
                    params.new_name,
                    program,
                    root_dir,
                )),
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_name,
                    parent: IdentParent::FragmentDefinitionName(_),
                }) => Ok(rename_fragment(
                    fragment_name.value,
                    params.new_name,
                    program,
                    root_dir,
                )),
                ResolutionPath::Ident(IdentPath {
                    inner: operation_name,
                    parent: IdentParent::OperationDefinitionName(_),
                }) => {
                    let location = common::Location::new(source_location_key, operation_name.span);

                    let lsp_location =
                        transform_relay_location_to_lsp_location(root_dir, location).unwrap();

                    Ok(rename_operation(params.new_name, lsp_location))
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        crate::Feature::DocblockIr(docblock) => {
            let resolution_info =
                create_docblock_resolution_info(&docblock, position_span).unwrap();

            match resolution_info {
                DocblockResolutionInfo::FieldName(docblock_field) => {
                    let parent_type = extract_parent_type(docblock);

                    let mut changes = rename_relay_resolver_field(
                        docblock_field.value,
                        parent_type,
                        &params.new_name,
                        program,
                        root_dir,
                    );

                    let location = common::Location::new(source_location_key, docblock_field.span);
                    let lsp_location =
                        transform_relay_location_to_lsp_location(root_dir, location)?;

                    merge_text_edit(&mut changes, lsp_location, &params.new_name);

                    // todo: rename JS function

                    Ok(changes)
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
    }?;

    return Ok(Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    }));
}

/// Resolve a PrepareRenameRequest to a PrepareRenameResponse
pub fn on_prepare_rename(
    state: &impl GlobalState,
    params: <PrepareRenameRequest as Request>::Params,
) -> LSPRuntimeResult<<PrepareRenameRequest as Request>::Result> {
    let text_document_position_params = lsp_types::TextDocumentPositionParams {
        text_document: params.text_document,
        position: params.position,
    };
    let (feature, position_span, source_location_key) =
        state.extract_feature_from_text(&text_document_position_params, 1)?;
    let root_dir = &state.root_dir();

    let range = match feature {
        crate::Feature::GraphQLDocument(document) => {
            let node_path = document.resolve((), position_span);

            match node_path {
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_spread_name,
                    parent: IdentParent::FragmentSpreadName(_),
                }) => span_to_range(&root_dir, source_location_key, fragment_spread_name.span),
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_name,
                    parent: IdentParent::FragmentDefinitionName(_),
                }) => span_to_range(&root_dir, source_location_key, fragment_name.span),
                ResolutionPath::Ident(IdentPath {
                    inner: operation_name,
                    parent: IdentParent::OperationDefinitionName(_),
                }) => span_to_range(&root_dir, source_location_key, operation_name.span),
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        crate::Feature::DocblockIr(docblock) => {
            let resolution_info =
                create_docblock_resolution_info(&docblock, position_span).unwrap();

            match resolution_info {
                DocblockResolutionInfo::FieldName(docblock_field) => {
                    span_to_range(root_dir, source_location_key, docblock_field.span)
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
    }?;

    Ok(Some(PrepareRenameResponse::Range(range)))
}

// todo: how can I get rid of all the unwraps below and make it more functional?

/// Resolve a WillRenameFilesRequest to a WillRenameFilesResponse
pub fn on_will_rename_files(
    state: &impl GlobalState,
    params: <WillRenameFiles as Request>::Params,
) -> LSPRuntimeResult<<WillRenameFiles as Request>::Result> {
    let mut rename_changes = HashMap::new();

    for file_rename in &params.files {
        let old_file_uri = Url::parse(&file_rename.old_uri).unwrap();
        let new_file_uri = Url::parse(&file_rename.new_uri).unwrap();

        if !is_file_uri_in_dir(state.root_dir(), &new_file_uri) {
            continue;
        }

        let old_path = old_file_uri.to_file_path().unwrap();
        let new_path = new_file_uri.to_file_path().unwrap();
        let old_file_name = old_path.file_stem().unwrap().to_str().unwrap();
        let new_file_name = new_path.file_stem().unwrap().to_str().unwrap();

        if old_file_name == new_file_name {
            continue;
        }

        let full_text = get_file_contents(&old_path).unwrap();

        let embedded_sources = extract_graphql::extract(&full_text);
        if embedded_sources.is_empty() {
            continue;
        }

        let program = &state.get_program(&state.extract_project_name_from_url(&old_file_uri)?)?;
        let root_dir = &state.root_dir();

        let mut index = 0;
        for embedded_source in &embedded_sources {
            // todo: do the rusty way
            let _ = match embedded_source {
                JavaScriptSourceFeature::GraphQL(graphql_source) => {
                    let source_location_key =
                        SourceLocationKey::embedded(new_file_uri.as_ref(), index);

                    let text_source = graphql_source.text_source();
                    let document = parse_executable_with_error_recovery(
                        &text_source.text,
                        source_location_key,
                    )
                    .item;

                    for definition in &document.definitions {
                        let changes = match definition {
                            graphql_syntax::ExecutableDefinition::Fragment(frag_def) => {
                                let frag_name = frag_def.name.value;
                                let old_frag_name = frag_name.to_string();
                                let new_frag_name =
                                    old_frag_name.replace(old_file_name, new_file_name);

                                rename_fragment(frag_name, new_frag_name, program, root_dir)
                            }
                            graphql_syntax::ExecutableDefinition::Operation(op_def) => {
                                let operation_name_identifier = op_def.name.unwrap();
                                let old_operation_name = operation_name_identifier.to_string();
                                let new_operation_name =
                                    old_operation_name.replace(old_file_name, new_file_name);

                                let name_range =
                                    text_source.to_span_range(operation_name_identifier.span);

                                let location = Location::new(old_file_uri.clone(), name_range);

                                rename_operation(new_operation_name, location)
                            }
                        };

                        merge_text_changes(&mut rename_changes, changes);
                    }

                    Ok(())
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            };

            index += 1;
        }
    }

    Ok(Some(WorkspaceEdit {
        changes: Some(rename_changes),
        ..Default::default()
    }))
}

fn rename_relay_resolver_field(
    field_name: StringKey,
    type_name: StringKey,
    new_field_name: &String,
    program: &Program,
    root_dir: &PathBuf,
) -> HashMap<Url, Vec<TextEdit>> {
    find_field_locations(program, field_name, type_name)
        .unwrap()
        .into_iter()
        .fold(HashMap::new(), |mut map, location| {
            let lsp_location =
                transform_relay_location_to_lsp_location(root_dir, location).unwrap();

            merge_text_edit(&mut map, lsp_location, &new_field_name);

            map
        })
}

fn rename_operation(new_operation_name: String, location: Location) -> HashMap<Url, Vec<TextEdit>> {
    HashMap::from([(
        location.uri,
        vec![TextEdit {
            new_text: new_operation_name,
            range: location.range,
        }],
    )])
}

fn rename_fragment(
    fragment_name: StringKey,
    new_fragment_name: String,
    program: &Program,
    root_dir: &PathBuf,
) -> HashMap<Url, Vec<TextEdit>> {
    FragmentFinder::get_fragment_usages(program, fragment_name)
        .into_iter()
        .fold(HashMap::new(), |mut map, location| {
            let lsp_location =
                transform_relay_location_to_lsp_location(root_dir, location).unwrap();

            merge_text_edit(&mut map, lsp_location, &new_fragment_name);

            map
        })
}

fn extract_parent_type(docblock: DocblockIr) -> StringKey {
    match docblock {
        DocblockIr::RelayResolver(resolver_ir) => match resolver_ir.on {
            On::Type(on_type) => on_type.value.item,
            On::Interface(on_interface) => on_interface.value.item,
        },
        DocblockIr::TerseRelayResolver(resolver_ir) => resolver_ir.type_.item,
        DocblockIr::StrongObjectResolver(strong_object) => strong_object.type_name.value,
        DocblockIr::WeakObjectType(weak_type_ir) => weak_type_ir.type_name.value,
    }
}

#[derive(Debug, Clone)]
pub struct FragmentFinder {
    fragment_locations: Vec<IRLocation>,
    fragment_name: StringKey,
}

impl FragmentFinder {
    pub fn get_fragment_usages(program: &Program, name: StringKey) -> Vec<IRLocation> {
        let mut fragment_finder = FragmentFinder {
            fragment_locations: vec![],
            fragment_name: name,
        };
        fragment_finder.visit_program(program);
        fragment_finder.fragment_locations
    }
}

impl Visitor for FragmentFinder {
    const NAME: &'static str = "FragmentFinder";
    const VISIT_ARGUMENTS: bool = false;
    const VISIT_DIRECTIVES: bool = false;

    fn visit_fragment_spread(&mut self, spread: &FragmentSpread) {
        if spread.fragment.item == FragmentDefinitionName(self.fragment_name) {
            self.fragment_locations.push(spread.fragment.location);
        }
    }

    fn visit_fragment(&mut self, fragment: &graphql_ir::FragmentDefinition) {
        if fragment.name.item == FragmentDefinitionName(self.fragment_name) {
            self.fragment_locations.push(fragment.name.location)
        }

        self.default_visit_fragment(fragment)
    }
}

fn merge_text_changes(
    source: &mut HashMap<Url, Vec<TextEdit>>,
    target: HashMap<Url, Vec<TextEdit>>,
) {
    for (uri, changes) in target {
        let entry = source.entry(uri);

        let existing_changes = entry.or_default();
        for new_change in changes {
            existing_changes.push(new_change);
        }
    }
}

fn merge_text_edit(source: &mut HashMap<Url, Vec<TextEdit>>, location: Location, change: &String) {
    let entry = source.entry(location.uri);

    let existing_changes = entry.or_default();

    existing_changes.push(TextEdit {
        range: location.range,
        new_text: change.to_owned(),
    });
}

fn span_to_range(
    root_dir: &Path,
    source_location_key: SourceLocationKey,
    span: Span,
) -> LSPRuntimeResult<Range> {
    let location = common::Location::new(source_location_key, span);

    let lsp_location = transform_relay_location_to_lsp_location(root_dir, location)?;

    Ok(lsp_location.range)
}
