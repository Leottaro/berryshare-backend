use crate::models::{CreateUserInput, NewUser, User};
use crate::schema::users::{self, dsl::*};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for interacting
// with the database.
pub struct Users;

// Note that all the function names here map directly onto the function names
// associated with the Query and Mutation structs. This is NOT necessary but
// I personally prefer it.
impl Users {
    // Query

    pub fn all_users(connection: &mut MysqlConnection) -> FieldResult<Vec<User>> {
        match users.load::<User>(connection) {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn get_user_by_id(
        connection: &mut MysqlConnection,
        user_id: i32,
    ) -> FieldResult<Option<User>> {
        match users.find(user_id).get_result::<User>(connection) {
            Ok(todo) => Ok(Some(todo)),
            Err(e) => match e {
                // Without this translation, GraphQL will return an error rather
                // than the more semantically sound JSON null if no User is found.
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }

    // Mutation

    pub fn create_user(
        connection: &mut MysqlConnection,
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
            .execute(connection);

        match res {
            Ok(0) => FieldResult::Ok(None),
            Ok(_) => {
                let created_user = users
                    .order(id.desc())
                    .first::<User>(connection)
                    .optional()?;
                FieldResult::Ok(created_user)
            }
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn set_user_name(
        connection: &mut MysqlConnection,
        user_id: i32,
        new_username: String,
    ) -> FieldResult<Option<User>> {
        let res = diesel::update(users.filter(users::id.eq(user_id)))
            .set(username.eq(new_username))
            .execute(connection);

        match res {
            Ok(0) => FieldResult::Ok(None),
            Ok(_) => {
                let created_user = users
                    .order(id.desc())
                    .first::<User>(connection)
                    .optional()?;
                FieldResult::Ok(created_user)
            }
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    pub fn set_user_password(
        connection: &mut MysqlConnection,
        user_id: i32,
        new_password: String,
    ) -> FieldResult<Option<User>> {
        let res = diesel::update(users.filter(id.eq(user_id)))
            .set(password.eq(new_password))
            .execute(connection);

        match res {
            Ok(0) => FieldResult::Ok(None),
            Ok(_) => {
                let created_user = users
                    .order(id.desc())
                    .first::<User>(connection)
                    .optional()?;
                FieldResult::Ok(created_user)
            }
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }
}
