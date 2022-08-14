use std::env;

use egg_mode::{auth, KeyPair, Token};

#[tokio::main]
async fn main() {
    let api_key: String = env::var("API_KEY").expect("$API_KEY env var is not set");
    let api_key_secret: String =
        env::var("API_KEY_SECRET").expect("$API_KEY_SECRET env var is not set");
    let locations: String = env::var("LOCATIONS").expect("$LOCATIONS env var is not set");

    let con_token: KeyPair = KeyPair::new(api_key, api_key_secret);
    let token: Token = auth::bearer_token(&con_token).await.unwrap();

    let outage_texts: Vec<String> = doom_alerts::fetch_tweets(token, "KenyaPower_care").await;

    // TODO: Send notification if affected == true
    let _affected: bool = doom_alerts::search(outage_texts, locations).unwrap();
}
