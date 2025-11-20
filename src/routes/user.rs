use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::PgPool;
use crate::repository::user::{User, upsert};

#[post("/user", data = "<user>")]
pub async fn add_user(pool: &State<PgPool>, user: Json<User>) -> Result<Status, Status> {
    upsert(pool, &user.into_inner())
        .await
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}