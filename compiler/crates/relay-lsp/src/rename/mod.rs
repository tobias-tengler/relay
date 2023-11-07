/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Utilities for providing the rename feature

use std::collections::HashMap;
use std::path::PathBuf;

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
use intern::string_key::Intern;
use intern::string_key::StringKey;
use lsp_types::request::PrepareRenameRequest;
use lsp_types::request::Rename;
use lsp_types::request::Request;
use lsp_types::request::WillRenameFiles;
use lsp_types::PrepareRenameResponse;
use lsp_types::TextEdit;
use lsp_types::Url;
use lsp_types::WorkspaceEdit;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use relay_docblock::DocblockIr;
use relay_docblock::On;
use relay_transforms::extract_module_name;
use resolution_path::ArgumentParent;
use resolution_path::ArgumentPath;
use resolution_path::DirectiveParent;
use resolution_path::DirectivePath;
use resolution_path::IdentParent;
use resolution_path::IdentPath;
use resolution_path::ResolutionPath;
use resolution_path::ResolvePosition;
use resolution_path::VariableIdentifierParent;
use resolution_path::VariableIdentifierPath;

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
use lazy_static::lazy_static;

lazy_static! {
    static ref ARGUMENTS_DIRECTIVE: StringKey = "arguments".intern();
    static ref ARGUMENTDEFINITIONS_DIRECTIVE: StringKey = "argumentDefinitions".intern();
}

/// Resolve a [`Rename`] request to workspace edits
pub fn on_rename(
    state: &impl GlobalState,
    params: <Rename as Request>::Params,
) -> LSPRuntimeResult<<Rename as Request>::Result> {
    let uri = &params.text_document_position.text_document.uri;
    let (feature, location) = state.extract_feature_from_text(&params.text_document_position, 1)?;

    let program = &state.get_program(&state.extract_project_name_from_url(&uri)?)?;
    let root_dir = &state.root_dir();

    let rename_request = create_rename_request(feature, location)?;
    let changes = process_rename_request(rename_request, params.new_name, program, root_dir)?;

    Ok(Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    }))
}

/// Resolve a [`PrepareRenameRequest`] to a [`PrepareRenameResponse`]
pub fn on_prepare_rename(
    state: &impl GlobalState,
    params: <PrepareRenameRequest as Request>::Params,
) -> LSPRuntimeResult<<PrepareRenameRequest as Request>::Result> {
    let root_dir = &state.root_dir();
    let (feature, location) = state.extract_feature_from_text(&params, 1)?;

    let rename_request = create_rename_request(feature, location)?;

    // TODO: Remove the condition, once https://github.com/facebook/relay/issues/4447 is resolved
    match rename_request.kind {
        RenameKind::ResolverField { .. } => Err(LSPRuntimeError::UnexpectedError(String::from(
            "Relay Resolvers can not yet be reliably renamed",
        ))),
        _ => {
            let lsp_location =
                transform_relay_location_to_lsp_location(root_dir, rename_request.location)?;

            Ok(Some(PrepareRenameResponse::Range(lsp_location.range)))
        }
    }
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

        let old_prefix = get_module_name(&old_path)?;
        let new_prefix = get_module_name(&new_path)?;

        if old_prefix == new_prefix {
            continue;
        }

        let full_text = get_file_contents(&old_path)?;

        let embedded_sources = extract_graphql::extract(&full_text);
        if embedded_sources.is_empty() {
            continue;
        }

        let program = &state.get_program(&state.extract_project_name_from_url(&old_file_uri)?)?;
        let root_dir = &state.root_dir();

        for (index, embedded_source) in embedded_sources.iter().enumerate() {
            match embedded_source {
                JavaScriptSourceFeature::GraphQL(graphql_source) => {
                    let text_source = graphql_source.text_source();
                    let old_source_location =
                        SourceLocationKey::embedded(old_path.to_str().unwrap(), index);
                    let new_source_location =
                        SourceLocationKey::embedded(new_path.to_str().unwrap(), index);
                    let document = parse_executable_with_error_recovery(
                        &text_source.text,
                        new_source_location,
                    )
                    .item;

                    for definition in &document.definitions {
                        let (old_name, rename_request) = match definition {
                            ExecutableDefinition::Fragment(
                                graphql_syntax::FragmentDefinition {
                                    name: fragment_name,
                                    ..
                                },
                            ) => {
                                let old_frag_name = fragment_name.value;

                                Some((
                                    old_frag_name.to_string(),
                                    RenameRequest::new(
                                        RenameKind::FragmentDefinitionOrSpread {
                                            fragment_name: old_frag_name,
                                        },
                                        IRLocation::new(old_source_location, fragment_name.span),
                                    ),
                                ))
                            }
                            ExecutableDefinition::Operation(OperationDefinition {
                                name: Some(operation_name),
                                ..
                            }) => Some((
                                operation_name.value.to_string(),
                                RenameRequest::new(
                                    RenameKind::OperationDefinition,
                                    IRLocation::new(old_source_location, operation_name.span),
                                ),
                            )),
                            ExecutableDefinition::Operation(
                                graphql_syntax::OperationDefinition { name: None, .. },
                            ) => None,
                        }
                        .ok_or(LSPRuntimeError::ExpectedError)?;

                        let new_name = replace_prefix(&old_name, &old_prefix, &new_prefix);
                        let changes =
                            process_rename_request(rename_request, new_name, program, root_dir)?;

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

#[derive(Debug, Clone)]
pub enum RenameKind {
    OperationDefinition,
    FragmentDefinitionOrSpread {
        fragment_name: StringKey,
    },
    ResolverField {
        field_name: StringKey,
        parent_type: StringKey,
    },
    VariableDefinition {
        variable_name: StringKey,
    },
    FragmentArgumentDefinition {
        fragment_name: StringKey,
        argument_name: StringKey,
    },
    VariableOrFragmentArgumentUsage {
        variable_name: StringKey,
    },
}

#[derive(Debug)]
pub struct RenameRequest {
    /// The type of the rename request we're responding to
    kind: RenameKind,
    location: IRLocation,
}

impl RenameRequest {
    fn new(kind: RenameKind, location: IRLocation) -> Self {
        Self { kind, location }
    }
}

fn create_rename_request(
    feature: Feature,
    location: IRLocation,
) -> LSPRuntimeResult<RenameRequest> {
    match feature {
        Feature::GraphQLDocument(document) => {
            let node_path = document.resolve((), location.span());

            match node_path {
                ResolutionPath::VariableIdentifier(VariableIdentifierPath {
                    inner: variable,
                    parent,
                    ..
                }) => {
                    let span_without_dollar = Span::new(variable.span.start + 1, variable.span.end);
                    let location = IRLocation::new(location.source_location(), span_without_dollar);

                    let kind = match parent {
                        VariableIdentifierParent::VariableDefinition(_) => {
                            RenameKind::VariableDefinition {
                                variable_name: variable.name,
                            }
                        }
                        VariableIdentifierParent::Value(_) => {
                            RenameKind::VariableOrFragmentArgumentUsage {
                                variable_name: variable.name,
                            }
                        }
                    };

                    Ok(RenameRequest::new(kind, location))
                }
                ResolutionPath::Ident(IdentPath {
                    inner: argument_name,
                    parent:
                        IdentParent::ArgumentName(ArgumentPath {
                            parent:
                                ArgumentParent::Directive(DirectivePath {
                                    inner: directive,
                                    parent,
                                    ..
                                }),
                            ..
                        }),
                }) => {
                    let fragment_name = match parent {
                        DirectiveParent::FragmentDefinition(fragment) => {
                            if directive.name.value != *ARGUMENTDEFINITIONS_DIRECTIVE {
                                return Err(LSPRuntimeError::ExpectedError);
                            }

                            Some(fragment.inner.name.value)
                        }
                        DirectiveParent::FragmentSpread(fragment_spread) => {
                            if directive.name.value != *ARGUMENTS_DIRECTIVE {
                                return Err(LSPRuntimeError::ExpectedError);
                            }

                            Some(fragment_spread.inner.name.value)
                        }
                        _ => None,
                    }
                    .ok_or(LSPRuntimeError::ExpectedError)?;

                    let location = IRLocation::new(location.source_location(), argument_name.span);
                    let kind = RenameKind::FragmentArgumentDefinition {
                        fragment_name,
                        argument_name: argument_name.value,
                    };

                    Ok(RenameRequest::new(kind, location))
                }
                ResolutionPath::Ident(IdentPath {
                    inner: fragment_name,
                    parent:
                        IdentParent::FragmentSpreadName(_) | IdentParent::FragmentDefinitionName(_),
                }) => {
                    let location = IRLocation::new(location.source_location(), fragment_name.span);
                    let kind = RenameKind::FragmentDefinitionOrSpread {
                        fragment_name: fragment_name.value,
                    };

                    Ok(RenameRequest::new(kind, location))
                }
                ResolutionPath::Ident(IdentPath {
                    inner: operation_name,
                    parent: IdentParent::OperationDefinitionName(_),
                }) => {
                    let location = IRLocation::new(location.source_location(), operation_name.span);

                    Ok(RenameRequest::new(
                        RenameKind::OperationDefinition,
                        location,
                    ))
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        Feature::DocblockIr(docblock) => {
            let resolution_info = create_docblock_resolution_info(&docblock, location.span());

            match resolution_info {
                Some(DocblockResolutionInfo::FieldName(docblock_field)) => {
                    let location = IRLocation::new(location.source_location(), docblock_field.span);
                    let kind = RenameKind::ResolverField {
                        field_name: docblock_field.value,
                        parent_type: extract_parent_type(docblock),
                    };

                    Ok(RenameRequest::new(kind, location))
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
    }
}

fn process_rename_request(
    rename_request: RenameRequest,
    new_name: String,
    program: &Program,
    root_dir: &PathBuf,
) -> LSPRuntimeResult<HashMap<Url, Vec<TextEdit>>> {
    match rename_request.kind {
        RenameKind::VariableDefinition { variable_name } => Ok(rename_variable()),
        RenameKind::FragmentArgumentDefinition {
            fragment_name,
            argument_name,
        } => Ok(rename_fragment_argument(
            fragment_name,
            argument_name,
            &new_name,
            program,
            root_dir,
        )),
        RenameKind::VariableOrFragmentArgumentUsage { variable_name } => {
            let definition_rename_request = get_rename_request_for_definition();

            match definition_rename_request {
                Ok(rename_request) => {
                    process_rename_request(rename_request, new_name, program, root_dir)
                }
                Err(_) => {
                    // We couldn't find a definition to rename,
                    // so we'll simply rename the current usage.
                    let lsp_location = transform_relay_location_to_lsp_location(
                        root_dir,
                        rename_request.location,
                    )?;

                    Ok(HashMap::from([(
                        lsp_location.uri,
                        vec![TextEdit {
                            new_text: new_name,
                            range: lsp_location.range,
                        }],
                    )]))
                }
            }
        }
        RenameKind::OperationDefinition => {
            rename_operation(new_name, rename_request.location, root_dir)
        }
        RenameKind::FragmentDefinitionOrSpread { fragment_name } => {
            Ok(rename_fragment(fragment_name, new_name, program, root_dir))
        }
        RenameKind::ResolverField {
            parent_type,
            field_name,
        } => Ok(rename_resolver_field(
            field_name,
            parent_type,
            &new_name,
            rename_request.location,
            program,
            root_dir,
        )),
    }
}

fn rename_variable() -> HashMap<Url, Vec<TextEdit>> {
    HashMap::new()
}

fn rename_fragment_argument(
    fragment_name: StringKey,
    argument_name: StringKey,
    new_argument_name: &str,
    program: &Program,
    root_dir: &PathBuf,
) -> HashMap<Url, Vec<TextEdit>> {
    let locations =
        FragmentArgumentFinder::get_argument_locations(program, fragment_name, argument_name);

    map_locations_to_text_edits(locations, new_argument_name.to_owned(), root_dir)
}

fn rename_resolver_field(
    field_name: StringKey,
    type_name: StringKey,
    new_field_name: &str,
    field_definition_location: IRLocation,
    program: &Program,
    root_dir: &PathBuf,
) -> HashMap<Url, Vec<TextEdit>> {
    let mut locations =
        find_field_locations(program, field_name, type_name).unwrap_or_else(|| vec![]);
    locations.push(field_definition_location);

    map_locations_to_text_edits(locations, new_field_name.to_owned(), root_dir)
}

fn rename_operation(
    new_operation_name: String,
    location: IRLocation,
    root_dir: &PathBuf,
) -> LSPRuntimeResult<HashMap<Url, Vec<TextEdit>>> {
    let lsp_location = transform_relay_location_to_lsp_location(root_dir, location)?;

    Ok(HashMap::from([(
        lsp_location.uri,
        vec![TextEdit {
            new_text: new_operation_name,
            range: lsp_location.range,
        }],
    )]))
}

fn rename_fragment(
    fragment_name: StringKey,
    new_fragment_name: String,
    program: &Program,
    root_dir: &PathBuf,
) -> HashMap<Url, Vec<TextEdit>> {
    let locations = FragmentFinder::get_fragment_usages(program, fragment_name);

    map_locations_to_text_edits(locations, new_fragment_name, root_dir)
}

fn get_rename_request_for_definition() -> LSPRuntimeResult<RenameRequest> {
    // TODO: Implement

    Err(LSPRuntimeError::ExpectedError)
}

fn map_locations_to_text_edits(
    locations: Vec<IRLocation>,
    new_text: String,
    root_dir: &PathBuf,
) -> HashMap<Url, Vec<TextEdit>> {
    let vec_res: Vec<(Url, TextEdit)> = locations
        .par_iter()
        .flat_map(|location| {
            let transformed = transform_relay_location_to_lsp_location(root_dir, *location);
            transformed.ok().map(|lsp_location| {
                let text_edit = TextEdit {
                    range: lsp_location.range,
                    new_text: new_text.to_owned(),
                };
                (lsp_location.uri, text_edit)
            })
        })
        .collect();

    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
    for (uri, text_edit) in vec_res {
        changes.entry(uri).or_default().push(text_edit);
    }

    changes
}

fn extract_parent_type(docblock: DocblockIr) -> StringKey {
    match docblock {
        DocblockIr::LegacyVerboseResolver(resolver_ir) => match resolver_ir.on {
            On::Type(on_type) => on_type.value.item,
            On::Interface(on_interface) => on_interface.value.item,
        },
        DocblockIr::TerseRelayResolver(resolver_ir) => resolver_ir.type_.item,
        DocblockIr::StrongObjectResolver(strong_object) => strong_object.type_name.value,
        DocblockIr::WeakObjectType(weak_type_ir) => weak_type_ir.type_name.value,
    }
}

// TODO: Move all this to a vistors helper module
#[derive(Debug, Clone)]
struct FragmentFinder {
    fragment_locations: Vec<IRLocation>,
    fragment_name: StringKey,
}

impl FragmentFinder {
    pub fn get_fragment_usages(program: &Program, name: StringKey) -> Vec<IRLocation> {
        let mut finder = FragmentFinder {
            fragment_locations: vec![],
            fragment_name: name,
        };
        finder.visit_program(program);
        finder.fragment_locations
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

struct FragmentArgumentFinderScope {
    // name of the enclosing Fragment
    fragment_name: Option<StringKey>,
}

struct FragmentArgumentFinder {
    argument_locations: Vec<IRLocation>,
    fragment_name: StringKey,
    argument_name: StringKey,
    current_scope: FragmentArgumentFinderScope,
}

impl FragmentArgumentFinder {
    pub fn get_argument_locations(
        program: &Program,
        fragment_name: StringKey,
        argument_name: StringKey,
    ) -> Vec<IRLocation> {
        let mut finder = FragmentArgumentFinder {
            argument_locations: vec![],
            fragment_name,
            argument_name,
            current_scope: FragmentArgumentFinderScope {
                fragment_name: None,
            },
        };

        finder.visit_program(program);
        finder.argument_locations
    }
}

impl Visitor for FragmentArgumentFinder {
    const NAME: &'static str = "FragmentArgumentFinder";
    const VISIT_ARGUMENTS: bool = true;
    const VISIT_DIRECTIVES: bool = true;

    fn visit_fragment_spread(&mut self, spread: &FragmentSpread) {
        if spread.fragment.item.0 == self.fragment_name {
            spread
                .arguments
                .iter()
                .filter(|a| a.name.item.0 == self.argument_name)
                .for_each(|a| {
                    self.argument_locations.push(a.name.location);
                })
        }
    }

    fn visit_argument(&mut self, argument: &graphql_ir::Argument) {
        if let Some(fragment_name) = self.current_scope.fragment_name {
            if fragment_name == self.fragment_name {
                match &argument.value.item {
                    graphql_ir::Value::Variable(variable) => {
                        if variable.name.item.0 == self.argument_name {
                            let name_location = variable.name.location;
                            let location_without_dollar = name_location.with_span(Span {
                                start: name_location.span().start + 1,
                                end: name_location.span().end,
                            });

                            self.argument_locations.push(location_without_dollar);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn visit_fragment(&mut self, fragment: &FragmentDefinition) {
        assert!(self.current_scope.fragment_name.is_none());
        self.current_scope.fragment_name = Some(fragment.name.item.0);

        if fragment.name.item.0 == self.fragment_name {
            fragment
                .variable_definitions
                .iter()
                .filter(|v| v.name.item.0 == self.argument_name)
                .for_each(|v| {
                    self.argument_locations.push(v.name.location);
                });
        }

        self.default_visit_fragment(fragment);

        self.current_scope.fragment_name = None;
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

fn replace_prefix(s: &str, old_prefix: &str, new_prefix: &str) -> String {
    if let Some(rest) = s.strip_prefix(old_prefix) {
        let mut result = String::from(new_prefix);
        result.push_str(rest);
        return result;
    }

    s.to_string()
}

fn get_module_name(path: &PathBuf) -> LSPRuntimeResult<String> {
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(LSPRuntimeError::ExpectedError)?;

    extract_module_name(file_name).ok_or(LSPRuntimeError::ExpectedError)
}
