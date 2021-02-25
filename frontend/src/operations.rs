use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct Register;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct AllDocuments;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct Document;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct Page;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct CreateAnchor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct DeleteAnchor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct CreateUserAnchor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct DeleteUserAnchor;
