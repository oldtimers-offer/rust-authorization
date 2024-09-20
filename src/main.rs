use crate::db::DbConn;
use crate::handlers::register;
use rocket::routes;
use rocket_db_pools::Database;

mod api;
mod db;
mod handlers;
mod hashing;
mod models;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/hello", routes![register])
        .attach(DbConn::init())
        .launch()
        .await;
}
