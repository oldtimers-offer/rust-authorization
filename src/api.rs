use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_user(conn: &mut AsyncPgConnection, user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)
        .await
}

pub async fn find_user_by_username(
    conn: &mut AsyncPgConnection,
    search_username: &str,
) -> QueryResult<usize> {
    users::table
        .filter(users::username.eq(search_username))
        .execute(conn)
        .await
}
