//! Search mod

use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::*, Index, IndexWriter, ReloadPolicy,
};

/// Perform search
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

/// Build search index
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
