use serde::{Deserialize, Serialize};

use crate::{Client, error::{ClientError, ErrorResponse}};

#[derive(Debug, Deserialize, Serialize)]
pub struct SMSMessage {
    pub message: String,
    pub phone_number_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SMSMessageResponse {
    pub object: String,
    pub id: String,
    pub from_phone_number: String,
    pub to_phone_number: String,
    pub status: String,
    pub delivery_status: bool,
    pub data: serde_json::Value,
}

pub struct SMS {}

impl SMS {
    pub async fn create_sms(client: &Client, message: SMSMessage) -> Result<SMSMessageResponse, ClientError> {
        let url = format!("{}{}", client.base_url, "sms_messages");
        
        let res = client.http_client.post(&url).json(&message).send().await.map_err(ClientError::Reqwest)?;
        let status = res.status();

        if status.is_success() {
            let body = res.json::<SMSMessageResponse>().await.map_err(ClientError::Reqwest)?;

            Ok(body)
        } else {
            let err_body: ErrorResponse = res.json().await?;

            Err(ClientError::ErrorResponse(err_body))
        }
    }
}
