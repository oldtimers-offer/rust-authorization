use crate::token::{Claims, SECRET_KEY};
use crate::verify_password;
use crate::LoginInput;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

use crate::models::*;
use crate::schema::*;

pub struct UserRepo;

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

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
    ) -> Result<Json<TokenResponse>, status::Custom<&'static str>> {
        match users::table
            .filter(users::username.eq(&login.username))
            .get_result::<User>(c)
            .await
        {
            Ok(user) => {
                if verify_password(&user.password_hash, &login.password) {
                    // Generate JWT token
                    let expiration = Utc::now()
                        .checked_add_signed(Duration::seconds(3))
                        .expect("valid timestamp")
                        .timestamp() as usize;

                    let claims = Claims {
                        sub: user.id.to_string(),
                        exp: expiration,
                    };

                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(&SECRET_KEY),
                    )
                    .map_err(|_| {
                        status::Custom(
                            rocket::http::Status::InternalServerError,
                            "Token creation error",
                        )
                    })?;

                    Ok(Json(TokenResponse { token }))
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
