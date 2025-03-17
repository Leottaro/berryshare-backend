use crate::context::GraphQLContext;
use crate::models::{CreateUserInput, NewUser, User};
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct QueryManager;

// Every function in here should map directly to a function of the graphql schema
// fore example :	getUserById(id: Int): User
// becomes 		:	get_user_by_id(context: &GraphQLContext, user_id: i32) -> FieldResult<Option<User>>
impl QueryManager {
    // Query

    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let res = users.load::<User>(&mut *context.get_connection().lock().unwrap());
        match res {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn get_user_by_id(context: &GraphQLContext, user_id: i32) -> FieldResult<Option<User>> {
        let res = users
            .find(user_id)
            .get_result::<User>(&mut *context.get_connection().lock().unwrap());

        match res {
            Ok(todo) => Ok(Some(todo)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }

    pub fn login_user(
        context: &GraphQLContext,
        login_email: String,
        login_password: String,
    ) -> FieldResult<Option<User>> {
        let res = users
            .filter(email.eq(login_email).and(password.eq(login_password)))
            .get_result::<User>(&mut *context.get_connection().lock().unwrap());

        match res {
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

    pub fn delete_user(
        context: &GraphQLContext,
        user_id: i32,
        user_password: String,
    ) -> FieldResult<Option<User>> {
        let deleted_user = users
            .find(user_id)
            .filter(password.eq(user_password))
            .get_result::<User>(&mut *context.get_connection().lock().unwrap());

        match deleted_user {
            Ok(user) => {
                diesel::delete(users.find(user_id))
                    .execute(&mut *context.get_connection().lock().unwrap())?;
                FieldResult::Ok(Some(user))
            }
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(e)),
            },
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
