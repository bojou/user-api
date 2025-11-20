use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub(crate) struct User {
    pub(crate) email: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}
