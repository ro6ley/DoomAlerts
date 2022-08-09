#![allow(dead_code)]

use egg_mode::{tweet::user_timeline, tweet::Timeline, user::UserID, Token};
use leptess::LepTess;
use regex::Regex;
use std::borrow::Cow::Borrowed;

#[derive(Debug, Default)]
struct Outage {
    region: Option<String>,
    county: Option<String>,
    area: Option<String>,
    date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    locations: Option<String>,
}

pub async fn fetch_tweets(token: Token, username: &'static str) {
    println!("\nFetching {}'s latest tweets... ", username);

    let user_id: UserID = UserID::ScreenName(Borrowed(username));
    // ? filter using keywords
    let timeline: Timeline = user_timeline(user_id, false, false, &token).with_page_size(200);

    let (_timeline, feed) = timeline.newer(None).await.unwrap();
    for tweet in &*feed {
        println!("\n=========================================================\n");
        println!(
            "@{}: {}",
            tweet.user.as_ref().unwrap().screen_name,
            tweet.text
        );

        match &tweet.extended_entities {
            Some(extended_entities) => {
                println!("\n---------------------------------------------------------\n");
                for entity in &extended_entities.media {
                    println!("{entity:?}");

                    let img_text: String = fetch_image(&entity.media_url).await.unwrap();
                    parse_text(&img_text);
                    println!("\n---------------------------------------------------------\n");
                }
            }
            None => {
                println!("Tweet has no media");
            }
        }
    }
}

async fn fetch_image(url: &str) -> Option<String> {
    let img_bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    Some(extract_from_mem(&img_bytes))
}

fn extract_from_mem(img_buffer: &[u8]) -> String {
    let mut lt = LepTess::new(None, "eng").unwrap();
    lt.set_image_from_mem(img_buffer).unwrap();

    String::from(lt.get_utf8_text().unwrap())
}

/// Using regex, extract outage information from the text extracted from image
fn parse_text(text: &str) {
    let re: Regex = Regex::new(r"(?mi)^(?P<region>[a-z\s]*\s*region\b)?\s*(\bparts\sof\s(?P<county>[a-z\s]*\scounty\b))?\s*(\barea:?)\s*\b(?P<area>[a-z\s,\.]*)(\bdate:?)\s*\b(?P<day>[a-z]*)\b\s*(?P<date>[\d\.]*)\b\s*(\btime:?)\s*\b(?P<start>[\d\.]*)\b\s*\b(?P<start_period>[ap]\.m\.)\s*[-—]*\s*\b(?P<end>[\d\.]*)\s*(?P<end_period>[ap]\.m\.)\s*\b(?P<locations>[a-z0-9&,\s\.]*)\n")
        .unwrap();

    for captures in re.captures_iter(text) {
        let mut outage = Outage::default();

        match captures.name("region") {
            Some(n) => {
                println!("REGION: {}", n.as_str());
                outage.region = Some(n.as_str().to_string());
            }
            _ => {
                println!("No region details found.");
            }
        }

        match captures.name("county") {
            Some(n) => {
                println!("COUNTY: {}", n.as_str());
                outage.county = Some(n.as_str().to_string());
            }
            _ => {
                println!("No county details found.");
            }
        }

        match captures.name("area") {
            Some(n) => {
                println!("AREA: {}", n.as_str());
                outage.area = Some(n.as_str().to_string());
            }
            _ => {
                println!("No area details found.");
            }
        }

        match captures.name("day") {
            Some(n) => {
                if let Some(d) = captures.name("date") {
                    let d_str = format!("{} {}", n.as_str(), d.as_str());
                    println!("DAY: {}", &d_str);
                    outage.date = Some(d_str);
                } else {
                    println!("DAY: {}", n.as_str());
                    outage.date = Some(n.as_str().to_string());
                }
            }
            _ => {
                println!("No date details found.");
            }
        }

        match captures.name("start") {
            Some(n) => {
                if let Some(t) = captures.name("start_period") {
                    let t_str = format!("{} {}", n.as_str(), t.as_str());
                    println!("START: {}", &t_str);
                    outage.start_time = Some(t_str);
                } else {
                    println!("START: {}", n.as_str());
                    outage.start_time = Some(n.as_str().to_string());
                }
            }
            _ => {
                println!("No start time details found.");
            }
        }

        match captures.name("end") {
            Some(n) => {
                if let Some(t) = captures.name("end_period") {
                    let t_str = format!("{} {}", n.as_str(), t.as_str());
                    println!("END: {}", &t_str);
                    outage.end_time = Some(t_str);
                } else {
                    println!("END: {}", n.as_str());
                    outage.end_time = Some(n.as_str().to_string());
                }
            }
            _ => {
                println!("No end time details found.");
            }
        }

        match captures.name("locations") {
            Some(n) => {
                println!("LOCATIONS: {}", n.as_str());
                outage.locations = Some(n.as_str().to_string());
            }
            _ => {
                println!("No location details found.");
            }
        }
    }
}

// TODO: check for notices including places within my provided location
// TODO: API, Yew frontend
// TODO: email, sms notifs, telegram
// TODO: login via Twitter
// TODO: Favorite locations
