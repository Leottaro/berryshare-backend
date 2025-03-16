use super::context::GraphQLContext;
use super::db::MysqlPool;
use super::graphql::Schema;
use super::graphql::create_schema;
use actix_web::{Error, HttpResponse, web};
use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;
use std::sync::Arc;

// The configuration callback that enables us to add the /graphql route
// to the actix-web server.
pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .app_data(schema)
        .route("/graphql", web::post().to(graphql))
        .route("/graphql", web::get().to(graphql_playground));
}

// The GraphQL Playground route.
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}

// The core handler that provides all GraphQL functionality.
async fn graphql(
    // The DB connection pool
    pool: web::Data<MysqlPool>,
    // The GraphQL schema
    schema: web::Data<Arc<Schema>>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    // Handle the incoming request and return a string result (or error)
    let res = web::block(async move || {
        let res = data.execute(&schema, &ctx).await;
        serde_json::to_string(&res)
    })
    .await
    .map_err(Error::from)?
    .await
    .map_err(Error::from)?;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
