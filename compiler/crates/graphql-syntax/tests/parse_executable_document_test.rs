/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<b0a2958309c0f951547c9fcf5a6d0fe3>>
 */

mod parse_executable_document;

use parse_executable_document::transform_fixture;
use fixture_tests::test_fixture;

#[tokio::test]
async fn block_string() {
    let input = include_str!("parse_executable_document/fixtures/block_string.graphql");
    let expected = include_str!("parse_executable_document/fixtures/block_string.expected");
    test_fixture(transform_fixture, "block_string.graphql", "parse_executable_document/fixtures/block_string.expected", input, expected).await;
}

#[tokio::test]
async fn fragment_with_variable_defs_invalid() {
    let input = include_str!("parse_executable_document/fixtures/fragment_with_variable_defs.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/fragment_with_variable_defs.invalid.expected");
    test_fixture(transform_fixture, "fragment_with_variable_defs.invalid.graphql", "parse_executable_document/fixtures/fragment_with_variable_defs.invalid.expected", input, expected).await;
}

#[tokio::test]
async fn incomplete_field_alias() {
    let input = include_str!("parse_executable_document/fixtures/incomplete_field_alias.graphql");
    let expected = include_str!("parse_executable_document/fixtures/incomplete_field_alias.expected");
    test_fixture(transform_fixture, "incomplete_field_alias.graphql", "parse_executable_document/fixtures/incomplete_field_alias.expected", input, expected).await;
}

#[tokio::test]
async fn incorrect_variable_name_invalid() {
    let input = include_str!("parse_executable_document/fixtures/incorrect_variable_name.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/incorrect_variable_name.invalid.expected");
    test_fixture(transform_fixture, "incorrect_variable_name.invalid.graphql", "parse_executable_document/fixtures/incorrect_variable_name.invalid.expected", input, expected).await;
}

#[tokio::test]
async fn invalid_number() {
    let input = include_str!("parse_executable_document/fixtures/invalid_number.graphql");
    let expected = include_str!("parse_executable_document/fixtures/invalid_number.expected");
    test_fixture(transform_fixture, "invalid_number.graphql", "parse_executable_document/fixtures/invalid_number.expected", input, expected).await;
}

#[tokio::test]
async fn keyword_as_name() {
    let input = include_str!("parse_executable_document/fixtures/keyword_as_name.graphql");
    let expected = include_str!("parse_executable_document/fixtures/keyword_as_name.expected");
    test_fixture(transform_fixture, "keyword_as_name.graphql", "parse_executable_document/fixtures/keyword_as_name.expected", input, expected).await;
}

#[tokio::test]
async fn kitchen_sink() {
    let input = include_str!("parse_executable_document/fixtures/kitchen-sink.graphql");
    let expected = include_str!("parse_executable_document/fixtures/kitchen-sink.expected");
    test_fixture(transform_fixture, "kitchen-sink.graphql", "parse_executable_document/fixtures/kitchen-sink.expected", input, expected).await;
}

#[tokio::test]
async fn list_of_enum() {
    let input = include_str!("parse_executable_document/fixtures/list_of_enum.graphql");
    let expected = include_str!("parse_executable_document/fixtures/list_of_enum.expected");
    test_fixture(transform_fixture, "list_of_enum.graphql", "parse_executable_document/fixtures/list_of_enum.expected", input, expected).await;
}

#[tokio::test]
async fn missing_zero_on_float_invalid() {
    let input = include_str!("parse_executable_document/fixtures/missing_zero_on_float.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/missing_zero_on_float.invalid.expected");
    test_fixture(transform_fixture, "missing_zero_on_float.invalid.graphql", "parse_executable_document/fixtures/missing_zero_on_float.invalid.expected", input, expected).await;
}

#[tokio::test]
async fn multiple_parse_errors_invalid() {
    let input = include_str!("parse_executable_document/fixtures/multiple_parse_errors.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/multiple_parse_errors.invalid.expected");
    test_fixture(transform_fixture, "multiple_parse_errors.invalid.graphql", "parse_executable_document/fixtures/multiple_parse_errors.invalid.expected", input, expected).await;
}

#[tokio::test]
async fn space_in_variable() {
    let input = include_str!("parse_executable_document/fixtures/space_in_variable.graphql");
    let expected = include_str!("parse_executable_document/fixtures/space_in_variable.expected");
    test_fixture(transform_fixture, "space_in_variable.graphql", "parse_executable_document/fixtures/space_in_variable.expected", input, expected).await;
}

#[tokio::test]
async fn spread_with_arguments_invalid() {
    let input = include_str!("parse_executable_document/fixtures/spread_with_arguments.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/spread_with_arguments.invalid.expected");
    test_fixture(transform_fixture, "spread_with_arguments.invalid.graphql", "parse_executable_document/fixtures/spread_with_arguments.invalid.expected", input, expected).await;
}

#[tokio::test]
async fn unterminated_string_invalid() {
    let input = include_str!("parse_executable_document/fixtures/unterminated_string.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/unterminated_string.invalid.expected");
    test_fixture(transform_fixture, "unterminated_string.invalid.graphql", "parse_executable_document/fixtures/unterminated_string.invalid.expected", input, expected).await;
}
