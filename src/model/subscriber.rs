use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification) {
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send()
            .await
            .ok();

        log::warn!("Sent {} notification of: [{}], to: {}",
            payload.status, payload.id, payload.product_title, self.url);
    }
}
