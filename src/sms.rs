use serde::{Deserialize, Serialize};

use crate::Client;

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
    pub async fn create_sms(client: &Client, message: SMSMessage) -> Result<SMSMessageResponse, reqwest::Error> {
        let url = format!("{}{}", client.base_url, "sms_messages");


        match client.http_client.post(&url).json(&message).send().await {
            Ok(res) => {
                match res.json::<SMSMessageResponse>().await {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
