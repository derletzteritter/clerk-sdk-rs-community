use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct SMSMessage {
    messsage: String,
    phone_number_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SMSMessageResponse {
    object: String,
    id: String,
    from_phone_number: String,
    to_phone_number: String,
    status: String,
    delivery_status: bool,
    data: serde_json::Value,
}
