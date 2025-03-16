use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn get_pool() -> MysqlPool {
    // TODO: pass the connection URL into this function rather than extracting
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set...!"); // TODO: handle errors
    let mgr = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool") // TODO: handle errors
}
