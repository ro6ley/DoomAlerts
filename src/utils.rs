//! Utility functions
//!
//! # Functions
//!
//! - `fetch_image_from_url` - fetch an image from the provided URL and return the text extracted from it
//! - `extract_from_mem` - extract text from an image in memory
//! - `extract_from_path`- extract text from an image located at the provided path

use log::info;

use leptess::LepTess;

/// Fetches an image from the provided URL and returns the text extracted from it
pub async fn fetch_image_from_url(url: &str) -> Option<String> {
    info!("Downloading image from {url}");
    let img_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    extract_from_mem(&img_bytes)
}

/// Extract text from an image in memory
fn extract_from_mem(img_buffer: &[u8]) -> Option<String> {
    let mut lt: LepTess = LepTess::new(None, "eng").expect("Error creating LepTess wrapper.");
    lt.set_image_from_mem(img_buffer)
        .expect("Error setting image to use for OCR");

    lt.get_utf8_text().ok()
}

/// Extract text from an image located at the provided path
pub fn extract_from_path(location: &str) -> Option<String> {
    let mut lt: LepTess = LepTess::new(None, "eng").expect("Error creating LepTess wrapper.");
    lt.set_image(location)
        .expect("Error setting image to use for OCR");

    lt.get_utf8_text().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_extracted() {
        let img_text = extract_from_path("./images/test_2.png").unwrap();
        assert!(img_text.starts_with("AREA: WHOLE OF UTAWALAFEEDER"))
    }

    #[test]
    fn test_location_in_text() {
        let img_text = extract_from_path("./images/test_2.png").unwrap();
        let first_interruption = &crate::interruption::parse_text(&img_text)[0];

        assert!(first_interruption.affects_location("Parts of Eastern Bypass"));
    }

    #[test]
    fn test_regex_extraction() {
        let img_text = extract_from_path("./images/test.png").unwrap();
        let interruptions = crate::interruption::parse_text(&img_text);
        assert_eq!(interruptions.len(), 3);
    }
}
