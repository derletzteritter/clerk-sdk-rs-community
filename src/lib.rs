use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};


pub struct Client {
    pub token: String,
    base_url: String,
    http_client: reqwest::Client,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailAddress {
    pub id: Option<String>,
    pub object: Option<String>,
    pub email_address: String,
    pub reserved: bool,
    pub linked_to: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub object: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub profile_image_url: Option<String>,
    pub primary_email_address_id: Option<String>,
    pub primary_phone_number_id: Option<String>,
    pub primary_web3_address_id: Option<String>,
    pub password_enabled: bool,
    pub two_factor_enabled: bool,
    pub totp_enabled: bool,
    pub backup_code_enabled: bool,
    pub email_addresses: Option<Vec<EmailAddress>>,

    // ... more fields
    pub external_id: Option<String>,
    pub last_sign_in_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub banned: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub web3_wallets: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_digest: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hasher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_password_requirements: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_password_check: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email_address_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_phone_number_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_web3_address_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    profile_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserMetadata {
    public_metadata: serde_json::Value,
    private_metadata: serde_json::Value,
    unsafe_metadata: serde_json::Value,
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
}
