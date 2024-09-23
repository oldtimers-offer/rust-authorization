use crate::verify_password;
use crate::LoginInput;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rocket::response::status;
use rocket::serde::json::Json;

use crate::models::*;
use crate::schema::*;

pub struct UserRepo;

impl UserRepo {
    pub async fn create(c: &mut AsyncPgConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(c)
            .await
    }

    pub async fn login(
        c: &mut AsyncPgConnection,
        login: Json<LoginInput>,
    ) -> Result<&'static str, status::Custom<&'static str>> {
        match users::table
            .filter(users::username.eq(&login.username))
            .get_result::<User>(c)
            .await
        {
            Ok(user) => {
                if verify_password(&user.password_hash, &login.password) {
                    Ok("Login successful")
                } else {
                    Err(status::Custom(
                        rocket::http::Status::Unauthorized,
                        "Invalid username or password",
                    ))
                }
            }
            Err(DieselError::NotFound) => Err(status::Custom(
                rocket::http::Status::NotFound,
                "User not found",
            )),
            Err(_) => Err(status::Custom(
                rocket::http::Status::InternalServerError,
                "Database error",
            )),
        }
    }
}
