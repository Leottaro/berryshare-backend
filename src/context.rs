use std::sync::Mutex;

use diesel::{Connection, MysqlConnection};

// The GraphQL context, which needs to provide everything necessary for interacting with the database.
pub struct GraphQLContext {
    connection: Mutex<MysqlConnection>,
}

impl GraphQLContext {
    pub fn new(database_url: String) -> Self {
        let mysql_connection = MysqlConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Self {
            connection: Mutex::new(mysql_connection),
        }
    }

    pub fn get_connection(&self) -> &Mutex<MysqlConnection> {
        &self.connection
    }
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL objects
impl juniper::Context for GraphQLContext {}
