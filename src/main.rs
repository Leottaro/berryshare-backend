use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, middleware, route,
    web::{self, Data},
};
use berryshare_backend::{
    context::GraphQLContext,
    graphql::{Schema, create_schema},
};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

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

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());
    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
