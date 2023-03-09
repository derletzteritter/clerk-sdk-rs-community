use model::{SMSMessage, SMSMessageResponse, DeleteResponse, UpdateUserParams, CreateUserParams, UpdateUserMetadata};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use crate::users::User;

pub mod model;
pub mod users;

pub struct Client {
    pub token: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(token: String) -> Client {
        let auth_token = format!("Bearer {}", token);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_token.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();


        let prod_url = "https://api.clerk.dev/v1/";
        Client {
            token,
            base_url: prod_url.to_string(),
            http_client: client,
        }
    }

    // SMS
    pub async fn create_sms(&self, message: SMSMessage) -> Result<SMSMessageResponse, reqwest::Error> {
        let url = format!("{}{}", self.base_url, "sms_messages");

        self.http_client.post(&url).json(&message).send().await?.json::<SMSMessageResponse>().await
    }

    // EMAILS
    pub async fn create_email(&self, message: SMSMessage) -> Result<SMSMessageResponse, reqwest::Error> {
        let url = format!("{}{}", self.base_url, "emails");

        self.http_client.post(&url).json(&message).send().await?.json::<SMSMessageResponse>().await
    }
}
