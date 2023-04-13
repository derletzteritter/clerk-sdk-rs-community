use serde::{Deserialize, Serialize};
use crate::{Client, model::{DeleteResponse, CreateUserParams, UpdateUserParams, UpdateUserMetadata}, error::{ClientError, ErrorResponse}};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressVerification {
    pub status: String,
    pub strategy: String,
    pub attemps: Option<i32>,
    pub expire_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddressLinkedTo {
    pub r#type: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailAddress {
    pub id: String,
    pub object: String,
    pub email_address: String,
    pub reserved: bool,
    pub verification: Option<EmailAddressVerification>,
    pub linked_to: Option<Vec<EmailAddressLinkedTo>>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub object: String,
    pub external_id: Option<String>,
    pub primary_email_address_id: Option<String>,
    pub primary_phone_number_id: Option<String>,
    pub primary_web3_address_id: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_image_url: String,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub password_enabled: bool,
    pub two_factor_enabled: bool,
    pub totp_enabled: bool,
    pub backup_code_enabled: bool,
    pub email_addresses: Vec<EmailAddress>,
    pub last_sign_in_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub banned: bool,
}

impl User {
    pub async fn list_all_users(client: &Client) -> Result<Vec<User>, ClientError> {
        let url = format!("{}{}", client.base_url, "users");

        let res = client.http_client.get(&url).send().await.map_err(ClientError::Reqwest)?;
        let status = res.status();

        if status.is_success() {
            let body = res.json::<Vec<User>>().await.map_err(ClientError::Reqwest)?;
            Ok(body)
        } else {
            let err_body: ErrorResponse = res.json().await?;

            Err(ClientError::ErrorResponse(err_body))
        }
    }

    pub async fn read_user(client: &Client, user_id: String) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}", client.base_url, "users/", user_id);

        match client.http_client.get(&url).send().await {
            Ok(response) => {
                match response.json::<User>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete_user(client: &Client, user_id: String) -> Result<DeleteResponse, reqwest::Error> {
        let url = format!("{}{}{}", client.base_url, "users/", user_id);


        match client.http_client.delete(&url).send().await {
            Ok(response) => {
                match response.json::<DeleteResponse>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn create_user(client: &Client, params: CreateUserParams) -> Result<User, reqwest::Error> {
        let url = format!("{}{}", client.base_url, "users");

        match client.http_client.post(&url).json(&params).send().await {
            Ok(response) => {
                match response.json::<User>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update_user(client: &Client, user_id: String, params: UpdateUserParams) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}", client.base_url, "users/", user_id);
            
        match client.http_client.patch(&url).json(&params).send().await {
            Ok(response) => {
                match response.json::<User>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update_user_metadata(client: &Client, user_id: String, metadata_request: UpdateUserMetadata) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}/metadata", client.base_url, "users/", user_id);

        client.http_client.patch(&url).json(&metadata_request).send().await?.json::<User>().await
    }

    pub async fn ban_user(client: &Client, user_id: String) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}/ban", client.base_url, "users/", user_id);

        match client.http_client.post(&url).send().await {
            Ok(response) => {
                match response.json::<User>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn unban_user(client: &Client, user_id: String) -> Result<User, reqwest::Error> {
        let url = format!("{}{}{}/unban", client.base_url, "users/", user_id);

        match client.http_client.post(&url).send().await {
            Ok(response) => {
                match response.json::<User>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
