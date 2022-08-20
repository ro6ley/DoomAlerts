use std::borrow::Cow::Borrowed;
use std::env;

use egg_mode::{auth, tweet::user_timeline, tweet::Timeline, user::UserID, KeyPair, Token};

// TODO: return HashMap<K: Tweet, V: Vec<String>>
pub async fn fetch_tweets(username: &'static str) -> Vec<String> {
    println!("\nFetching {}'s latest tweets... ", username);

    let api_key: String = env::var("API_KEY").expect("$API_KEY env var is not set");
    let api_key_secret: String =
        env::var("API_KEY_SECRET").expect("$API_KEY_SECRET env var is not set");

    let con_token: KeyPair = KeyPair::new(api_key, api_key_secret);
    let token: Token = auth::bearer_token(&con_token).await.unwrap();

    let user_id: UserID = UserID::ScreenName(Borrowed(username));
    let timeline: Timeline = user_timeline(user_id, false, false, &token).with_page_size(200);

    let mut outage_texts: Vec<String> = Vec::new();

    let (_timeline, feed) = timeline.newer(None).await.unwrap();
    for tweet in &*feed {
        println!(
            "@{}: {}",
            tweet.user.as_ref().unwrap().screen_name,
            tweet.text
        );

        match &tweet.extended_entities {
            Some(extended_entities) => {
                for entity in &extended_entities.media {
                    println!("Media found. Extracting text...");

                    let img_text: String = crate::utils::fetch_image_from_url(&entity.media_url)
                        .await
                        .unwrap();
                    outage_texts.push(img_text);
                }
            }
            None => {
                println!("Tweet has no media");
            }
        }
    }
    outage_texts
}
