use super::context::GraphQLContext;
use juniper::{FieldResult, RootNode};

use super::data::Users;
use super::models::{CreateUserInput, User};

// The root GraphQL query
pub struct Query;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    // This annotation isn't really necessary, as Juniper would convert the
    // all_todos function name into CamelCase. But I like to keep it explicit.
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        // TODO: pass the GraphQLContext into the querying functions rather
        // than a PgConnection (for brevity's sake)
        let mut connection = context.pool.get().unwrap();

        Users::all_users(&mut connection)
    }

    #[graphql(name = "getUserById")]
    pub fn get_user_by_id(context: &GraphQLContext, id: i32) -> FieldResult<Option<User>> {
        let mut connection = context.pool.get().unwrap();

        Users::get_user_by_id(&mut connection, id)
    }
}

// The root GraphQL mutation
pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    #[graphql(name = "createUser")]
    pub fn create_user(
        context: &GraphQLContext,
        input: CreateUserInput,
    ) -> FieldResult<Option<User>> {
        let mut connection = context.pool.get().unwrap();
        Users::create_user(&mut connection, input)
    }

    #[graphql(name = "setUserName")]
    pub fn set_user_name(
        context: &GraphQLContext,
        id: i32,
        new_username: String,
    ) -> FieldResult<Option<User>> {
        let mut connection = context.pool.get().unwrap();
        Users::set_user_name(&mut connection, id, new_username)
    }

    #[graphql(name = "setUserPassword")]
    pub fn set_user_password(
        context: &GraphQLContext,
        id: i32,
        new_password: String,
    ) -> FieldResult<Option<User>> {
        let mut connection = context.pool.get().unwrap();
        Users::set_user_password(&mut connection, id, new_password)
    }
}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, juniper::EmptySubscription::new())
}
