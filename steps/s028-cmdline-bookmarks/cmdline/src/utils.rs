use std::error::Error;

use reqwest;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};
use tantivy::directory::MmapDirectory;

use scraper::{Html, Selector};

pub fn write_index(index_path: &str, url: &str) -> Result<(), Box<dyn Error>> {
    println!("Indexing {} to index path {}", url, index_path);
    let client = reqwest::blocking::Client::builder()
        .user_agent("CLIAgent-bookmark-search/0.1")
        .build()?;
    let result = client.get(url).send()?;
    // println!("{:?}", result);
    let text = result.text()?;
    // println!("text: {}", text);

    let document = Html::parse_document(&text);
    let selector = Selector::parse("head title").unwrap();
    let mut title_string = "".to_string();
    for element in document.select(&selector) {
        // println!("Title: {}", element.text().collect::<String>());
        title_string = title_string + &element.text().collect::<String>();
    }

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    // let index = Index::create_in_dir(&index_path, schema.clone())?;
    let directory = MmapDirectory::open(&index_path)?;
    let index = Index::open_or_create(directory, schema.clone())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut wsws_doc = TantivyDocument::default();
    wsws_doc.add_text(title, &title_string);
    wsws_doc.add_text(
        body,
        text
    );
    index_writer.add_document(wsws_doc)?;
    index_writer.commit()?;

    Ok(())
}

pub fn search(index_path: &str, query_str: &str) -> Result<(), Box<dyn Error>> {

    println!("Searching...");

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    // let index = Index::create_in_dir(&index_path, schema.clone())?;
    let directory = MmapDirectory::open(index_path)?;
    // let index = Index::create(directory, schema.clone(), IndexSettings::default())?;
    let index = Index::open_or_create(directory, schema.clone())?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query(query_str)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }

    Ok(())
}
