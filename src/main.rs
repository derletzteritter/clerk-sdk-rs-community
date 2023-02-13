use clerk_rs::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("".to_string());

    let users = client.read_user("".to_string()).await.unwrap();

    println!("{:?}", users);
}
