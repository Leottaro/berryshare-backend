use super::schema::users;
use juniper::{GraphQLInputObject, GraphQLObject};

// The core data type undergirding the GraphQL interface
#[derive(Queryable, GraphQLObject)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
    pub admin: bool,
}

// Used to create new TODOs
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub username: &'a str,
    pub admin: bool,
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub username: String,
    pub admin: Option<bool>,
}
