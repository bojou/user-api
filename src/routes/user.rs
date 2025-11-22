use crate::error::ExceptionHandler;
use crate::repository::user::{User, find_by_email, upsert};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use sqlx::PgPool;

#[post("/user", data = "<user>")]
pub async fn add_user(pool: &State<PgPool>, user: Json<User>) -> Result<Status, ExceptionHandler> {
    upsert(pool, &user.into_inner())
        .await
        .map(|_| Status::Created)
        .map_err(ExceptionHandler::handle)
}

#[post("/get", data = "<email>")]
pub async fn get_user(
    pool: &State<PgPool>,
    email: Json<String>,
) -> Result<Json<User>, ExceptionHandler> {
    find_by_email(pool, &email.into_inner())
        .await
        .map(Json)
        .map_err(ExceptionHandler::handle)
}
