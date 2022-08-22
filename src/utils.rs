//! Utility functions
//!
//! # Functions
//!
//! - `fetch_image_from_url` - fetch an image from the provided URL and return the text extracted from it
//! - `extract_from_mem` - extract text from an image in memory
//! - `extract_from_path`- extract text from an image located at the provided path

use leptess::LepTess;

/// Fetches an image from the provided URL and returns the text extracted from it
pub async fn fetch_image_from_url(url: &str) -> Option<String> {
    let img_bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    Some(extract_from_mem(&img_bytes))
}

/// Extract text from an image in memory
fn extract_from_mem(img_buffer: &[u8]) -> String {
    let mut lt: LepTess = LepTess::new(None, "eng").unwrap();
    lt.set_image_from_mem(img_buffer).unwrap();

    lt.get_utf8_text().unwrap()
}

/// Extract text from an image located at the provided path
pub fn extract_from_path(location: &str) -> String {
    let mut lt: LepTess = LepTess::new(None, "eng").unwrap();
    lt.set_image(location).unwrap();

    lt.get_utf8_text().unwrap()
}
