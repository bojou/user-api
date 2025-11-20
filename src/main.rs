mod repository;
mod routes;

#[macro_use]
extern crate rocket;
use sqlx::PgPool;

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
    rocket::build()
        .manage(pool)
        .mount("/", routes![routes::user::add_user])
}
