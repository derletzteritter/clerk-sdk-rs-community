use serde::{Serialize, Deserialize};

use crate::{Client, error::{ErrorResponse, ClientError}};

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
    ) -> Result<CreatePhoneNumberResponse, ClientError> {
        let url = format!("{}{}", client.base_url, "phone_numbers");

        let res = client.http_client.post(&url).json(&params).send().await?;
        let status = res.status();

        if status.is_success() {
            let body = res.json::<CreatePhoneNumberResponse>().await?;

            Ok(body)
        } else {
            let err_body: ErrorResponse = res.json().await?;

            Err(ClientError::ErrorResponse(err_body))
        }
    }

    pub async fn retrive(client: &Client, id: &str) -> Result<CreatePhoneNumberResponse, ClientError> {
        let url = format!("{}{}{}", client.base_url, "phone_numbers/", id);
        
        let res = client.http_client.get(&url).send().await?;
        let status = res.status();

        if status.is_success() {
            let body = res.json::<CreatePhoneNumberResponse>().await?;

            Ok(body)
        } else {
            let err_body: ErrorResponse = res.json().await?;

            Err(ClientError::ErrorResponse(err_body))
        }
    }
}
