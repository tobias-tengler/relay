/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::Arc;

use common::DirectiveName;
use common::Span;
use graphql_ir::FragmentDefinitionName;
use graphql_syntax::ExecutableDocument;
use graphql_syntax::SchemaDocument;
use intern::string_key::StringKey;
use resolution_path::ConstantArgumentParent;
use resolution_path::ConstantArgumentPath;
use resolution_path::ConstantEnumPath;
use resolution_path::ConstantValuePath;
use resolution_path::ConstantValueRoot;
use resolution_path::IdentParent;
use resolution_path::IdentPath;
use resolution_path::LinkedFieldPath;
use resolution_path::ResolutionPath;
use resolution_path::ResolvePosition;
use resolution_path::ScalarFieldPath;
use resolution_path::SelectionParent;
use resolution_path::TypeConditionPath;
use schema::SDLSchema;
use schema::Schema;

use super::DefinitionDescription;
use crate::lsp_runtime_error::LSPRuntimeError;
use crate::lsp_runtime_error::LSPRuntimeResult;

pub fn get_schema_definition_description(
    document: &SchemaDocument,
    position_span: Span,
    schema: &Arc<SDLSchema>,
) -> LSPRuntimeResult<DefinitionDescription> {
    let node_path = document.resolve((), position_span);

    match node_path {
        ResolutionPath::Ident(IdentPath {
            inner: union_type_member_name,
            parent:
                IdentParent::UnionTypeDefinitionMemberName(_)
                | IdentParent::UnionTypeExtensionMemberName(_),
        }) => Ok(DefinitionDescription::Type {
            type_name: union_type_member_name.value,
        }),
        ResolutionPath::Ident(IdentPath {
            inner: implemented_interface_name,
            parent:
                IdentParent::ObjectTypeDefinitionImplementedInterfaceName(_)
                | IdentParent::ObjectTypeExtensionImplementedInterfaceName(_)
                | IdentParent::InterfaceTypeDefinitionImplementedInterfaceName(_)
                | IdentParent::InterfaceTypeExtensionImplementedInterfaceName(_),
        }) => Ok(DefinitionDescription::Type {
            type_name: implemented_interface_name.value,
        }),
        ResolutionPath::Ident(IdentPath {
            inner: field_type_name,
            parent: IdentParent::NamedTypeAnnotation(_),
        }) => Ok(DefinitionDescription::Type {
            type_name: field_type_name.value,
        }),
        ResolutionPath::Ident(IdentPath {
            inner: operation_type_definition_type_name,
            parent: IdentParent::OperationTypeDefinitionType(_),
        }) => Ok(DefinitionDescription::Type {
            type_name: operation_type_definition_type_name.value,
        }),
        ResolutionPath::Ident(IdentPath {
            inner: type_extension_name,
            parent:
                IdentParent::ObjectTypeExtensionName(_)
                | IdentParent::InterfaceTypeExtensionName(_)
                | IdentParent::UnionTypeExtensionName(_)
                | IdentParent::InputObjectTypeExtensionName(_)
                | IdentParent::EnumTypeExtensionName(_)
                | IdentParent::ScalarTypeExtensionName(_),
        }) => Ok(DefinitionDescription::Type {
            type_name: type_extension_name.value,
        }),
        ResolutionPath::Ident(IdentPath {
            inner: directive_name,
            parent: IdentParent::ConstantDirectiveName(_),
        }) => Ok(DefinitionDescription::Directive {
            directive_name: directive_name.value,
        }),
        ResolutionPath::ConstantEnum(ConstantEnumPath {
            inner: enum_value,
            parent: ConstantValuePath { inner: _, parent },
        }) => {
            let constant_value_root = parent.find_constant_value_root();

            match constant_value_root {
                ConstantValueRoot::InputValueDefinition(input_value_definition_path) => {
                    let enum_name = input_value_definition_path.inner.type_.inner().name.value;

                    Ok(DefinitionDescription::EnumValue {
                        enum_name,
                        enum_value: enum_value.value,
                    })
                }
                ConstantValueRoot::ConstantArgument(ConstantArgumentPath {
                    inner: constant_argument,
                    parent: ConstantArgumentParent::ConstantDirective(constant_directive_path),
                }) => {
                    let directive_argument_name = constant_argument.name.value;
                    let directive_name = constant_directive_path.inner.name.value;

                    schema
                        .get_directive(DirectiveName(directive_name))
                        .and_then(|directive| {
                            directive
                                .arguments
                                .iter()
                                .find(|arg| arg.name.0 == directive_argument_name)
                        })
                        .and_then(|arg| arg.type_.inner().get_enum_id())
                        .and_then(|enum_id| {
                            let enum_name = schema.enum_(enum_id).name.item.0;

                            Some(DefinitionDescription::EnumValue {
                                enum_name,
                                enum_value: enum_value.value,
                            })
                        })
                        .ok_or(LSPRuntimeError::ExpectedError)
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        _ => Err(LSPRuntimeError::ExpectedError),
    }
}

pub fn get_graphql_definition_description(
    document: ExecutableDocument,
    position_span: Span,
    schema: &Arc<SDLSchema>,
) -> LSPRuntimeResult<DefinitionDescription> {
    let node_path = document.resolve((), position_span);

    match node_path {
        ResolutionPath::Ident(IdentPath {
            inner: fragment_name,
            parent: IdentParent::FragmentSpreadName(_),
        }) => Ok(DefinitionDescription::Fragment {
            fragment_name: FragmentDefinitionName(fragment_name.value),
        }),
        ResolutionPath::Ident(IdentPath {
            inner: field_name,
            parent:
                IdentParent::LinkedFieldName(LinkedFieldPath {
                    inner: _,
                    parent: selection_path,
                }),
        }) => resolve_field(field_name.value, selection_path.parent, schema),
        ResolutionPath::Ident(IdentPath {
            inner: field_name,
            parent:
                IdentParent::ScalarFieldName(ScalarFieldPath {
                    inner: _,
                    parent: selection_path,
                }),
        }) => resolve_field(field_name.value, selection_path.parent, schema),
        ResolutionPath::Ident(IdentPath {
            inner: _,
            parent:
                IdentParent::TypeConditionType(TypeConditionPath {
                    inner: type_condition,
                    parent: _,
                }),
        }) => Ok(DefinitionDescription::Type {
            type_name: type_condition.type_.value,
        }),
        ResolutionPath::Ident(IdentPath {
            inner: directive_name,
            parent: IdentParent::DirectiveName(_),
        }) => Ok(DefinitionDescription::Directive {
            directive_name: directive_name.value,
        }),
        ResolutionPath::ConstantEnum(ConstantEnumPath {
            inner: enum_value,
            parent: ConstantValuePath { inner: _, parent },
        }) => {
            let constant_value_root = parent.find_constant_value_root();

            match constant_value_root {
                ConstantValueRoot::VariableDefinition(variable_definition_path) => {
                    let enum_name = variable_definition_path.inner.type_.inner().name.value;

                    Ok(DefinitionDescription::EnumValue {
                        enum_name,
                        enum_value: enum_value.value,
                    })
                }
                _ => Err(LSPRuntimeError::ExpectedError),
            }
        }
        _ => Err(LSPRuntimeError::ExpectedError),
    }
}

fn resolve_field(
    field_name: StringKey,
    selection_parent: SelectionParent<'_>,
    schema: &Arc<SDLSchema>,
) -> LSPRuntimeResult<DefinitionDescription> {
    let parent_type = selection_parent
        .find_parent_type(schema)
        .ok_or(LSPRuntimeError::ExpectedError)?;

    Ok(DefinitionDescription::Field {
        parent_type,
        field_name,
    })
}
