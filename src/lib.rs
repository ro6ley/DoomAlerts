use std::borrow::Cow::Borrowed;
use std::env;

use egg_mode::{auth, tweet::user_timeline, tweet::Timeline, user::UserID, KeyPair, Token};
use leptess::LepTess;
use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::*, Index, IndexWriter, ReloadPolicy,
};

pub mod notifs;
pub mod outages;

pub async fn fetch_tweets(username: &'static str) -> Vec<String> {
    println!("\nFetching {}'s latest tweets... ", username);

    let api_key: String = env::var("API_KEY").expect("$API_KEY env var is not set");
    let api_key_secret: String =
        env::var("API_KEY_SECRET").expect("$API_KEY_SECRET env var is not set");

    let con_token: KeyPair = KeyPair::new(api_key, api_key_secret);
    let token: Token = auth::bearer_token(&con_token).await.unwrap();

    let user_id: UserID = UserID::ScreenName(Borrowed(username));
    let timeline: Timeline = user_timeline(user_id, false, false, &token).with_page_size(200);

    let mut outage_texts: Vec<String> = Vec::new();

    let (_timeline, feed) = timeline.newer(None).await.unwrap();
    for tweet in &*feed {
        println!(
            "@{}: {}",
            tweet.user.as_ref().unwrap().screen_name,
            tweet.text
        );

        match &tweet.extended_entities {
            Some(extended_entities) => {
                for entity in &extended_entities.media {
                    println!("Media found. Extracting text...");

                    let img_text: String = fetch_image_from_url(&entity.media_url).await.unwrap();
                    outage_texts.push(img_text);
                }
            }
            None => {
                println!("Tweet has no media");
            }
        }
    }
    outage_texts
}

async fn fetch_image_from_url(url: &str) -> Option<String> {
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

pub fn search(full_texts: Vec<String>, locations: String) -> tantivy::Result<bool> {
    // abc,xyz -> "abc" OR "xyz"
    let formatted_locations: String = locations
        .split(',')
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<String>>()
        .join(" OR ");

    println!("Searching for {} ...", formatted_locations);

    let (index, schema) = build_index(full_texts)?;
    let text = schema.get_field("text").unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![text]);

    let query = query_parser.parse_query(&formatted_locations)?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    Ok(!top_docs.is_empty())
}

fn build_index(full_texts: Vec<String>) -> tantivy::Result<(Index, Schema)> {
    let mut schema_builder: SchemaBuilder = Schema::builder();
    schema_builder.add_text_field("text", TEXT | STORED);

    let schema: Schema = schema_builder.build();
    let text: Field = schema.get_field("text").unwrap();

    // build index
    let index: Index = Index::create_in_ram(schema.clone());
    let mut index_writer: IndexWriter = index.writer(10_000_000)?;

    // populate the index
    full_texts.iter().for_each(|t: &String| {
        index_writer
            .add_document(doc!(
                text => &**t
            ))
            .unwrap();
    });
    index_writer.commit()?;

    Ok((index, schema))
}
