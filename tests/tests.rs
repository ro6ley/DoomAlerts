#[cfg(test)]
mod tests {
    const TEST_IMAGE: &str = "./tests/images/test_2.png";

    #[test]
    fn test_text_extracted() {
        let img_text = doom_alerts::utils::extract_from_path(TEST_IMAGE);
        assert!(img_text.starts_with("AREA: WHOLE OF UTAWALAFEEDER"))
    }

    #[test]
    fn test_regex_extraction() {
        let img_text = doom_alerts::utils::extract_from_path(TEST_IMAGE);
        let interruptions = doom_alerts::interruption::parse_text(&img_text);
        assert_eq!(interruptions.len(), 3);
    }

    #[test]
    fn test_location_in_text() {
        let img_text = doom_alerts::utils::extract_from_path(TEST_IMAGE);
        let first_interruption = &doom_alerts::interruption::parse_text(&img_text)[0];

        assert!(first_interruption.affects_location("Parts of Eastern Bypass"));
    }

    #[test]
    fn test_search() {
        let image_paths: Vec<&str> = vec!["./tests/images/test_6.png", "./tests/images/test_2.png"];
        let locations: String = String::from("Nyangweso,Bogani,Mwalimu Motors");

        let mut interruption_texts: Vec<String> = Vec::new();

        for p in image_paths {
            interruption_texts.push(doom_alerts::utils::extract_from_path(p));
        }

        let affected = doom_alerts::search::search(interruption_texts, locations).unwrap();
        assert_eq!(affected, true);
    }

    #[test]
    fn test_failing_search() {
        let image_paths: Vec<&str> = vec!["./tests/images/test_6.png", "./tests/images/test_2.png"];
        let locations: String = String::from("Turkana,Marsabit County");

        let mut interruption_texts: Vec<String> = Vec::new();

        for p in image_paths {
            interruption_texts.push(doom_alerts::utils::extract_from_path(p));
        }

        let affected = doom_alerts::search::search(interruption_texts, locations).unwrap();
        assert_eq!(affected, false);
    }
}
