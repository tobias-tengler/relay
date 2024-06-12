/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(clippy::all)]

mod errors;
mod find_resolver_imports;

use std::fmt;
use std::path::Path;
use std::str::FromStr;

use ::errors::try_all;
use ::intern::intern;
use ::intern::string_key::Intern;
use ::intern::string_key::StringKey;
use ::intern::Lookup;
use common::Diagnostic;
use common::DiagnosticsResult;
use common::Location;
use common::SourceLocationKey;
use common::Span;
use common::WithLocation;
use docblock_shared::ResolverSourceHash;
use docblock_syntax::parse_docblock;
use docblock_syntax::DocblockAST;
use docblock_syntax::DocblockSection;
use errors::SchemaGenerationError;
use find_resolver_imports::ImportExportVisitor;
use find_resolver_imports::Imports;
use graphql_ir::FragmentDefinitionName;
use graphql_syntax::ExecutableDefinition;
use graphql_syntax::FieldDefinition;
use graphql_syntax::Identifier;
use graphql_syntax::InputValueDefinition;
use graphql_syntax::List;
use graphql_syntax::ListTypeAnnotation;
use graphql_syntax::NamedTypeAnnotation;
use graphql_syntax::NonNullTypeAnnotation;
use graphql_syntax::StringNode;
use graphql_syntax::Token;
use graphql_syntax::TokenKind;
use graphql_syntax::TypeAnnotation;
use hermes_comments::find_nodes_after_comments;
use hermes_estree::Declaration;
use hermes_estree::FlowTypeAnnotation;
use hermes_estree::Function;
use hermes_estree::Introspection;
use hermes_estree::Node;
use hermes_estree::ObjectTypePropertyKey;
use hermes_estree::ObjectTypePropertyType;
use hermes_estree::Pattern;
use hermes_estree::Range;
use hermes_estree::SourceRange;
use hermes_estree::TypeAlias;
use hermes_estree::TypeAnnotationEnum;
use hermes_parser::parse;
use hermes_parser::ParseResult;
use hermes_parser::ParserDialect;
use hermes_parser::ParserFlags;
use relay_docblock::DocblockIr;
use relay_docblock::ResolverTypeDocblockIr;
use relay_docblock::StrongObjectIr;
use relay_docblock::TerseRelayResolverIr;
use relay_docblock::UnpopulatedIrField;
use relay_docblock::WeakObjectIr;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use schema_extractor::SchemaExtractor;

pub static LIVE_FLOW_TYPE_NAME: &str = "LiveState";

/**
 * Reprensents a subset of supported Flow type definitions
 */
#[derive(Debug)]
pub enum ResolverFlowData {
    Strong(FieldData), // strong object or field on an object
    Weak(WeakObjectData),
}

#[derive(Debug)]
pub struct FieldData {
    pub field_name: WithLocation<StringKey>,
    pub return_type: FlowTypeAnnotation,
    pub entity_type: FlowTypeAnnotation,
    pub arguments: Option<FlowTypeAnnotation>,
    pub is_live: Option<Location>,
}

#[derive(Debug)]
pub struct WeakObjectData {
    pub field_name: WithLocation<StringKey>,
    pub type_alias: FlowTypeAnnotation,
}

pub struct RelayResolverExtractor {
    /// Cross module states
    type_definitions: FxHashMap<ModuleResolutionKey, DocblockIr>,
    unresolved_field_definitions: Vec<(ModuleResolutionKey, UnresolvedFieldDefinition)>,
    resolved_field_definitions: Vec<TerseRelayResolverIr>,

    // Needs to keep track of source location because hermes_parser currently
    // does not embed the information
    current_location: SourceLocationKey,
}

#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Debug,
    Hash,
    Copy,
    serde::Serialize
)]
pub enum JSImportType {
    Default,
    Namespace,
    Named(StringKey),
}
impl fmt::Display for JSImportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSImportType::Default => write!(f, "default"),
            JSImportType::Namespace => write!(f, "namespace"),
            JSImportType::Named(_) => write!(f, "named"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct ModuleResolutionKey {
    module_name: StringKey,
    import_type: JSImportType,
}

struct UnresolvedFieldDefinition {
    entity_name: WithLocation<StringKey>,
    field_name: WithLocation<StringKey>,
    return_type: FlowTypeAnnotation,
    arguments: Option<FlowTypeAnnotation>,
    source_hash: ResolverSourceHash,
    is_live: Option<Location>,
    description: Option<WithLocation<StringKey>>,
}

impl Default for RelayResolverExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl RelayResolverExtractor {
    pub fn new() -> Self {
        Self {
            type_definitions: Default::default(),
            unresolved_field_definitions: Default::default(),
            resolved_field_definitions: vec![],
            current_location: SourceLocationKey::generated(),
        }
    }

    /// First pass to extract all object definitions and field definitions
    pub fn parse_document(
        &mut self,
        text: &str,
        source_module_path: &str,
        fragment_definitions: &Option<&Vec<ExecutableDefinition>>,
    ) -> DiagnosticsResult<()> {
        // Assume the caller knows the text contains at least one RelayResolver decorator

        self.current_location = SourceLocationKey::standalone(source_module_path);
        let source_hash = ResolverSourceHash::new(text);
        let ParseResult { ast, comments } = parse(
            text,
            "", // Not used in hermes_parser
            ParserFlags {
                strict_mode: true,
                enable_jsx: true,
                dialect: ParserDialect::Flow,
                store_doc_block: false,
                store_comments: true,
            },
        )
        .map_err(|errs| {
            errs.into_iter()
                .map(|err| {
                    let source_span = err.span();
                    Diagnostic::error(
                        err.into_message(),
                        Location::new(
                            self.current_location,
                            Span::new(
                                source_span.offset().try_into().unwrap(),
                                (source_span.offset() + source_span.len())
                                    .try_into()
                                    .unwrap(),
                            ),
                        ),
                    )
                })
                .collect::<Vec<_>>()
        })?;

        let import_export_visitor = ImportExportVisitor::new(self.current_location);
        let (imports, exports) = import_export_visitor.get_all(&ast)?;

        let attached_comments = find_nodes_after_comments(&ast, &comments);

        try_all(
            attached_comments
                .into_iter()
                .filter(|(comment, _, _, _)| comment.contains("@RelayResolver"))
                .map(|(comment, comment_range, node, range)| {
                    // TODO: Handle unwraps
                    let docblock = parse_docblock(comment, self.current_location)?;
                    let resolver_value = docblock.find_field(intern!("RelayResolver")).unwrap();
                    let description = get_description(&docblock, comment_range)?;

                    match self.extract_graphql_types(&node, range)? {
                        ResolverFlowData::Strong(FieldData {
                            field_name,
                            return_type,
                            entity_type,
                            arguments,
                            is_live,
                        }) => {
                            let name = resolver_value.field_value.unwrap_or(field_name);

                            // Heuristic to treat lowercase name as field definition, otherwise object definition
                            // if there is a `.` in the name, it is the old resolver synatx, e.g. @RelayResolver Client.field,
                            // we should treat it as a field definition
                            let is_field_definition = {
                                let name_str = name.item.lookup();
                                let is_lowercase_initial =
                                    name_str.chars().next().unwrap().is_lowercase();
                                is_lowercase_initial || name_str.contains('.')
                            };
                            if is_field_definition {
                                let entity_name = self.extract_entity_name(entity_type)?;
                                self.add_field_definition(
                                    &imports,
                                    &exports,
                                    source_module_path,
                                    fragment_definitions,
                                    UnresolvedFieldDefinition {
                                        entity_name,
                                        field_name: name,
                                        return_type,
                                        arguments,
                                        source_hash,
                                        is_live,
                                        description,
                                    },
                                )?
                            } else {
                                self.add_type_definition(
                                    &imports,
                                    &exports,
                                    source_module_path,
                                    name,
                                    return_type,
                                    source_hash,
                                    is_live,
                                    description,
                                )?
                            }
                        }
                        ResolverFlowData::Weak(WeakObjectData {
                            field_name,
                            type_alias,
                        }) => {
                            let name = resolver_value.field_value.unwrap_or(field_name);
                            self.add_weak_type_definition(
                                name,
                                type_alias,
                                source_hash,
                                source_module_path,
                                description,
                            )?
                        }
                    }
                    Ok(())
                }),
        )?;
        Ok(())
    }

    /// Second pass to resolve all field definitions
    pub fn resolve(mut self) -> DiagnosticsResult<(Vec<DocblockIr>, Vec<TerseRelayResolverIr>)> {
        try_all(
            self.unresolved_field_definitions
                .into_iter()
                .map(|(key, field)| {
                    if let Some(DocblockIr::Type(ResolverTypeDocblockIr::StrongObjectResolver(
                        object,
                    ))) = self.type_definitions.get(&key)
                    {
                        let arguments = if let Some(args) = field.arguments {
                            Some(flow_type_to_field_arguments(self.current_location, &args)?)
                        } else {
                            None
                        };
                        let description_node = field.description.map(|desc| StringNode {
                            token: Token {
                                span: desc.location.span(),
                                kind: TokenKind::Empty,
                            },
                            value: desc.item,
                        });
                        let field_definition = FieldDefinition {
                            name: string_key_to_identifier(field.field_name),
                            type_: return_type_to_type_annotation(
                                self.current_location,
                                &field.return_type,
                            )?,
                            arguments,
                            directives: vec![],
                            description: description_node,
                            hack_source: None,
                            span: field.field_name.location.span(),
                        };
                        let live = field
                            .is_live
                            .map(|loc| UnpopulatedIrField { key_location: loc });
                        self.resolved_field_definitions.push(TerseRelayResolverIr {
                            field: field_definition,
                            type_: object
                                .type_name
                                .name_with_location(SourceLocationKey::Generated),
                            root_fragment: None,
                            location: field.field_name.location,
                            deprecated: None,
                            live,
                            fragment_arguments: None,
                            source_hash: field.source_hash,
                            semantic_non_null: None,
                        });
                        Ok(())
                    } else {
                        Err(vec![Diagnostic::error(
                            SchemaGenerationError::ModuleNotFound {
                                entity_name: field.entity_name.item,
                                export_type: key.import_type,
                                module_name: key.module_name,
                            },
                            field.entity_name.location,
                        )])
                    }
                }),
        )?;
        Ok((
            self.type_definitions.into_values().collect(),
            self.resolved_field_definitions,
        ))
    }

    fn add_field_definition(
        &mut self,
        imports: &Imports,
        exports: &FxHashSet<StringKey>,
        source_module_path: &str,
        fragment_definitions: &Option<&Vec<ExecutableDefinition>>,
        field_definition: UnresolvedFieldDefinition,
    ) -> DiagnosticsResult<()> {
        let name = field_definition.entity_name.item;
        let key = if let Some(res) = imports.get(&name) {
            res.0.clone()
        } else if exports.contains(&name) {
            let haste_module_name = Path::new(source_module_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            ModuleResolutionKey {
                module_name: haste_module_name.intern(),
                import_type: JSImportType::Named(name),
            }
        } else {
            return Err(vec![Diagnostic::error(
                SchemaGenerationError::ExpectedFlowDefinitionForType { name },
                field_definition.entity_name.location,
            )]);
        };

        if key.module_name.lookup().ends_with(".graphql") && name.lookup().ends_with("$key") {
            self.add_fragment_field_definition(fragment_definitions, field_definition)?
        } else {
            self.unresolved_field_definitions
                .push((key, field_definition));
        }
        Ok(())
    }

    fn add_fragment_field_definition(
        &mut self,
        fragment_definitions: &Option<&Vec<ExecutableDefinition>>,
        field: UnresolvedFieldDefinition,
    ) -> DiagnosticsResult<()> {
        let arguments = if let Some(args) = field.arguments {
            Some(flow_type_to_field_arguments(self.current_location, &args)?)
        } else {
            None
        };
        let description_node = field.description.map(|desc| StringNode {
            token: Token {
                span: desc.location.span(),
                kind: TokenKind::Empty,
            },
            value: desc.item,
        });
        let field_definition = FieldDefinition {
            name: string_key_to_identifier(field.field_name),
            type_: return_type_to_type_annotation(self.current_location, &field.return_type)?,
            arguments,
            directives: vec![],
            description: description_node,
            hack_source: None,
            span: field.field_name.location.span(),
        };
        let fragment_name = field
            .entity_name
            .item
            .lookup()
            .strip_suffix("$key")
            .unwrap();
        let fragment_definition_result = relay_docblock::assert_fragment_definition(
            field.entity_name,
            fragment_name.intern(),
            *fragment_definitions,
        );
        let fragment_definition = fragment_definition_result.map_err(|err| vec![err])?;

        let fragment_type_condition = WithLocation::from_span(
            fragment_definition.location.source_location(),
            fragment_definition.type_condition.span,
            fragment_definition.type_condition.type_.value,
        );
        let live = field
            .is_live
            .map(|loc| UnpopulatedIrField { key_location: loc });
        self.resolved_field_definitions.push(TerseRelayResolverIr {
            field: field_definition,
            type_: fragment_type_condition,
            root_fragment: Some(field.entity_name.map(FragmentDefinitionName)), // this includes the $key
            location: field.field_name.location,
            deprecated: None,
            live,
            fragment_arguments: None, // We don't support arguments right now
            source_hash: field.source_hash,
            semantic_non_null: None,
        });
        Ok(())
    }

    fn add_type_definition(
        &mut self,
        imports: &Imports,
        exports: &FxHashSet<StringKey>,
        source_module_path: &str,
        name: WithLocation<StringKey>,
        mut return_type: FlowTypeAnnotation,
        source_hash: ResolverSourceHash,
        is_live: Option<Location>,
        description: Option<WithLocation<StringKey>>,
    ) -> DiagnosticsResult<()> {
        let strong_object = StrongObjectIr {
            type_name: string_key_to_identifier(name),
            rhs_location: name.location,
            root_fragment: WithLocation::new(
                name.location,
                FragmentDefinitionName(format!("{}__id", name.item).intern()),
            ),
            description,
            deprecated: None,
            live: is_live.map(|loc| UnpopulatedIrField { key_location: loc }),
            location: name.location,
            implements_interfaces: vec![],
            source_hash,
            semantic_non_null: None,
        };

        // We ignore nullable annotation since both nullable and non-nullable types are okay for
        // defining a strong object
        return_type = if let FlowTypeAnnotation::NullableTypeAnnotation(return_type) = return_type {
            return_type.type_annotation
        } else {
            return_type
        };
        // For now, we assume the flow type for the strong object is always imported
        // from a separate file
        match return_type {
            FlowTypeAnnotation::GenericTypeAnnotation(generic_type) => {
                let name = schema_extractor::get_identifier_for_flow_generic(WithLocation {
                    item: generic_type.as_ref(),
                    location: self.to_location(generic_type.as_ref()),
                })?;
                if generic_type.type_parameters.is_some() {
                    return Err(vec![Diagnostic::error(
                        SchemaGenerationError::GenericNotSupported,
                        name.location,
                    )]);
                }

                let key = if let Some((key, import_location)) = imports.get(&name.item) {
                    if let JSImportType::Namespace = key.import_type {
                        return Err(vec![
                            Diagnostic::error(
                                SchemaGenerationError::UseNamedOrDefaultImport,
                                name.location,
                            )
                            .annotate(format!("{} is imported from", name.item), *import_location),
                        ]);
                    };
                    key.clone()
                } else if exports.contains(&name.item) {
                    let haste_module_name = Path::new(source_module_path)
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    ModuleResolutionKey {
                        module_name: haste_module_name.intern(),
                        import_type: JSImportType::Named(name.item),
                    }
                } else {
                    return Err(vec![Diagnostic::error(
                        SchemaGenerationError::ExpectedFlowDefinitionForType { name: name.item },
                        name.location,
                    )]);
                };

                self.type_definitions.insert(
                    key.clone(),
                    DocblockIr::Type(ResolverTypeDocblockIr::StrongObjectResolver(strong_object)),
                );

                Ok(())
            }
            FlowTypeAnnotation::ObjectTypeAnnotation(object_type) => Err(vec![Diagnostic::error(
                SchemaGenerationError::ObjectNotSupported,
                self.to_location(object_type.as_ref()),
            )]),
            _ => self.error_result(
                SchemaGenerationError::UnsupportedType {
                    name: return_type.name(),
                },
                &return_type,
            ),
        }
    }

    fn add_weak_type_definition(
        &mut self,
        name: WithLocation<StringKey>,
        type_alias: FlowTypeAnnotation,
        source_hash: ResolverSourceHash,
        source_module_path: &str,
        description: Option<WithLocation<StringKey>>,
    ) -> DiagnosticsResult<()> {
        let weak_object = WeakObjectIr {
            type_name: string_key_to_identifier(name),
            rhs_location: name.location,
            description,
            hack_source: None,
            deprecated: None,
            location: name.location,
            implements_interfaces: vec![],
            source_hash,
        };
        let haste_module_name = Path::new(source_module_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let key = ModuleResolutionKey {
            module_name: haste_module_name.intern(),
            import_type: JSImportType::Named(name.item),
        };
        // Add fields
        if let FlowTypeAnnotation::ObjectTypeAnnotation(object_node) = type_alias {
            let field_map = self.get_object_fields(&object_node)?;
            if !field_map.is_empty() {
                try_all(field_map.into_iter().map(|(field_name, field_type)| {
                    let field_definition = FieldDefinition {
                        name: string_key_to_identifier(field_name),
                        type_: return_type_to_type_annotation(self.current_location, field_type)?,
                        arguments: None,
                        directives: vec![],
                        description: None,
                        hack_source: None,
                        span: field_name.location.span(),
                    };

                    self.resolved_field_definitions.push(TerseRelayResolverIr {
                        field: field_definition,
                        type_: weak_object
                            .type_name
                            .name_with_location(SourceLocationKey::Generated),
                        root_fragment: None,
                        location: field_name.location,
                        deprecated: None,
                        live: None,
                        fragment_arguments: None,
                        source_hash,
                        semantic_non_null: None,
                    });
                    Ok(())
                }))?;

                self.type_definitions.insert(
                    key.clone(),
                    DocblockIr::Type(ResolverTypeDocblockIr::WeakObjectType(weak_object)),
                );
                Ok(())
            } else {
                let location = self.to_location(object_node.as_ref());
                Err(vec![Diagnostic::error(
                    SchemaGenerationError::ExpectedWeakObjectToHaveFields,
                    location,
                )])
            }
        } else {
            Err(vec![Diagnostic::error(
                SchemaGenerationError::ExpectedTypeAliasToBeObject,
                self.to_location(&type_alias),
            )])
        }
    }

    pub fn extract_function(&self, node: &Function) -> DiagnosticsResult<ResolverFlowData> {
        let ident = node.id.as_ref().ok_or_else(|| {
            Diagnostic::error(
                SchemaGenerationError::MissingFunctionName,
                self.to_location(node),
            )
        })?;
        let field_name = WithLocation {
            item: (&ident.name).intern(),
            location: self.to_location(ident),
        };

        let return_type_annotation = node.return_type.as_ref().ok_or_else(|| {
            Diagnostic::error(
                SchemaGenerationError::MissingReturnType,
                self.to_location(node),
            )
        })?;
        let flow_return_type = self.unwrap_annotation_enum(return_type_annotation)?;
        let (return_type_with_live, is_optional) =
            schema_extractor::unwrap_nullable_type(flow_return_type);

        // unwrap is_live from the return type
        let (return_type, is_live) = match return_type_with_live {
            FlowTypeAnnotation::GenericTypeAnnotation(type_node) => {
                let name = schema_extractor::get_identifier_for_flow_generic(WithLocation {
                    item: type_node,
                    location: self.to_location(type_node.as_ref()),
                })?;
                if let Some(type_param) = &type_node.type_parameters {
                    match type_param.params.as_slice() {
                        [param] => {
                            if name.item.lookup() == LIVE_FLOW_TYPE_NAME {
                                if is_optional {
                                    return Err(vec![Diagnostic::error(
                                        SchemaGenerationError::NoOptionalLiveType,
                                        name.location,
                                    )]);
                                }
                                (param, Some(name.location))
                            } else {
                                (flow_return_type, None)
                            }
                        }
                        _ => {
                            // Does not support multiple type params for now
                            return self.error_result(
                                SchemaGenerationError::UnsupportedType {
                                    name: "Multiple type params",
                                },
                                type_node.as_ref(),
                            );
                        }
                    }
                } else {
                    (flow_return_type, None)
                }
            }
            _ => (flow_return_type, None),
        };

        if node.params.is_empty() {
            return self.error_result(SchemaGenerationError::MissingFunctionParam, node);
        }
        let param = &node.params[0];
        let entity_type = if let Pattern::Identifier(identifier) = param {
            let type_annotation = identifier.type_annotation.as_ref().ok_or_else(|| {
                Diagnostic::error(
                    SchemaGenerationError::MissingParamType,
                    self.to_location(param),
                )
            })?;
            if let TypeAnnotationEnum::FlowTypeAnnotation(type_) = &type_annotation.type_annotation
            {
                type_
            } else {
                return self.error_result(
                    SchemaGenerationError::UnsupportedType { name: param.name() },
                    param,
                );
            }
        } else {
            return self.error_result(
                SchemaGenerationError::UnsupportedType { name: param.name() },
                param,
            );
        };

        let arguments = if node.params.len() > 1 {
            let arg_param = &node.params[1];
            let args = if let Pattern::Identifier(identifier) = arg_param {
                let type_annotation = identifier.type_annotation.as_ref().ok_or_else(|| {
                    Diagnostic::error(
                        SchemaGenerationError::MissingParamType,
                        self.to_location(param),
                    )
                })?;
                if let TypeAnnotationEnum::FlowTypeAnnotation(type_) =
                    &type_annotation.type_annotation
                {
                    Some(type_)
                } else {
                    None
                }
            } else {
                None
            };
            if args.is_none() {
                return self.error_result(
                    SchemaGenerationError::IncorrectArgumentsDefinition,
                    arg_param,
                );
            }
            args
        } else {
            None
        };

        Ok(ResolverFlowData::Strong(FieldData {
            field_name,
            return_type: return_type.clone(),
            entity_type: entity_type.clone(),
            arguments: arguments.cloned(),
            is_live,
        }))
    }

    fn extract_type_alias(&self, node: &TypeAlias) -> DiagnosticsResult<WeakObjectData> {
        let field_name = WithLocation {
            item: (&node.id.name).intern(),
            location: self.to_location(&node.id),
        };
        Ok(WeakObjectData {
            field_name,
            type_alias: node.right.clone(),
        })
    }

    fn extract_graphql_types(
        &self,
        node: &Node<'_>,
        range: SourceRange,
    ) -> DiagnosticsResult<ResolverFlowData> {
        if let Node::ExportNamedDeclaration(node) = node {
            match node.declaration {
                Some(Declaration::FunctionDeclaration(ref node)) => {
                    self.extract_function(&node.function)
                }
                Some(Declaration::TypeAlias(ref node)) => {
                    let data = self.extract_type_alias(node)?;
                    Ok(ResolverFlowData::Weak(data))
                }
                _ => Err(vec![Diagnostic::error(
                    SchemaGenerationError::ExpectedFunctionOrTypeAlias,
                    Location::new(
                        self.current_location,
                        Span::new(range.start, range.end.into()),
                    ),
                )]),
            }
        } else {
            Err(vec![Diagnostic::error(
                SchemaGenerationError::ExpectedNamedExport,
                Location::new(
                    self.current_location,
                    Span::new(range.start, range.end.into()),
                ),
            )])
        }
    }

    fn extract_entity_name(
        &self,
        entity_type: FlowTypeAnnotation,
    ) -> DiagnosticsResult<WithLocation<StringKey>> {
        match entity_type {
            FlowTypeAnnotation::NumberTypeAnnotation(annot) => Ok(WithLocation {
                item: intern!("Float"),
                location: self.to_location(annot.as_ref()),
            }),
            FlowTypeAnnotation::StringTypeAnnotation(annot) => Ok(WithLocation {
                item: intern!("String"),
                location: self.to_location(annot.as_ref()),
            }),
            FlowTypeAnnotation::GenericTypeAnnotation(annot) => {
                let id = schema_extractor::get_identifier_for_flow_generic(WithLocation {
                    item: &annot,
                    location: self.to_location(annot.as_ref()),
                })?;
                if annot.type_parameters.is_some() {
                    return Err(vec![Diagnostic::error(
                        SchemaGenerationError::GenericNotSupported,
                        self.to_location(annot.as_ref()),
                    )]);
                }
                Ok(id)
            }
            FlowTypeAnnotation::NullableTypeAnnotation(annot) => Err(vec![Diagnostic::error(
                SchemaGenerationError::UnexpectedNullableStrongType,
                self.to_location(annot.as_ref()),
            )]),
            _ => Err(vec![Diagnostic::error(
                SchemaGenerationError::UnsupportedType {
                    name: entity_type.name(),
                },
                self.to_location(&entity_type),
            )]),
        }
    }
}

impl SchemaExtractor for RelayResolverExtractor {
    fn to_location<T: Range>(&self, node: &T) -> Location {
        to_location(self.current_location, node)
    }
}

fn to_location<T: Range>(source_location: SourceLocationKey, node: &T) -> Location {
    let range = node.range();
    Location::new(source_location, Span::new(range.start, range.end.into()))
}

fn string_key_to_identifier(name: WithLocation<StringKey>) -> Identifier {
    Identifier {
        span: name.location.span(),
        token: Token {
            span: name.location.span(),
            kind: TokenKind::Identifier,
        },
        value: name.item,
    }
}

fn return_type_to_type_annotation(
    source_location: SourceLocationKey,
    return_type: &FlowTypeAnnotation,
) -> DiagnosticsResult<TypeAnnotation> {
    let (return_type, mut is_optional) = schema_extractor::unwrap_nullable_type(return_type);
    let location = to_location(source_location, return_type);
    let type_annotation = match return_type {
        FlowTypeAnnotation::GenericTypeAnnotation(node) => {
            let identifier = schema_extractor::get_identifier_for_flow_generic(WithLocation {
                item: node,
                location: to_location(source_location, node.as_ref()),
            })?;
            match &node.type_parameters {
                None => TypeAnnotation::Named(NamedTypeAnnotation {
                    name: string_key_to_identifier(identifier),
                }),
                Some(type_parameters) if type_parameters.params.len() == 1 => {
                    let identifier_name = identifier.item.lookup();
                    match identifier_name {
                        "Array" | "$ReadOnlyArray" => {
                            let param = &type_parameters.params[0];
                            TypeAnnotation::List(Box::new(ListTypeAnnotation {
                                span: location.span(),
                                open: generated_token(),
                                type_: return_type_to_type_annotation(source_location, param)?,
                                close: generated_token(),
                            }))
                        }
                        "IdOf" => {
                            let param = &type_parameters.params[0];
                            let location = to_location(source_location, param);
                            if let FlowTypeAnnotation::StringLiteralTypeAnnotation(node) = param {
                                TypeAnnotation::Named(NamedTypeAnnotation {
                                    name: Identifier {
                                        span: location.span(),
                                        token: Token {
                                            span: location.span(),
                                            kind: TokenKind::Identifier,
                                        },
                                        value: (&node.value).intern(),
                                    },
                                })
                            } else {
                                return Err(vec![Diagnostic::error(
                                    SchemaGenerationError::TODO,
                                    location,
                                )]);
                            }
                        }
                        "RelayResolverValue" => {
                            // Special case for `RelayResolverValue`, it is always optional
                            is_optional = true;
                            TypeAnnotation::Named(NamedTypeAnnotation {
                                name: Identifier {
                                    span: location.span(),
                                    token: Token {
                                        span: location.span(),
                                        kind: TokenKind::Identifier,
                                    },
                                    value: intern!("RelayResolverValue"),
                                },
                            })
                        }
                        _ => {
                            return Err(vec![Diagnostic::error(
                                SchemaGenerationError::UnSupportedGeneric {
                                    name: identifier.item,
                                },
                                location,
                            )]);
                        }
                    }
                }
                _ => {
                    return Err(vec![Diagnostic::error(
                        SchemaGenerationError::TODO,
                        location,
                    )]);
                }
            }
        }
        FlowTypeAnnotation::StringTypeAnnotation(node) => {
            let identifier = WithLocation {
                item: intern!("String"),
                location: to_location(source_location, node.as_ref()),
            };
            TypeAnnotation::Named(NamedTypeAnnotation {
                name: string_key_to_identifier(identifier),
            })
        }
        FlowTypeAnnotation::NumberTypeAnnotation(node) => {
            let identifier = WithLocation {
                item: intern!("Float"),
                location: to_location(source_location, node.as_ref()),
            };
            TypeAnnotation::Named(NamedTypeAnnotation {
                name: string_key_to_identifier(identifier),
            })
        }
        FlowTypeAnnotation::BooleanTypeAnnotation(node) => {
            let identifier = WithLocation {
                item: intern!("Boolean"),
                location: to_location(source_location, node.as_ref()),
            };
            TypeAnnotation::Named(NamedTypeAnnotation {
                name: string_key_to_identifier(identifier),
            })
        }
        FlowTypeAnnotation::BooleanLiteralTypeAnnotation(node) => {
            let identifier = WithLocation {
                item: intern!("Boolean"),
                location: to_location(source_location, node.as_ref()),
            };
            TypeAnnotation::Named(NamedTypeAnnotation {
                name: string_key_to_identifier(identifier),
            })
        }
        _ => {
            return Err(vec![Diagnostic::error(
                SchemaGenerationError::UnsupportedType {
                    name: return_type.name(),
                },
                location,
            )]);
        }
    };

    if !is_optional {
        let non_null_annotation = TypeAnnotation::NonNull(Box::new(NonNullTypeAnnotation {
            span: location.span(),
            type_: type_annotation,
            exclamation: generated_token(),
        }));
        Ok(non_null_annotation)
    } else {
        Ok(type_annotation)
    }
}

fn flow_type_to_field_arguments(
    source_location: SourceLocationKey,
    args_type: &FlowTypeAnnotation,
) -> DiagnosticsResult<List<InputValueDefinition>> {
    let obj = if let FlowTypeAnnotation::ObjectTypeAnnotation(type_) = &args_type {
        // unwrap the ref then the box, then re-add the ref
        type_
    } else {
        return Err(vec![Diagnostic::error(
            SchemaGenerationError::IncorrectArgumentsDefinition,
            to_location(source_location, args_type),
        )]);
    };
    let mut items = vec![];
    for prop_type in obj.properties.iter() {
        let prop_span = to_location(source_location, prop_type).span();
        if let ObjectTypePropertyType::ObjectTypeProperty(prop) = prop_type {
            let ident = if let ObjectTypePropertyKey::Identifier(ident) = &prop.key {
                ident
            } else {
                return Err(vec![Diagnostic::error(
                    SchemaGenerationError::IncorrectArgumentsDefinition,
                    to_location(source_location, &prop.key),
                )]);
            };

            let ident_node: &hermes_estree::Identifier = ident;
            let name_span = to_location(source_location, ident_node).span();
            let arg = InputValueDefinition {
                name: graphql_syntax::Identifier {
                    span: name_span,
                    token: Token {
                        span: name_span,
                        kind: TokenKind::Identifier,
                    },
                    value: StringKey::from_str(&ident.name).map_err(|_| {
                        vec![Diagnostic::error(
                            SchemaGenerationError::IncorrectArgumentsDefinition,
                            to_location(source_location, args_type),
                        )]
                    })?,
                },
                type_: return_type_to_type_annotation(source_location, &prop.value)?,
                default_value: None,
                directives: vec![],
                span: prop_span,
            };
            items.push(arg);
        }
    }

    let list_start: u32 = args_type.range().start;
    let list_end: u32 = args_type.range().end;
    Ok(List {
        items,
        span: to_location(source_location, args_type).span(),
        start: Token {
            span: Span {
                start: list_start,
                end: list_start + 1,
            },
            kind: TokenKind::OpenBrace,
        },
        end: Token {
            span: Span {
                start: list_end - 1,
                end: list_end,
            },
            kind: TokenKind::CloseBrace,
        },
    })
}

fn get_description(
    docblock: &DocblockAST,
    range: SourceRange,
) -> DiagnosticsResult<Option<WithLocation<StringKey>>> {
    let mut description = None;
    for section in docblock.sections.iter() {
        match section {
            DocblockSection::Field(_) => (),
            DocblockSection::FreeText(text) => {
                let location = Location::new(
                    text.location.source_location(),
                    Span::new(range.start, range.end),
                );
                if description.is_none() {
                    description = Some(WithLocation {
                        location,
                        item: text.item,
                    })
                } else {
                    return Err(vec![Diagnostic::error(
                        SchemaGenerationError::MultipleDocblockDescriptions,
                        location,
                    )]);
                }
            }
        }
    }
    Ok(description)
}

fn generated_token() -> Token {
    Token {
        span: Span::empty(),
        kind: TokenKind::Empty,
    }
}
