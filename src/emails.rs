use serde::{Deserialize, Serialize};

use crate::{Client, error::{ClientError, ErrorResponse}};

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

    pub async fn create_email(client: &Client, email: EmailParams) -> Result<EmailResponse, ClientError> {
        let url = format!("{}{}", client.base_url, "emails");

        let res = client.http_client.post(&url).json(&email).send().await.map_err(ClientError::Reqwest)?;
        let status = res.status();

        if status.is_success() {
            let body = res.json::<EmailResponse>().await.map_err(ClientError::Reqwest)?;

            Ok(body)
        } else {
            let err_body: ErrorResponse = res.json().await?;

            Err(ClientError::ErrorResponse(err_body))
        }
    }
}
