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
