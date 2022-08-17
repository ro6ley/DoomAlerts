use std::env;

mod utils;

#[tokio::main]
async fn main() {
    utils::extract_image_text().await;
    let locations: String = env::var("LOCATIONS").expect("$LOCATIONS env var is not set");

    let outage_texts: Vec<String> = doom_alerts::fetch_tweets("KenyaPower_care").await;

    let affected: bool = doom_alerts::search(outage_texts.clone(), locations).unwrap();
    if affected {
        println!("One or more areas in watchlist will be affected by a scheduled power supply interruption. Sending outage information via email... ");

        // TODO: email html template
        let outage_text: String =
            outage_texts.join("\n ---------------------------------------- \n");

        match doom_alerts::notifs::send_email(outage_text).await {
            Ok(msg) => println!("{msg}"),
            _ => println!("ERROR: Email not sent!"),
        };
    }
}
