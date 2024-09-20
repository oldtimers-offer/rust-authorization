use crate::api::create_user;
use crate::db::server_error;
use crate::models::{NewUser, User};
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket_db_pools::Connection;

#[rocket::post("/register", format = "json", data = "<new_user>")]
pub async fn register(
    mut db: Connection<DbConn>,
    new_user: Json<NewUser>,
) -> Result<Value, Custom<Value>> {
    create_user(&mut db, user.into_inner())
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| server_error(e.into()))
}
