//! Structs and functions for working with interruptions
//!
//! In this module, you can find structs and methods to interact with interruption
//! metadata.
//!
//! ## Structs
//!
//! - `Interruption`: this struct represents the interruption information for a single area.
//!   The area may be a county, sub-county e.t.c depending on the KPLC notice.
//!
//! ## Functions
//!
//! - `parse_text` - uses regex to extract interruption information from the text extracted from notice images.
//! - `extract_date` - parses a vector of texts extracted from the interruption images and return the first date found

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter, Result};

lazy_static! {
    static ref INTERRUPTION_RE: Regex = Regex::new(r"(?mi)^(?P<region>[a-z\s]*\sregion)?\s*((parts\sof)?\b(?P<county>[a-z\s]*\scounty\b))?\s*(\barea:?)\s\b(?P<area>[a-z\s,\.]*)(\bdate:?)\s\b(?P<day>[a-z]*)\b\s*(?P<date>[\d\.]*)\b\s(\btime:?)\s\b(?P<start>[\d\.]*)\b\s\b(?P<start_period>[ap]\.m\.)\s*[-~—]\s*\b(?P<end>[\d\.]*)\s*(?P<end_period>[ap]\.m\.)\s*(?P<locations>[a-z0-9&,\s\.-]*)\n")
    .expect("Error compiling regex");
}

/// Represents the interruption details for an area.
///
/// ## Fields
///
///* `region` - e.g. Nyanza, Coast
///* `county` - e.g. Machako County, Mombasa County
///* `area` - e.g. Kitengela, Bamburi
///* `start_time`- e.g. 9.00 A.M.
///* `end_time` - e.g. 2.00 P.M.
///* `locations` - e.g. Parts of Eastern Bypass, Ruai Tuskys Supermarket, Triple O Hotel, Bakri Petrol Stn
///
#[derive(Debug, Default)]
pub struct Interruption {
    region: Option<String>,
    county: Option<String>,
    area: Option<String>,
    pub date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    locations: Option<String>,
}

impl Display for Interruption {
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

impl Interruption {
    /// This function returns a boolean indicating whether the provided location will be
    /// affected by this power supply interuption.
    pub fn affects_location(&self, location: &str) -> bool {
        if let Some(l) = &self.locations {
            return l.contains(location);
        }
        false
    }
}

/// Using regex, extract interruption information from the text extracted from image.
///
/// The extracted information makes up the fields on the `Interruption` struct.
pub fn parse_text(text: &str) -> Vec<Interruption> {
    let mut interruptions: Vec<Interruption> = Vec::new();

    for captures in INTERRUPTION_RE.captures_iter(text) {
        let mut interruption = Interruption::default();

        if let Some(n) = captures.name("region") {
            interruption.region = Some(n.as_str().to_string());
        }

        if let Some(n) = captures.name("county") {
            interruption.county = Some(n.as_str().to_string());
        }

        if let Some(n) = captures.name("area") {
            interruption.area = Some(n.as_str().to_string());
        }

        if let Some(n) = captures.name("day") {
            if let Some(d) = captures.name("date") {
                let d_str = format!("{} {}", n.as_str(), d.as_str());
                interruption.date = Some(d_str);
            } else {
                interruption.date = Some(n.as_str().to_string());
            }
        }

        if let Some(n) = captures.name("start") {
            if let Some(t) = captures.name("start_period") {
                let t_str = format!("{} {}", n.as_str(), t.as_str());
                interruption.start_time = Some(t_str);
            } else {
                interruption.start_time = Some(n.as_str().to_string());
            }
        }

        if let Some(n) = captures.name("end") {
            if let Some(t) = captures.name("end_period") {
                let t_str = format!("{} {}", n.as_str(), t.as_str());
                interruption.end_time = Some(t_str);
            } else {
                interruption.end_time = Some(n.as_str().to_string());
            }
        }

        if let Some(n) = captures.name("locations") {
            interruption.locations = Some(n.as_str().to_string());
        }

        interruptions.push(interruption);
    }

    interruptions
}

/// Parses a vector of texts extracted from the interruption images and return the first date found
pub fn extract_date(interruption_texts: Vec<String>) -> Option<String> {
    for interruption_text in interruption_texts {
        let interruptions = parse_text(&interruption_text);
        for interruption in interruptions {
            if let Some(date) = interruption.date {
                return Some(date);
            }
        }
    }
    None
}
