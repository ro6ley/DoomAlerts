use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    let watchlist: String = env::var("WATCHLIST").expect("$WATCHLIST env var is not set");

    let interruptions: HashMap<u64, Vec<String>> =
        doom_alerts::tweets::fetch_tweets("KenyaPower_Care").await;

    for (id, interruption_texts) in &interruptions {
        let interruption_date: String =
            doom_alerts::interruption::extract_date(interruption_texts.clone());
        let affected: bool =
            doom_alerts::search::search(interruption_texts.clone(), watchlist.clone()).unwrap();

        if affected {
            let tweet_link: String = doom_alerts::tweets::build_tweet_link(*id, "KenyaPower_Care");

            // TODO: send email once
            match doom_alerts::notifications::send_email(interruption_date, tweet_link).await {
                Ok(msg) => println!("{msg}"),
                _ => println!("ERROR: Email not sent!"),
            };
        }
    }
}
