use clerk_rs::Client;
use clerk_rs::model;

#[tokio::main]
async fn main() {
    let client = Client::new("".to_string());

    let message = model::SMSMessage{
        message: "Hello from Clerk!".to_string(),
        phone_number_id: "phone_number_id".to_string(),
    };

    let response = client.create_sms(message).await.unwrap();

    println!("{:?}", response);
}
