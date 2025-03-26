use rocket::response::status::Created;
use rocket::serde::json::Json;

use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;
use bambangshop::Result;
