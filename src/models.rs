use super::schema::users;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub name: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub username: &'a str,
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub username: String,
}
