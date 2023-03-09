use serde::{Deserialize, Serialize};

use crate::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailResponse {
    object: String,
    id: String,
    slug: String,
    from_email_name: String,
    email_address_id: String,
    to_email_address: String,
    user_id: String,
    subject: String,
    body: String,
    body_plain: String,
    status: String,
    delivered_by_clerk: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailParams {
    from_email_name: String,
    email_address_id: String,
    body: String,
    subject: String,
    template_slug: String,
}

pub struct Email {}

impl Email {

    pub async fn create_email(client: &Client, email: EmailParams) -> Result<EmailResponse, reqwest::Error> {
        let url = format!("{}{}", client.base_url, "emails");

        match client.http_client.post(&url).json(&email).send().await {
            Ok(res) => {
                match res.json::<EmailResponse>().await {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
