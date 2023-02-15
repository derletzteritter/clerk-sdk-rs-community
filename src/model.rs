use serde::{Deserialize, Serialize};

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
