use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Default, FromRow)]
#[serde(crate = "rocket::serde")]
pub(crate) struct User {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password_hash: String,
    #[serde(skip_deserializing, default = "Utc::now")]
    pub(crate) created_at: DateTime<Utc>,
}

pub(crate) async fn upsert(pool: &PgPool, user: &User) -> Result<(), Box<dyn Error>> {
    let query = r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        ON CONFLICT (email) DO UPDATE
        SET username = EXCLUDED.username,
            password_hash = EXCLUDED.password_hash
    "#;
    sqlx::query(query)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.password_hash)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn find_by_email(pool: &PgPool, email: &String) -> Result<User, Box<dyn Error>> {
    let query = "SELECT email, username, password_hash, created_at FROM users WHERE email = $1";
    let user = sqlx::query_as::<_, User>(query)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
