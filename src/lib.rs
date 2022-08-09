#![allow(dead_code)]

use egg_mode::{tweet::user_timeline, tweet::Timeline, user::UserID, Token};
use leptess::LepTess;
use regex::Regex;
use std::borrow::Cow::Borrowed;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct Outage {
    region: Option<String>,
    county: Option<String>,
    area: Option<String>,
    date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    locations: Option<String>,
}

impl Display for Outage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "REGION: {}",
            self.region
                .as_ref()
                .unwrap_or(&"No region details found.".to_string())
        )?;
        writeln!(
            f,
            "COUNTY: {}",
            self.county
                .as_ref()
                .unwrap_or(&"No county details found.".to_string())
        )?;
        writeln!(
            f,
            "AREA: {}",
            self.area
                .as_ref()
                .unwrap_or(&"No area details found.".to_string())
        )?;
        writeln!(
            f,
            "DATE: {} ({} - {})",
            self.date
                .as_ref()
                .unwrap_or(&"No date details found.".to_string()),
            self.start_time.as_ref().unwrap_or(&"N/A".to_string()),
            self.end_time.as_ref().unwrap_or(&"N/A".to_string())
        )?;
        writeln!(
            f,
            "LOCATIONS: {}",
            self.locations
                .as_ref()
                .unwrap_or(&"No affected locations details found.".to_string())
        )?;
        writeln!(
            f,
            "---------------------------------------------------------"
        )
    }
}

impl Outage {
    pub fn includes_location(&self, location: &str) -> bool {
        if let Some(l) = &self.locations {
            return l.contains(location);
        }
        false
    }
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

    lt.get_utf8_text().unwrap()
}

pub fn extract_from_path(location: &str) -> String {
    let mut lt: LepTess = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(location).unwrap();

    lt.get_utf8_text().unwrap()
}

/// Using regex, extract outage information from the text extracted from image
pub fn parse_text(text: &str) -> Vec<Outage> {
    let re: Regex = Regex::new(r"(?mi)^(?P<region>[a-z\s]*\s*region\b)?\s*(\bparts\sof\s(?P<county>[a-z\s]*\scounty\b))?\s*(\barea:?)\s*\b(?P<area>[a-z\s,\.]*)(\bdate:?)\s*\b(?P<day>[a-z]*)\b\s*(?P<date>[\d\.]*)\b\s*(\btime:?)\s*\b(?P<start>[\d\.]*)\b\s*\b(?P<start_period>[ap]\.m\.)\s*[-—]*\s*\b(?P<end>[\d\.]*)\s*(?P<end_period>[ap]\.m\.)\s*\b(?P<locations>[a-z0-9&,\s\.]*)\n")
        .unwrap();
    let mut outages: Vec<Outage> = Vec::new();

    for captures in re.captures_iter(text) {
        let mut outage = Outage::default();

        if let Some(n) = captures.name("region") {
            outage.region = Some(n.as_str().to_string());
        }

        if let Some(n) = captures.name("county") {
            outage.county = Some(n.as_str().to_string());
        }

        if let Some(n) = captures.name("area") {
            outage.area = Some(n.as_str().to_string());
        }

        if let Some(n) = captures.name("day") {
            if let Some(d) = captures.name("date") {
                let d_str = format!("{} {}", n.as_str(), d.as_str());
                outage.date = Some(d_str);
            } else {
                outage.date = Some(n.as_str().to_string());
            }
        }

        if let Some(n) = captures.name("start") {
            if let Some(t) = captures.name("start_period") {
                let t_str = format!("{} {}", n.as_str(), t.as_str());
                outage.start_time = Some(t_str);
            } else {
                outage.start_time = Some(n.as_str().to_string());
            }
        }

        if let Some(n) = captures.name("end") {
            if let Some(t) = captures.name("end_period") {
                let t_str = format!("{} {}", n.as_str(), t.as_str());
                outage.end_time = Some(t_str);
            } else {
                outage.end_time = Some(n.as_str().to_string());
            }
        }

        if let Some(n) = captures.name("locations") {
            outage.locations = Some(n.as_str().to_string());
        }

        outages.push(outage);
    }

    outages
}

// TODO: check for notices including places within my provided location
// TODO: API, Yew frontend
// TODO: email, sms notifs, telegram
// TODO: login via Twitter
// TODO: Favorite locations
