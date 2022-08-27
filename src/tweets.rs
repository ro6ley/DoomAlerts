//! Functions for working with tweets.
//!
//! ## Functions
//!
//! - `fetch_tweets` - function fetches a users tweets and returns a hashmap of tweets
//! and the interruption information extracted from the images attached on them, if any.
//! - `build_tweet_link` - returns a tweet's URL from a tweet ID and a username

use log::info;
use std::{borrow::Cow::Borrowed, collections::HashMap, env};

use egg_mode::{auth, tweet::user_timeline, tweet::Timeline, user::UserID, KeyPair, Token};

/// This function fetches a users tweets and filters those with images attached.
///
/// It returns a hashmap with the tweet ID as the key
/// and the value is a vector containing the texts extracted from the images attached to the tweet
pub async fn fetch_tweets(username: &'static str) -> HashMap<u64, Vec<String>> {
    info!("Fetching {}'s latest tweets... ", username);

    let api_key: String = env::var("API_KEY").expect("$API_KEY env var is not set");
    let api_key_secret: String =
        env::var("API_KEY_SECRET").expect("$API_KEY_SECRET env var is not set");

    let con_token: KeyPair = KeyPair::new(api_key, api_key_secret);
    let token: Token = auth::bearer_token(&con_token)
        .await
        .expect("Error creating token");

    let user_id: UserID = UserID::ScreenName(Borrowed(username));
    let timeline: Timeline = user_timeline(user_id, false, false, &token).with_page_size(200);

    let mut interruptions: HashMap<u64, Vec<String>> = HashMap::new();

    let (_timeline, feed) = timeline
        .newer(None)
        .await
        .expect("Error getting latest tweets");
    for tweet in &*feed {
        info!(
            "@{}: {}",
            tweet
                .user
                .as_ref()
                .expect("Error getting username")
                .screen_name,
            tweet.text
        );

        match &tweet.extended_entities {
            Some(extended_entities) => {
                let mut interruption_texts: Vec<String> = Vec::new();

                for entity in &extended_entities.media {
                    info!("Media found. Extracting text...");

                    if let Some(img_text) =
                        crate::utils::fetch_image_from_url(&entity.media_url).await
                    {
                        interruption_texts.push(img_text);
                    }
                }

                if !interruption_texts.is_empty() {
                    interruptions.insert(tweet.id, interruption_texts);
                }
            }
            None => {
                info!("Tweet has no media");
            }
        }
    }
    interruptions
}

/// This function returns a tweet's URL from a tweet ID and a username.
///
/// The URL is embedded within the outgoing notifications.
pub fn build_tweet_link(id: u64, username: &'static str) -> String {
    format!("twitter.com/{username}/status/{id}")
}
