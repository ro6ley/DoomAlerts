use egg_mode::{tweet::user_timeline, tweet::Timeline, user::UserID, Token};
use leptess::LepTess;
use std::borrow::Cow::Borrowed;


async fn fetch_image(url: &str) -> Option<String> {
    let img_bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    Some(extract_from_mem(&img_bytes))
}

fn extract_from_mem(img_buffer: &[u8]) -> String {
    let mut lt = LepTess::new(None, "eng").unwrap();
    lt.set_image_from_mem(img_buffer).unwrap();

    String::from(lt.get_utf8_text().unwrap())
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

                    let img_txt: String = fetch_image(&entity.media_url).await.unwrap();
                    println!("\nExtracted text from {}:\n\n{}", entity.media_url, img_txt);
                    println!("\n---------------------------------------------------------\n");
                }
            }
            None => {
                println!("Tweet has no media");
            }
        }
    }
}

// TODO: check for notices including places within my provided location
// TODO: API, Yew frontend
// TODO: email, sms notifs
// TODO: login via Twitter
// TODO: Favorite locations
