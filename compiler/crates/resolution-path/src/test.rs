/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use common::Span;
use graphql_syntax::parse_executable_with_features;
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
fn operation_definition_name() {
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
fn operation_definition_variable_definition_name() {
    let source = r#"
            query Foo($bar: ID!) {
                me {
                    id
                }
            }
        "#;
    test_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::VariableIdentifier(VariableIdentifierPath {
                inner: _,
                parent: VariableIdentifierParent::VariableDefinition(VariableDefinitionPath {
                    inner: _,
                    parent: VariableDefinitionListPath {
                        inner: _,
                        parent: VariableDefinitionListParent::OperationDefinition(_),
                    },
                }),
            })
        );
    })
}

#[test]
fn operation_definition_variable_definition_type() {
    let source = r#"
            query Foo($bar: ID!) {
                me {
                    id
                }
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
fn operation_definition_variable_definition_default_value() {
    let source = r#"
            query Foo($localId: ID! = "1") {
                me {
                    id
                }
            }
        "#;
    test_resolution(source, r#""1""#, |resolved| {
        assert_matches!(resolved, ResolutionPath::ConstantString(_));
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
fn fragment_spread_argument_name() {
    let source = r#"
            fragment Foo on Node {
                ...someFragment(someArg: 5)
            }
        "#;
    test_resolution(source, "someArg", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::ArgumentName(ArgumentPath {
                    inner: _,
                    parent: ArgumentParent::FragmentSpread(_),
                }),
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
fn fragment_argument_definition_name() {
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
fn fragment_argument_definition_type() {
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
fn fragment_argument_definition_default_value() {
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
fn fragment_argument_definition_directive() {
    let source = r#"
        fragment Foo($localId: ID! = "1" @bar) on User {
            id
          }
        "#;
    test_resolution(source, "bar", |resolved| {
        assert_matches!(
            resolved,
            ResolutionPath::Ident(IdentPath {
                inner: _,
                parent: IdentParent::DirectiveName(_)
            })
        );
    })
}
