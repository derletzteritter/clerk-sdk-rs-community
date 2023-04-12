use serde::{Deserialize, Serialize};

use crate::Client;

pub struct ActorTokens {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActorTokenResponse {
    pub object: String,
    pub id: String,
    pub status: ActorTokenStatus,
    pub user_id: String,
    pub actor: serde_json::Value,
    pub token: Option<String>,
    pub url: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActorTokenParams {
    pub user_id: String,
    pub actor: serde_json::Value,
    pub expires_in_seconds: i32,
    pub session_max_duration_in_seconds: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ActorTokenStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "revoked")]
    Revoked,
}

impl ActorTokens {
    pub async fn create_actor_token(
        client: &Client,
        params: &ActorTokenParams,
    ) -> Result<ActorTokenResponse, reqwest::Error> {
        let url = format!("{}{}", client.base_url, "actor-tokens");

        match client.http_client.post(&url).json(&params).send().await {
            Ok(response) => match response.json::<ActorTokenResponse>().await {
                Ok(token) => Ok(token),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn revoke_actor_token(
        client: &Client,
        token_id: String,
    ) -> Result<ActorTokenResponse, reqwest::Error> {
        let url = format!("{}{}{}", client.base_url, "actor-tokens/", token_id);

        match client.http_client.delete(&url).send().await {
            Ok(response) => match response.json::<ActorTokenResponse>().await {
                Ok(token) => Ok(token),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}
