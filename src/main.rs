use clerk_rs::Client;
use clerk_rs;
use clerk_rs::users::User;

#[tokio::main]
async fn main() {
    let client = Client::new("".to_string());

    match User::list_all_users(&client).await {
        Ok(users) => {
            println!("users: {:?}", users);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
