use crate::context::GraphQLContext;
use crate::models::{CreateUserInput, NewUser, User};
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct QueryManager;

// Every function in here should map directly to a function of the graphql schema
// fore example	getUserById(id: Int): User
// becomes 		get_user_by_id(context: &GraphQLContext, user_id: i32) -> FieldResult<Option<User>>
impl QueryManager {
    // Query

    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        match users.load::<User>(&mut *context.get_connection().lock().unwrap()) {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn get_user_by_id(context: &GraphQLContext, user_id: i32) -> FieldResult<Option<User>> {
        match users
            .find(user_id)
            .get_result::<User>(&mut *context.get_connection().lock().unwrap())
        {
            Ok(todo) => Ok(Some(todo)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }

    // Mutation

    pub fn create_user(
        context: &GraphQLContext,
        new_user: CreateUserInput,
    ) -> FieldResult<Option<User>> {
        let new_user = NewUser {
            email: &new_user.email,
            password: &new_user.password,
            username: &new_user.username,
            admin: new_user.admin.unwrap_or(false),
        };

        let res = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut *context.get_connection().lock().unwrap());

        match res {
            Ok(0) => FieldResult::Ok(None),
            Ok(_) => {
                let created_user = users
                    .order(id.desc())
                    .first::<User>(&mut *context.get_connection().lock().unwrap())
                    .optional()?;
                FieldResult::Ok(created_user)
            }
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn set_user_name(
        context: &GraphQLContext,
        user_id: i32,
        new_username: String,
    ) -> FieldResult<Option<User>> {
        let res = diesel::update(users.filter(users::id.eq(user_id)))
            .set(username.eq(new_username))
            .execute(&mut *context.get_connection().lock().unwrap());

        match res {
            Ok(0) => FieldResult::Ok(None),
            Ok(_) => {
                let created_user = users
                    .order(id.desc())
                    .first::<User>(&mut *context.get_connection().lock().unwrap())
                    .optional()?;
                FieldResult::Ok(created_user)
            }
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn set_user_password(
        context: &GraphQLContext,
        user_id: i32,
        new_password: String,
    ) -> FieldResult<Option<User>> {
        let res = diesel::update(users.filter(id.eq(user_id)))
            .set(password.eq(new_password))
            .execute(&mut *context.get_connection().lock().unwrap());

        match res {
            Ok(0) => FieldResult::Ok(None),
            Ok(_) => {
                let created_user = users
                    .order(id.desc())
                    .first::<User>(&mut *context.get_connection().lock().unwrap())
                    .optional()?;
                FieldResult::Ok(created_user)
            }
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }
}
