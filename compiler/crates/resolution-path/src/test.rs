/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use common::Span;
use graphql_syntax::parse_executable_with_features;
use graphql_syntax::parse_schema_document;
use graphql_syntax::FragmentArgumentSyntaxKind;
use graphql_syntax::ParserFeatures;

use super::*;

pub(super) fn test_resolution(source: &str, sub_str: &str, cb: impl Fn(&ResolutionPath<'_>)) {
    let document = parse_executable_with_features(
        source,
        SourceLocationKey::standalone("/test/file"),
        ParserFeatures {
            fragment_argument_capability:
                FragmentArgumentSyntaxKind::SpreadArgumentsAndFragmentVariableDefinitions,
        },
    )
    .unwrap();

    let pos = source.find(sub_str).unwrap() as u32;

    // Select the `uri` field
    let position_span = Span {
        start: pos,
        end: pos,
    };

    let resolved = document.resolve((), position_span);

    cb(&resolved);
}

pub(super) fn test_schema_resolution(
    source: &str,
    sub_str: &str,
    cb: impl Fn(&ResolutionPath<'_>),
) {
    let document =
        parse_schema_document(source, SourceLocationKey::standalone("/test/file")).unwrap();

    let pos = source.find(sub_str).unwrap() as u32;

    // Select the `uri` field
    let position_span = Span {
        start: pos,
        end: pos,
    };

    let resolved = document.resolve((), position_span);

    cb(&resolved);
}

#[test]
fn operation_definition_type_condition() {
    let source = r#"
            query Foo {
                me {
                    id
                }
            }
        "#;
    test_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::OperationDefinitionName(_),
            })
        );
    })
}

#[test]
fn operation_definition_operation() {
    let source = r#"
            query Foo {
                me {
                    id
                }
            }
        "#;
    test_resolution(source, "query", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Operation(OperationPath {
                inner: (_, OperationKind::Query),
                parent: _,
            })
        );
    })
}

#[test]
fn linked_field_name() {
    let source = r#"
            query Foo {
                me {
                    id
                }
            }
        "#;
    test_resolution(source, "me", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::LinkedFieldName(_),
            })
        );
    })
}

#[test]
fn linked_field_alias() {
    let source = r#"
            query Foo {
                mario: me {
                    id
                }
            }
        "#;
    test_resolution(source, "mario", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::LinkedFieldAlias(_),
            })
        );
    })
}

#[test]
fn scalar_field_name() {
    let source = r#"
            query Foo {
                me {
                    id
                }
            }
        "#;
    test_resolution(source, "id", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ScalarFieldName(_),
            })
        );
    })
}

#[test]
fn scalar_field_alias() {
    let source = r#"
            query Foo {
                me {
                    identity: id
                }
            }
        "#;
    test_resolution(source, "identity", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ScalarFieldAlias(_),
            })
        );
    })
}

#[test]
fn inline_fragment() {
    let source = r#"
            query Foo {
                me {
                    ... on User {
                        id
                    }
                }
            }
        "#;
    test_resolution(source, "...", |resolved| {
        assert_matches!(resolved, ResolutionPath::InlineFragment(_));
    })
}

#[test]
fn inline_fragment_type_condition() {
    let source = r#"
            query Foo {
                me {
                    ... on User {
                        id
                    }
                }
            }
        "#;
    test_resolution(source, "User", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::TypeConditionType(_),
            })
        );
    })
}
#[test]
fn fragment_definition_name() {
    let source = r#"
            fragment Foo on User {
                id
            }
        "#;
    test_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::FragmentDefinitionName(_),
            })
        );
    })
}

#[test]
fn fragment_definition_type() {
    let source = r#"
            fragment Foo on User {
                id
            }
        "#;
    test_resolution(source, "User", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::TypeConditionType(_),
            })
        );
    })
}

#[test]
fn fragment_spread_name() {
    let source = r#"
            fragment Foo on Node {
                ...someFragment
            }
        "#;
    test_resolution(source, "someFragment", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::FragmentSpreadName(_),
            })
        );
    })
}

#[test]
fn directive_name() {
    let source = r#"
            fragment Foo on Node {
                id @required(action: LOG)
            }
        "#;
    test_resolution(source, "required", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::DirectiveName(_),
            })
        );
    })
}

#[test]
fn argument_name() {
    let source = r#"
            fragment Foo on Node {
                id @required(action: LOG)
            }
        "#;
    test_resolution(source, "action", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ArgumentName(_),
            })
        );
    })
}

#[test]
fn argument_value() {
    let source = r#"
            fragment Foo on Node {
                id @required(action: LOG)
            }
        "#;
    test_resolution(source, "LOG", |resolved| {
        assert_matches!(resolved, ResolutionPath::ConstantEnum(_));
    })
}

#[test]
fn list_literal() {
    let source = r#"
            query Foo {
                checkinSearchQuery(query: {
                    query: "Hello",
                    inputs: [{query: "Goodbye", inputs: []}]
                })
            }
        "#;
    test_resolution(source, "Goodbye", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::ConstantString(ConstantStringPath {
                inner: _,
                parent: ConstantValuePath {
                    inner: _,
                    parent: ConstantValueParent::ConstantArgValue(_),
                },
            })
        );
    })
}

#[test]
fn fragment_argument_name() {
    let source = r#"
        fragment Foo($localId: ID!) on User {
            id
          }
        "#;
    test_resolution(source, "localId", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::VariableIdentifier(VariableIdentifierPath {
                inner: _,
                parent: VariableIdentifierParent::VariableDefinition(VariableDefinitionPath {
                    inner: _,
                    parent: VariableDefinitionListPath {
                        inner: _,
                        parent: VariableDefinitionListParent::FragmentDefinition(_),
                    },
                }),
            })
        );
    })
}

#[test]
fn fragment_argument_type() {
    let source = r#"
        fragment Foo($localId: ID!) on User {
            id
          }
        "#;
    test_resolution(source, "ID!", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(NamedTypeAnnotationPath {
                    inner: _,
                    parent: TypeAnnotationPath {
                        inner: _,
                        parent: TypeAnnotationParent::NonNullTypeAnnotation(_),
                    }
                }),
            })
        )
    })
}

#[test]
fn fragment_argument_default_value() {
    let source = r#"
        fragment Foo($localId: ID! = "1") on User {
            id
          }
        "#;
    test_resolution(source, r#""1""#, |resolved| {
        assert_matches!(resolved, ResolutionPath::ConstantString(_));
    })
}

#[test]
fn fragment_argument_directive() {
    let source = r#"
        fragment Foo($localId: ID! = "1") on User {
            id
          }
        "#;
    test_resolution(source, r#""1""#, |resolved| {
        assert_matches!(resolved, ResolutionPath::ConstantString(_));
    })
}

// ## Directives

#[test]
fn directive_definition_name() {
    let source = r#"
        directive @foo on FIELD
        "#;
    test_schema_resolution(source, "foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::DirectiveDefinitionName(_),
            })
        );
    })
}

// ## Union Types

#[test]
fn union_type_definition_name() {
    let source = r#"
        union Foo = Bar | Baz
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::UnionTypeDefinitionName(_),
            })
        );
    })
}

#[test]
fn union_type_definition_member_name() {
    let source = r#"
        union Foo = Bar | Baz
        "#;
    test_schema_resolution(source, "Bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::UnionTypeDefinitionMemberName(_),
            })
        );
    })
}

#[test]
fn union_type_extension_name() {
    let source = r#"
        extend union Foo = Bar | Baz
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::UnionTypeExtensionName(_),
            })
        );
    })
}

#[test]
fn union_type_extension_member_name() {
    let source = r#"
        extend union Foo = Bar | Baz
        "#;
    test_schema_resolution(source, "Bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::UnionTypeExtensionMemberName(_),
            })
        );
    })
}

// ## Interface Types

#[test]
fn interface_type_definition_name() {
    let source = r#"
        interface Foo {
            bar: String
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InterfaceTypeDefinitionName(_),
            })
        );
    })
}

#[test]
fn interface_type_definition_implements_interface() {
    let source = r#"
        interface Foo implements Bar {
            baz: String
        }
        "#;
    test_schema_resolution(source, "Bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InterfaceTypeDefinitionImplementedInterfaceName(_),
            })
        );
    })
}

#[test]
fn interface_type_definition_field() {
    let source = r#"
        interface Foo {
            bar: String
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::FieldDefinitionName(FieldDefinitionPath {
                    inner: _,
                    parent: FieldDefinitionParent::InterfaceTypeDefinition(_),
                }),
            })
        );
    })
}

// ## Interface Type Extensions

#[test]
fn interface_type_extension_name() {
    let source = r#"
        extend interface Foo {
            bar: String
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InterfaceTypeExtensionName(_),
            })
        );
    })
}

#[test]
fn interface_type_extension_implements_interface() {
    let source = r#"
        extend interface Foo implements Bar {
            baz: String
        }
        "#;
    test_schema_resolution(source, "Bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InterfaceTypeExtensionImplementedInterfaceName(_),
            })
        );
    })
}

#[test]
fn interface_type_extension_field() {
    let source = r#"
        extend interface Foo {
            bar: String
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::FieldDefinitionName(FieldDefinitionPath {
                    inner: _,
                    parent: FieldDefinitionParent::InterfaceTypeExtension(_),
                }),
            })
        );
    })
}

// ## Object Types

#[test]
fn object_type_definition_name() {
    let source = r#"
        type Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ObjectTypeDefinitionName(_),
            })
        );
    })
}

#[test]
fn object_type_definition_implemented_interface_name() {
    let source = r#"
        type Foo implements Node {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Node", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ObjectTypeDefinitionImplementedInterfaceName(_),
            })
        );
    })
}

#[test]
fn object_type_definition_field_name() {
    let source = r#"
        type Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::FieldDefinitionName(_),
            })
        );
    })
}

#[test]
fn object_type_definition_field_named_type() {
    let source = r#"
        type Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Baz", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(_),
            })
        );
    })
}

#[test]
fn object_type_definition_field_non_null_type() {
    let source = r#"
        type Foo {
            bar: Baz!
        }
        "#;
    test_schema_resolution(source, "Baz!", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(NamedTypeAnnotationPath {
                    inner: _,
                    parent: TypeAnnotationPath {
                        inner: _,
                        parent: TypeAnnotationParent::NonNullTypeAnnotation(_),
                    }
                }),
            })
        );
    })
}

// ## Object Type Extensions

#[test]
fn object_type_extension_name() {
    let source = r#"
        extend type Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ObjectTypeExtensionName(_),
            })
        );
    })
}

#[test]
fn object_type_extension_implemented_interface_name() {
    let source = r#"
        extend type Foo implements Node {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Node", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ObjectTypeExtensionImplementedInterfaceName(_),
            })
        );
    })
}

#[test]
fn object_type_extension_field_name() {
    let source = r#"
        extend type Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::FieldDefinitionName(_),
            })
        );
    })
}

#[test]
fn object_type_extension_field_named_type() {
    let source = r#"
        extend type Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Baz", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(_),
            })
        );
    })
}

#[test]
fn object_type_extension_field_non_null_type() {
    let source = r#"
        extend type Foo {
            bar: Baz!
        }
        "#;
    test_schema_resolution(source, "Baz!", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(NamedTypeAnnotationPath {
                    inner: _,
                    parent: TypeAnnotationPath {
                        inner: _,
                        parent: TypeAnnotationParent::NonNullTypeAnnotation(_),
                    }
                }),
            })
        );
    })
}

// ## Input Object Types

#[test]
fn input_object_type_definition_name() {
    let source = r#"
        input Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InputObjectTypeDefinitionName(_),
            })
        );
    })
}

#[test]
fn input_object_type_definition_field_name() {
    let source = r#"
        input Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InputValueDefinitionName(_),
            })
        );
    })
}

#[test]
fn input_object_type_definition_field_named_type() {
    let source = r#"
        input Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Baz", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(_),
            })
        );
    })
}

#[test]
fn input_object_type_definition_field_non_null_type() {
    let source = r#"
        input Foo {
            bar: Baz!
        }
        "#;
    test_schema_resolution(source, "Baz!", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(NamedTypeAnnotationPath {
                    inner: _,
                    parent: TypeAnnotationPath {
                        inner: _,
                        parent: TypeAnnotationParent::NonNullTypeAnnotation(_),
                    }
                }),
            })
        );
    })
}

// ## Input Object Type Extensions

#[test]
fn input_object_type_extension_name() {
    let source = r#"
        extend input Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InputObjectTypeExtensionName(_),
            })
        );
    })
}

#[test]
fn input_object_type_extension_field_name() {
    let source = r#"
        extend input Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InputValueDefinitionName(_),
            })
        );
    })
}

#[test]
fn input_object_type_extension_field_named_type() {
    let source = r#"
        extend input Foo {
            bar: Baz
        }
        "#;
    test_schema_resolution(source, "Baz", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(_),
            })
        );
    })
}

#[test]
fn input_object_type_extension_field_non_null_type() {
    let source = r#"
        extend input Foo {
            bar: Baz!
        }
        "#;
    test_schema_resolution(source, "Baz!", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(NamedTypeAnnotationPath {
                    inner: _,
                    parent: TypeAnnotationPath {
                        inner: _,
                        parent: TypeAnnotationParent::NonNullTypeAnnotation(_),
                    }
                }),
            })
        );
    })
}

// ## Input Value Definitions

#[test]
fn input_value_definition_name() {
    let source = r#"
        type Foo {
            bar(baz: Qux): Quux
        }
        "#;
    test_schema_resolution(source, "baz", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::InputValueDefinitionName(_),
            })
        );
    })
}

#[test]
fn input_value_definition_named_type() {
    let source = r#"
        type Foo {
            bar(baz: Qux): Quux
        }
        "#;
    test_schema_resolution(source, "Qux", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(_),
            })
        );
    })
}

#[test]
fn input_value_definition_non_null_type() {
    let source = r#"
        type Foo {
            bar(baz: Qux!): Quux
        }
        "#;
    test_schema_resolution(source, "Qux!", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::NamedTypeAnnotation(NamedTypeAnnotationPath {
                    inner: _,
                    parent: TypeAnnotationPath {
                        inner: _,
                        parent: TypeAnnotationParent::NonNullTypeAnnotation(_),
                    }
                }),
            })
        );
    })
}

// ## Enum Types

#[test]
fn enum_definition_name() {
    let source = r#"
        enum Foo {
            BAR
            BAZ
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::EnumTypeDefinitionName(_),
            })
        );
    })
}

#[test]
fn enum_definition_directive() {
    let source = r#"
        enum Foo @bar {
            BAZ
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::EnumTypeDefinition(_),
                }),
            })
        );
    })
}

#[test]
fn enum_definition_value() {
    let source = r#"
        enum Foo {
            BAZ
        }
        "#;
    test_schema_resolution(source, "BAZ", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::EnumValueDefinitionName(_),
            })
        );
    })
}

#[test]
fn enum_definition_value_directive() {
    let source = r#"
       enum Foo {
            BAZ @bar
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::EnumValueDefinition(_),
                }),
            })
        );
    })
}

// ## Enum Type Extensions

#[test]
fn enum_extension_name() {
    let source = r#"
        extend enum Foo {
            BAR
            BAZ
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::EnumTypeExtensionName(_),
            })
        );
    })
}

#[test]
fn enum_extension_directive() {
    let source = r#"
        extend enum Foo @bar {
            BAZ
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::EnumTypeExtension(_),
                }),
            })
        );
    })
}

#[test]
fn enum_extension_value() {
    let source = r#"
        extend enum Foo {
            BAZ
        }
        "#;
    test_schema_resolution(source, "BAZ", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::EnumValueDefinitionName(_),
            })
        );
    })
}

#[test]
fn enum_extension_value_directive() {
    let source = r#"
        extend enum Foo {
            BAZ @bar
        }
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::EnumValueDefinition(_),
                }),
            })
        );
    })
}

// ## Scalar Types

#[test]
fn scalar_definition_name() {
    let source = r#"
        scalar Foo
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ScalarTypeDefinitionName(_),
            })
        );
    })
}

#[test]
fn scalar_definition_directive() {
    let source = r#"
        scalar Foo @bar
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::ScalarTypeDefinition(_),
                }),
            })
        );
    })
}

// ## Scalar Type Extensions

#[test]
fn scalar_extension_name() {
    let source = r#"
        extend scalar Foo
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ScalarTypeExtensionName(_),
            })
        );
    })
}

#[test]
fn scalar_extension_directive() {
    let source = r#"
        extend scalar Foo @bar
        "#;
    test_schema_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::ScalarTypeExtension(_),
                }),
            })
        );
    })
}

// ## Schema Definition

#[test]
fn schema_definition_operation_type_type_name() {
    let source = r#"
        schema {
            query: Foo
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::OperationTypeDefinitionType(OperationTypeDefinitionPath {
                    inner: _,
                    parent: OperationTypeDefinitionParent::SchemaDefinition(_),
                }),
            })
        );
    })
}

#[test]
fn schema_definition_directive() {
    let source = r#"
        schema @foo {
            query: Bar
        }
        "#;
    test_schema_resolution(source, "foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::SchemaDefinition(_),
                }),
            })
        );
    })
}

// ## Schema Extension

#[test]
fn schema_extension_operation_type_type_name() {
    let source = r#"
        extend schema {
            query: Foo
        }
        "#;
    test_schema_resolution(source, "Foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::OperationTypeDefinitionType(OperationTypeDefinitionPath {
                    inner: _,
                    parent: OperationTypeDefinitionParent::SchemaExtension(_),
                }),
            })
        );
    })
}

#[test]
fn schema_extension_directive() {
    let source = r#"
        extend schema @foo {
            query: Bar
        }
        "#;
    test_schema_resolution(source, "foo", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ConstantDirectiveName(ConstantDirectivePath {
                    inner: _,
                    parent: ConstantDirectiveParent::SchemaExtension(_),
                }),
            })
        );
    })
}

// TODOS:
// - directives for all
// - default input value
// - interface implementing interface
// - constant directive arguments
