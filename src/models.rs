use crate::schema::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Insertable;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: String,
}
