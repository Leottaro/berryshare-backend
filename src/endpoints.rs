use super::context::GraphQLContext;
use super::graphql::Schema;
use actix_web::{HttpResponse, web};
use juniper::http::GraphQLRequest;

use actix_web::{Responder, get, route};
use juniper::http::graphiql::graphiql_source;

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graphql", None))
}

// The core handler that provides all GraphQL functionality.
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|err| panic!("Couldn't get DATABASE_URL environment variable: {}", err));
    let context = GraphQLContext::new(database_url);
    let res = data.execute(&st, &context).await;
    HttpResponse::Ok().json(res)
}
