use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailAddress {
    pub id: Option<String>,
    pub object: Option<String>,
    pub email_address: Option<String>,
    pub reserved: bool,
    pub linked_to: Option<Vec<LinkedTo>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkedTo {
    pub r#type: String,
    pub id: String,
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
    pub public_metadata: serde_json::Value,
    pub private_metadata: serde_json::Value,
    pub unsafe_metadata: serde_json::Value,
}
