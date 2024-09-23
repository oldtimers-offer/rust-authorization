use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // The subject (user identifier)
    pub exp: usize,  // Expiry time
}

pub static SECRET_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    dotenv().ok(); // Load .env file
    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    key.into_bytes() // Convert String to Vec<u8>
});

pub struct AuthenticatedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or(Status::Unauthorized)
            .expect("Missing token");

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(&SECRET_KEY),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                let claims = token_data.claims;
                Outcome::Success(AuthenticatedUser {
                    user_id: claims.sub,
                })
            }
            Err(_) => Outcome::Forward(Status::Unauthorized),
        }
    }
}
