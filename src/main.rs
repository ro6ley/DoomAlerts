use log::{error, info};
use std::{collections::HashMap, env};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> ! {
    env_logger::init();

    let watchlist: String = env::var("WATCHLIST").expect("$WATCHLIST env var is not set");
    let interval: u64 = env::var("INTERVAL")
        .expect("$INTERVAL env var is not set")
        .parse::<u64>()
        .expect("$INTERVAL env var value is NOT a number!");

    let mut interval_timer = tokio::time::interval(Duration::from_secs(interval));
    loop {
        interval_timer.tick().await;

        info!("Fetching tweets every {interval} seconds ... ");

        let interruptions: HashMap<u64, Vec<String>> =
            doom_alerts::tweets::fetch_tweets("KenyaPower_Care").await;

        for (id, interruption_texts) in &interruptions {
            let interruption_date: String =
                doom_alerts::interruption::extract_date(interruption_texts.clone())
                    .unwrap_or_else(|| "N/A".to_string());
            let affected: bool =
                doom_alerts::search::search(interruption_texts.clone(), watchlist.clone())
                    .unwrap_or(false);

            if affected {
                info!("One or more areas in watchlist will be affected by a scheduled power supply interruption. Sending interruption information via email... ");
                let tweet_link: String =
                    doom_alerts::tweets::build_tweet_link(*id, "KenyaPower_Care");

                // TODO: send email once
                match doom_alerts::notifications::send_email(interruption_date, tweet_link).await {
                    Ok(msg) => info!("{msg}"),
                    _ => error!("Email was not sent."),
                };
            }
        }
    }
}
