/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::Arc;

use common::DiagnosticsResult;
use common::Location;
use common::WithLocation;
use graphql_ir::Argument;
use graphql_ir::ConstantValue;
use graphql_ir::Field;
use graphql_ir::FragmentDefinition;
use graphql_ir::FragmentSpread;
use graphql_ir::InlineFragment;
use graphql_ir::LinkedField;
use graphql_ir::OperationDefinition;
use graphql_ir::Program;
use graphql_ir::ScalarField;
use graphql_ir::Selection;
use graphql_ir::Transformed;
use graphql_ir::Transformer;
use graphql_ir::Value;
use graphql_syntax::OperationKind;
use intern::Lookup;
use intern::string_key::Intern;
use schema::FieldID;
use schema::Schema;
use schema::Type;
use schema::TypeReference;

use crate::ModuleMetadata;
use crate::match_::MATCH_CONSTANTS;
use crate::util::get_normalization_operation_name;

pub fn transform_subscriptions(program: &Program) -> DiagnosticsResult<Program> {
    let mut transformer = SubscriptionTransform::new(program);
    let next_program = transformer.transform_program(program);
    Ok(next_program.replace_or_else(|| program.clone()))
}

pub struct SubscriptionTransform<'program> {
    program: &'program Program,
}

impl<'program> SubscriptionTransform<'program> {
    fn new(program: &'program Program) -> Self {
        Self { program }
    }

    /// Validate that the given operation meets the following conditions:
    /// - Is a subscription, which
    /// - has a single selection, which
    /// - is a linked field, whose type
    /// - is an object with a js: JSDependency field; and
    /// - the linked field has a single selection, which is a fragment spread
    ///   These restrictions may be loosened over time.
    ///
    /// Return Some(Vec<ValidFieldResult>) if the operation is valid
    /// (i.e. if each field is valid), or None otherwise.
    fn validate_operation<'operation>(
        &self,
        operation: &'operation OperationDefinition,
    ) -> Option<Vec<ValidFieldResult<'operation>>> {
        if matches!(operation.kind, OperationKind::Subscription) && operation.selections.len() == 1
        {
            operation
                .selections
                .iter()
                .map(|selection| self.validate_selection(selection))
                .collect::<Option<Vec<_>>>()
        } else {
            None
        }
    }

    fn validate_selection<'operation>(
        &self,
        selection: &'operation Selection,
    ) -> Option<ValidFieldResult<'operation>> {
        match selection {
            Selection::LinkedField(linked_field) => {
                let field_id = linked_field.definition.item;
                let field = self.program.schema.field(field_id);
                let type_ = field.type_.inner();

                let fragment_spread = self.validate_linked_field(linked_field)?;

                match type_ {
                    Type::Object(object_id) => {
                        let object = self.program.schema.object(object_id);
                        for object_field_id in object.fields.iter() {
                            let object_field = self.program.schema.field(*object_field_id);
                            if object_field.name.item == MATCH_CONSTANTS.js_field_name {
                                // if we find a js field, it must be valid
                                return self.is_valid_js_dependency(&object_field.type_).then_some(
                                    ValidFieldResult {
                                        linked_field,
                                        js_field_id: *object_field_id,
                                        fragment_spread,
                                    },
                                );
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// The linked field must contain only a single fragment spread.
    fn validate_linked_field<'operation>(
        &self,
        linked_field: &'operation Arc<LinkedField>,
    ) -> Option<&'operation FragmentSpread> {
        if linked_field.selections.len() != 1 {
            return None;
        }
        let first_item = linked_field.selections.first().unwrap();
        match first_item {
            Selection::FragmentSpread(fragment_spread) => Some(fragment_spread),
            _ => None,
        }
    }

    fn is_valid_js_dependency(&self, type_: &TypeReference<Type>) -> bool {
        match type_ {
            TypeReference::Named(Type::Scalar(scalar_id)) => {
                let scalar = self.program.schema.scalar(*scalar_id);
                scalar.name.item == MATCH_CONSTANTS.js_field_type && !scalar.is_extension
            }
            _ => false,
        }
    }

    fn get_replacement_selection(
        &self,
        operation: &OperationDefinition,
        valid_result: ValidFieldResult<'_>,
    ) -> Selection {
        let ValidFieldResult {
            linked_field,
            js_field_id,
            fragment_spread,
        } = valid_result;
        let location = linked_field.definition.location;
        let operation_name_with_suffix =
            format!("{}__subscription", operation.name.item.0.lookup());
        let normalization_operation_name = format!(
            "{}.graphql",
            get_normalization_operation_name(fragment_spread.fragment.item.0)
        )
        .intern();

        let mut selections = linked_field.selections.clone();
        selections.push(Selection::ScalarField(Arc::new(ScalarField {
            alias: Some(WithLocation::new(
                location,
                format!("__module_operation_{}", operation_name_with_suffix).intern(),
            )),
            definition: WithLocation::new(location, js_field_id),
            arguments: vec![Argument {
                name: WithLocation::new(location, MATCH_CONSTANTS.js_field_module_arg),
                value: WithLocation::new(
                    location,
                    Value::Constant(ConstantValue::String(normalization_operation_name)),
                ),
            }],
            directives: vec![],
        })));

        let type_condition = Some(
            self.program
                .schema
                .field(linked_field.definition.item)
                .type_
                .inner(),
        );
        let name_location = linked_field.alias_or_name_location();

        let selections = vec![Selection::InlineFragment(Arc::new(InlineFragment {
            type_condition,
            directives: vec![],
            selections: vec![Selection::InlineFragment(Arc::new(InlineFragment {
                type_condition,
                directives: vec![
                    ModuleMetadata {
                        key: operation_name_with_suffix.intern(),
                        module_id: format!(
                            "{}.{}",
                            operation.name.item.0,
                            linked_field.alias_or_name(&self.program.schema).lookup()
                        )
                        .intern(),
                        module_name: normalization_operation_name,
                        source_document_name: operation.name.item.into(),
                        read_time_resolvers: false,
                        fragment_name: fragment_spread.fragment.item,
                        fragment_source_location: self
                            .program
                            .fragment(fragment_spread.fragment.item)
                            .unwrap()
                            .name
                            .location,
                        location: name_location,
                        no_inline: false,
                    }
                    .into(),
                ],
                selections,
                spread_location: Location::generated(),
            }))],
            spread_location: Location::generated(),
        }))];

        Selection::LinkedField(Arc::new(LinkedField {
            alias: linked_field.alias,
            definition: linked_field.definition,
            arguments: linked_field.arguments.clone(),
            directives: linked_field.directives.clone(),
            selections,
        }))
    }
}

/// If an operation is valid, each field will be a LinkedField pointing to an
/// Object. The successful result of validating an operation will be a vector
/// where this information is extracted from each field, in the form of a
/// ValidFieldResult.
struct ValidFieldResult<'operation> {
    /// a reference to the valid linked field
    linked_field: &'operation Arc<LinkedField>,
    /// the FieldID of the js: JSDependency field in the schema
    js_field_id: FieldID,
    /// a reference to the contained fragment spread
    fragment_spread: &'operation FragmentSpread,
}

impl Transformer<'_> for SubscriptionTransform<'_> {
    const NAME: &'static str = "SubscriptionTransform";
    const VISIT_ARGUMENTS: bool = false;
    const VISIT_DIRECTIVES: bool = false;

    fn transform_operation(
        &mut self,
        operation: &OperationDefinition,
    ) -> Transformed<OperationDefinition> {
        if let Some(valid_results) = self.validate_operation(operation) {
            let selections = valid_results
                .into_iter()
                .map(|valid_result| self.get_replacement_selection(operation, valid_result))
                .collect::<Vec<_>>();

            Transformed::Replace(OperationDefinition {
                selections,
                ..operation.clone()
            })
        } else {
            Transformed::Keep
        }
    }

    fn transform_fragment(
        &mut self,
        _fragment: &FragmentDefinition,
    ) -> Transformed<FragmentDefinition> {
        Transformed::Keep
    }
}
