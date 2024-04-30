use axum::handler::{HandlerWithoutStateExt, Layered};
use axum::{routing::post, routing::get, Router, extract::Extension, response::{Html, IntoResponse}, http::StatusCode};
use async_graphql::Schema;
use async_graphql_axum::GraphQL;
use async_graphql::{EmptySubscription, http::{GraphiQLSource, playground_source, GraphQLPlaygroundConfig}};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
mod graphql;
use graphql::{query_root::QueryRoot, mutation_root::MutationRoot};
use graphql::schema::MySchema;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
#[tokio::main]
async fn main() {
    let schema= Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Router::new().route("/", get(graphql_playground).post(graphql_handler)).layer(Extension(schema));
    let app = app.fallback(notfound_handler);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
async fn graphql_handler(
    Extension(schema): Extension<MySchema>, 
    req: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
async fn notfound_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found")
  }