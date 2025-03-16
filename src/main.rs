extern crate actix_rt;
extern crate actix_web;
extern crate berryshare_backend;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
extern crate r2d2;

use std::{env, io};

use actix_web::{App, HttpServer, middleware};

use berryshare_backend::db::get_pool;
use berryshare_backend::endpoints::graphql_endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    // Instantiate a new connection pool
    let pool = get_pool();

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    unsafe { env::set_var("RUST_LOG", "actix_web=info") };
    env_logger::init();
}
