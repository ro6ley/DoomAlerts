use std::env;

#[tokio::main]
async fn main() {
    let locations: String = env::var("LOCATIONS").expect("$LOCATIONS env var is not set");

    let outage_texts: Vec<String> = doom_alerts::tweets::fetch_tweets("KenyaPower_care").await;

    let outages_date: String = doom_alerts::outages::extract_date(outage_texts.clone());
    let affected: bool = doom_alerts::search::search(outage_texts.clone(), locations).unwrap();

    if affected {
        println!("One or more areas in watchlist will be affected by a scheduled power supply interruption. Sending outage information via email... ");

        // TODO: email html template
        let outage_text: String =
            outage_texts.join("\n ---------------------------------------- \n");

        match doom_alerts::notifications::send_email(outage_text, outages_date).await {
            Ok(msg) => println!("{msg}"),
            _ => println!("ERROR: Email not sent!"),
        };
    }
}
