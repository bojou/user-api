mod repository;

#[macro_use]
extern crate rocket;
use crate::repository::user::User;
use rocket::serde::json::Json;
use sqlx::PgPool;

#[post("/user", data = "<user>")]
fn add_user(user: Json<User>) -> String {
    format!("Adding {:?}", user.into_inner())
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let pool = PgPool::connect("postgres://jonatanberko@localhost/user_api")
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    rocket::build().manage(pool).mount("/", routes![add_user])
}
