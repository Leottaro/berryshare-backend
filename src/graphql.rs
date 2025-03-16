use super::context::GraphQLContext;
use juniper::{EmptySubscription, FieldResult, RootNode};

use super::data::QueryManager;
use super::models::{CreateUserInput, User};

// The root GraphQL query
pub struct QueryRoot;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    // This annotation isn't really necessary, as Juniper would convert the
    // all_users function name into CamelCase. But I like to keep it explicit.
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        QueryManager::all_users(context)
    }

    #[graphql(name = "getUserById")]
    pub fn get_user_by_id(context: &GraphQLContext, id: i32) -> FieldResult<Option<User>> {
        QueryManager::get_user_by_id(context, id)
    }

    #[graphql(name = "loginUser")]
    pub fn login_user(
        context: &GraphQLContext,
        email: String,
        password: String,
    ) -> FieldResult<Option<User>> {
        QueryManager::login_user(context, email, password)
    }
}

// The root GraphQL mutation
pub struct MutationRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl MutationRoot {
    #[graphql(name = "createUser")]
    pub fn create_user(
        context: &GraphQLContext,
        input: CreateUserInput,
    ) -> FieldResult<Option<User>> {
        QueryManager::create_user(context, input)
    }

    #[graphql(name = "deleteUser")]
    pub fn delete_user(
        context: &GraphQLContext,
        id: i32,
        password: String,
    ) -> FieldResult<Option<User>> {
        QueryManager::delete_user(context, id, password)
    }

    #[graphql(name = "setUserName")]
    pub fn set_user_name(
        context: &GraphQLContext,
        id: i32,
        new_username: String,
    ) -> FieldResult<Option<User>> {
        QueryManager::set_user_name(context, id, new_username)
    }

    #[graphql(name = "setUserPassword")]
    pub fn set_user_password(
        context: &GraphQLContext,
        id: i32,
        new_password: String,
    ) -> FieldResult<Option<User>> {
        QueryManager::set_user_password(context, id, new_password)
    }
}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
