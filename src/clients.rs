use serde::{Deserialize, Serialize};

use crate::{Client, error::{ClientError, ErrorResponse}};

pub struct Clients {}

pub struct ListAllParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ClientResponse {
    pub object: String,
    pub id: String,
    pub session_ids: Vec<String>,
    pub sessions: Vec<Session>,
    pub sign_in_id: String,
    pub sign_up_id: String,
    pub last_active_session_id: String,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Session {
    pub object: String,
    pub id: String,
    pub user_id: String,
    pub actor: serde_json::Value,
    pub status: String,
    pub last_active_organization_id: Option<String>,
    pub last_active_at: i64,
    pub expire_at: i64,
    pub abandon_at: i64,
    pub updated_at: i64,
    pub created_at: i64,
}



impl Clients { 
    pub async fn list_all(client: &Client, params: ListAllParams) -> Result<Vec<ClientResponse>, ClientError> {
        let url = format!("{}{}", client.base_url, "clients");

        // create query string
        let mut query = vec![];
        if let Some(limit) = params.limit {
            query.push(format!("limit={}", limit));
        }
        if let Some(offset) = params.offset {
            query.push(format!("offset={}", offset));
        }

        let res = client.http_client.get(&url).query(&query).send().await.map_err(ClientError::Reqwest)?;
        let status = res.status();

        if status.is_success() {
            let body = res.json::<Vec<ClientResponse>>().await.map_err(ClientError::Reqwest)?;
            Ok(body)
        } else {
            let err_body: ErrorResponse = res.json().await?;

            Err(ClientError::ErrorResponse(err_body))
        }

    }
}
