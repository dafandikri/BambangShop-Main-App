use crate::model::notification::Notification;
use bambangshop::REQWEST_CLIENT;
use rocket::log;
use rocket::serde::json::to_string;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}
