use model::{SMSMessage, SMSMessageResponse, User, DeleteResponse, UpdateUserParams, CreateUserParams, UpdateUserMetadata};
use reqwest::header::{HeaderMap, AUTHORIZATION};

mod model;

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

    // USERS
    pub async fn list_all_users(&self) -> Result<Vec<User>, reqwest::Error> {
        let url = format!("{}{}", self.base_url, "users");

        self.http_client 
            .get(&url)
            .send()
            .await?
            .json::<Vec<User>>().await
    }

    pub async fn read_user(&self, user_id: String) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}", self.base_url, "users/", user_id);

        self.http_client
            .get(&url)
            .send()
            .await?
            .json::<User>().await
    }
    
    pub async fn delete_user(&self, user_id: String) -> Result<DeleteResponse, reqwest::Error> {
        let url = format!("{}{}{}", self.base_url, "users/", user_id);
        self.http_client
            .delete(&url)
            .send()
            .await?.json::<DeleteResponse>().await
    }

    pub async fn create_user(&self, params: CreateUserParams) -> Result<User, reqwest::Error> {
        let url = format!("{}{}", self.base_url, "users");

        self.http_client.post(&url).json(&params).send().await?.json::<User>().await
    }

    pub async fn update_user(&self, user_id: String, params: UpdateUserParams) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}", self.base_url, "users/", user_id);

        self.http_client.patch(&url).json(&params).send().await?.json::<User>().await
    }

    pub async fn update_user_metadata(&self, user_id: String, metadata_request: UpdateUserMetadata) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}/metadata", self.base_url, "users/", user_id);

        self.http_client.patch(&url).json(&metadata_request).send().await?.json::<User>().await
    }

    pub async fn ban_user(&self, user_id: String) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}/ban", self.base_url, "users/", user_id);

        self.http_client.post(&url).send().await?.json::<User>().await
    }

    pub async fn unban_user(&self, user_id: String) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}/unban", self.base_url, "users/", user_id);

        self.http_client.post(&url).send().await?.json::<User>().await
    }

    // SMS
    pub async fn create_sms(&self, message: SMSMessage) -> Result<SMSMessageResponse, reqwest::Error> {
        let url = format!("{}{}", self.base_url, "sms_messages");

        self.http_client.post(&url).json(&message).send().await?.json::<SMSMessageResponse>().await
    }
}
