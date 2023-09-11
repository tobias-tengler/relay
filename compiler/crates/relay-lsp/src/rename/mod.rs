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
use std::sync::Arc;

use common::Location as IRLocation;
use common::SourceLocationKey;
use common::Span;
use extract_graphql::JavaScriptSourceFeature;
use graphql_ir::FragmentDefinition;
use graphql_ir::FragmentSpread;
use graphql_ir::Program;
use graphql_ir::Visitor;
use graphql_syntax::parse_executable_with_error_recovery;
use graphql_syntax::ExecutableDefinition;
use graphql_syntax::OperationDefinition;
use intern::string_key::StringKey;
use lsp_types::request::PrepareRenameRequest;
use lsp_types::request::Rename;
use lsp_types::request::Request;
use lsp_types::request::WillRenameFiles;
use lsp_types::Location as LspLocation;
use lsp_types::PrepareRenameResponse;
use lsp_types::Range;
use lsp_types::TextEdit;
use lsp_types::Url;
use lsp_types::WorkspaceEdit;
use relay_docblock::DocblockIr;
use relay_docblock::On;
use relay_transforms::extract_module_name;
use resolution_path::IdentParent;
use resolution_path::IdentPath;
use resolution_path::ResolutionPath;
use resolution_path::ResolvePosition;
use schema::SDLSchema;

use crate::docblock_resolution_info::create_docblock_resolution_info;
use crate::docblock_resolution_info::DocblockResolutionInfo;
use crate::find_field_usages::find_field_locations;
use crate::location::get_file_contents;
use crate::location::transform_relay_location_to_lsp_location;
use crate::utils::is_file_uri_in_dir;
use crate::Feature;
use crate::GlobalState;
use crate::LSPRuntimeError;
use crate::LSPRuntimeResult;

/// Resolve a [`Rename`] request to workspace edits
pub fn on_rename(
    state: &impl GlobalState,
    params: <Rename as Request>::Params,
) -> LSPRuntimeResult<<Rename as Request>::Result> {
    let uri = &params.text_document_position.text_document.uri;
    let (feature, location) = state.extract_feature_from_text(&params.text_document_position, 1)?;

    let project_name = &state.extract_project_name_from_url(uri)?;
    let program = &state.get_program(project_name)?;
    let schema = &state.get_schema(project_name)?;
    let root_dir = &state.root_dir();

    let changes = match feature {
        Feature::GraphQLDocument(document) => {
            let node_path = document.resolve((), location.span());

            match node_path {
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_spread_name,
                    parent:
                        IdentParent::FragmentSpreadName(_) | IdentParent::FragmentDefinitionName(_),
                }) => rename_fragment(
                    fragment_spread_name.value,
                    params.new_name,
                    program,
                    root_dir,
                ),
                ResolutionPath::Ident(IdentPath {
                    inner: operation_name,
                    parent: IdentParent::OperationDefinitionName(_),
                }) => {
                    let location = IRLocation::new(location.source_location(), operation_name.span);
                    let lsp_location =
                        transform_relay_location_to_lsp_location(root_dir, location)?;

                    Ok(rename_operation(params.new_name, lsp_location))
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        Feature::DocblockIr(docblock) => {
            let resolution_info = create_docblock_resolution_info(&docblock, location.span());

            match resolution_info {
                Some(DocblockResolutionInfo::FieldName(docblock_field)) => {
                    let parent_type = extract_parent_type(docblock);

                    let mut changes = rename_relay_resolver_field(
                        docblock_field.value,
                        parent_type,
                        &params.new_name,
                        program,
                        schema,
                        root_dir,
                    )?;

                    let location =
                        common::Location::new(location.source_location(), docblock_field.span);
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

/// Resolve a [`PrepareRenameRequest`] to a [`PrepareRenameResponse`]
pub fn on_prepare_rename(
    state: &impl GlobalState,
    params: <PrepareRenameRequest as Request>::Params,
) -> LSPRuntimeResult<<PrepareRenameRequest as Request>::Result> {
    let (feature, location) = state.extract_feature_from_text(&params, 1)?;
    let root_dir = &state.root_dir();

    let range = match feature {
        Feature::GraphQLDocument(document) => {
            let node_path = document.resolve((), location.span());

            match node_path {
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_spread_name,
                    parent:
                        IdentParent::FragmentSpreadName(_)
                        | IdentParent::FragmentDefinitionName(_)
                        | IdentParent::OperationDefinitionName(_),
                }) => span_to_range(
                    &root_dir,
                    location.source_location(),
                    fragment_spread_name.span,
                ),
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        Feature::DocblockIr(docblock) => {
            let resolution_info = create_docblock_resolution_info(&docblock, location.span());

            match resolution_info {
                Some(DocblockResolutionInfo::FieldName(docblock_field)) => {
                    span_to_range(root_dir, location.source_location(), docblock_field.span)
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
    }?;

    Ok(Some(PrepareRenameResponse::Range(range)))
}

/// Resolve a [`WillRenameFiles`] request to workspace edits
pub fn on_will_rename_files(
    state: &impl GlobalState,
    params: <WillRenameFiles as Request>::Params,
) -> LSPRuntimeResult<<WillRenameFiles as Request>::Result> {
    let mut rename_changes = HashMap::new();

    for file_rename in &params.files {
        let old_file_uri =
            Url::parse(&file_rename.old_uri).map_err(|_| LSPRuntimeError::ExpectedError)?;
        let new_file_uri =
            Url::parse(&file_rename.new_uri).map_err(|_| LSPRuntimeError::ExpectedError)?;

        if !is_file_uri_in_dir(state.root_dir(), &new_file_uri) {
            continue;
        }

        let old_path = old_file_uri
            .to_file_path()
            .map_err(|_| LSPRuntimeError::ExpectedError)?;
        let new_path = new_file_uri
            .to_file_path()
            .map_err(|_| LSPRuntimeError::ExpectedError)?;
        let old_file_name = old_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(LSPRuntimeError::ExpectedError)?;
        let new_file_name = new_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(LSPRuntimeError::ExpectedError)?;

        if old_file_name == new_file_name {
            continue;
        }

        let full_text = get_file_contents(&old_path)?;

        let embedded_sources = extract_graphql::extract(&full_text);
        if embedded_sources.is_empty() {
            continue;
        }

        let old_name_prefix = extract_module_name(old_file_name).unwrap();
        let new_name_prefix = extract_module_name(new_file_name).unwrap();

        let program = &state.get_program(&state.extract_project_name_from_url(&old_file_uri)?)?;
        let root_dir = &state.root_dir();

        for (index, embedded_source) in embedded_sources.iter().enumerate() {
            match embedded_source {
                JavaScriptSourceFeature::GraphQL(graphql_source) => {
                    let text_source = graphql_source.text_source();
                    let document = parse_executable_with_error_recovery(
                        &text_source.text,
                        SourceLocationKey::embedded(new_file_uri.as_ref(), index),
                    )
                    .item;

                    for definition in &document.definitions {
                        let changes = match definition {
                            ExecutableDefinition::Fragment(frag_def) => {
                                let old_frag_name = frag_def.name.value;
                                let new_frag_name = replace_prefix(
                                    &old_frag_name.to_string(),
                                    &old_name_prefix,
                                    &new_name_prefix,
                                );

                                rename_fragment(old_frag_name, new_frag_name, program, root_dir)?
                            }
                            ExecutableDefinition::Operation(OperationDefinition {
                                name: Some(operation_name_identifier),
                                ..
                            }) => {
                                let new_operation_name = replace_prefix(
                                    &operation_name_identifier.value.to_string(),
                                    &old_name_prefix,
                                    &new_name_prefix,
                                );

                                let name_range =
                                    text_source.to_span_range(operation_name_identifier.span);

                                let location = LspLocation::new(old_file_uri.clone(), name_range);

                                rename_operation(new_operation_name, location)
                            }
                            ExecutableDefinition::Operation(
                                graphql_syntax::OperationDefinition { name: None, .. },
                            ) => HashMap::new(),
                        };

                        merge_text_changes(&mut rename_changes, changes);
                    }
                }
                _ => (),
            };
        }
    }

    Ok(Some(WorkspaceEdit {
        changes: Some(rename_changes),
        ..Default::default()
    }))
}

fn replace_prefix(s: &str, old_prefix: &str, new_prefix: &str) -> String {
    if let Some(rest) = s.strip_prefix(old_prefix) {
        let mut result = String::from(new_prefix);
        result.push_str(rest);
        return result;
    }

    s.to_string()
}

fn rename_relay_resolver_field(
    field_name: StringKey,
    type_name: StringKey,
    new_field_name: &str,
    program: &Program,
    schema: &Arc<SDLSchema>,
    root_dir: &PathBuf,
) -> LSPRuntimeResult<HashMap<Url, Vec<TextEdit>>> {
    let mut changes = HashMap::<Url, Vec<TextEdit>>::new();

    for location in
        find_field_locations(program, schema, field_name, type_name).unwrap_or_else(|| vec![])
    {
        let lsp_location = transform_relay_location_to_lsp_location(root_dir, location)?;

        merge_text_edit(&mut changes, lsp_location, new_field_name);
    }

    Ok(changes)
}

fn rename_operation(
    new_operation_name: String,
    location: LspLocation,
) -> HashMap<Url, Vec<TextEdit>> {
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
) -> LSPRuntimeResult<HashMap<Url, Vec<TextEdit>>> {
    let mut changes = HashMap::<Url, Vec<TextEdit>>::new();

    for location in FragmentFinder::get_fragment_usages(program, fragment_name) {
        let lsp_location = transform_relay_location_to_lsp_location(root_dir, location)?;
        merge_text_edit(&mut changes, lsp_location, &new_fragment_name);
    }

    Ok(changes)
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
        if spread.fragment.item.0 == self.fragment_name {
            self.fragment_locations.push(spread.fragment.location);
        }
    }

    fn visit_fragment(&mut self, fragment: &FragmentDefinition) {
        if fragment.name.item.0 == self.fragment_name {
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
        source.entry(uri).or_default().extend(changes);
    }
}

fn merge_text_edit(source: &mut HashMap<Url, Vec<TextEdit>>, location: LspLocation, change: &str) {
    source.entry(location.uri).or_default().push(TextEdit {
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
