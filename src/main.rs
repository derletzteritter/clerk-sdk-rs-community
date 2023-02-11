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
pub struct DeleteResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub deleted: bool,
}

impl Client {
    fn new(token: String) -> Client {
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

        let res = self.http_client 
            .get(&url)
            .send()
            .await;
            

        let result = match res {
            Ok(res) => {
                let users = res.json::<Vec<User>>().await;
                Ok(users)
            }
            Err(err) => Err(err),
        };

        match result {
            Ok(users) => {
                match users {
                    Ok(users) => {
                        Ok(users)
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(err),
        }
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
}

#[tokio::main]
async fn main() {
    let client = Client::new("".to_string());

    let users = client.read_user("".to_string()).await.unwrap();

    println!("{:?}", users);
}
