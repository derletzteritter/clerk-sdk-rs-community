use serde::{Serialize, Deserialize};

use crate::Client;

pub struct PhoneNumbers;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberParams {
    pub user_id: String,
    pub phone_number: String,
    pub verified: Option<bool>,
    pub primary: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePhoneNumberResponse {
    pub id: String,
    pub object: String,
    pub phone_number: String,
    pub reserved_for_second_factor: bool,
    pub default_second_factor: bool,
    pub reserved: bool,
    // ..add more
}

impl PhoneNumbers {
    pub async fn create(
        client: &Client,
        params: PhoneNumberParams,
    ) -> Result<CreatePhoneNumberResponse, reqwest::Error> {
        let url = format!("{}{}", client.base_url, "phone_numbers");

        match client.http_client.post(&url).json(&params).send().await {
            Ok(response) => match response.json::<CreatePhoneNumberResponse>().await {
                Ok(user) => Ok(user),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn retrive(client: &Client, id: &str) -> Result<CreatePhoneNumberResponse, reqwest::Error> {
        let url = format!("{}{}{}", client.base_url, "phone_numbers/", id);

        match client.http_client.get(&url).send().await {
            Ok(response) => match response.json::<CreatePhoneNumberResponse>().await {
                Ok(user) => Ok(user),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}
