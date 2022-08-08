#![allow(dead_code)]

use egg_mode::{tweet::user_timeline, tweet::Timeline, user::UserID, Token};
use leptess::LepTess;
use regex::{Match, Regex};
use std::borrow::Cow::Borrowed;

struct Outage<'a> {
    region: &'a str,
    county: &'a str,
    date: &'a str,
    time: &'a str,
    locations: &'a str,
}

// TODO: implement IncludesLocation for Outage
trait IncludesLocation {
    fn includes_location(&self, location: &str);
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
    let re: Regex = Regex::new(r"(?mi)^(?P<region>[a-z\s]*\s*region\b)?\s*(\bparts\sof\s(?P<county>[a-z\s]*\scounty\b))?\s*(\barea:?)\s*\b(?P<area>[a-z\s,\.]*)(\bdate:?)\s*\b(?P<day>[a-z]*)\b\s*(?P<date>[\d\.]*)\b\s*(\btime:?)\s*\b(?P<start>[\d\.]*)\b\s*\b(?P<start_period>[ap]\.m\.)\s*[-—]*\s*\b(?P<end>[\d\.]*)\s*(?P<end_period>[ap]\.m\.)\s*\b(?P<locations>[a-z0-9&,\s\.]*)\n").unwrap();

    for captures in re.captures_iter(text) {
        let region: Option<Match> = captures.name("region");
        let county: Option<Match> = captures.name("county");
        let area: Option<Match> = captures.name("area");
        let day: Option<Match> = captures.name("day");
        let date: Option<Match> = captures.name("date");

        let start_time: Option<Match> = captures.name("start");
        let start_time_period: Option<Match> = captures.name("start_period");
        let end_time: Option<Match> = captures.name("end");
        let end_time_period: Option<Match> = captures.name("end_period");
        let locations: Option<Match> = captures.name("locations");

        match region {
            Some(n) => {
                println!("REGION: {}", n.as_str());
            }
            _ => {
                println!("No region details found.");
            }
        }

        match county {
            Some(n) => {
                println!("COUNTY: {}", n.as_str());
            }
            _ => {
                println!("No county details found.");
            }
        }

        match area {
            Some(n) => {
                println!("AREA: {}", n.as_str());
            }
            _ => {
                println!("No area details found.");
            }
        }

        match day {
            Some(n) => {
                if let Some(d) = date {
                    println!("DAY: {} - {}", n.as_str(), d.as_str());
                } else {
                    println!("DAY: {}", n.as_str());
                }
            }
            _ => {
                println!("No area details found.");
            }
        }

        match start_time {
            Some(n) => {
                if let Some(t) = start_time_period {
                    println!("START: {} {}", n.as_str(), t.as_str());
                } else {
                    println!("START: {}", n.as_str());
                }
            }
            _ => {
                println!("No area details found.");
            }
        }

        match end_time {
            Some(n) => {
                if let Some(t) = end_time_period {
                    println!("END: {} {}", n.as_str(), t.as_str());
                } else {
                    println!("END: {}", n.as_str());
                }
            }
            _ => {
                println!("No area details found.");
            }
        }

        match locations {
            Some(n) => {
                println!("LOCATIONS: {}", n.as_str());
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
