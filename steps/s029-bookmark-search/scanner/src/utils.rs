use std::error::Error;

use crate::bookmark_item::*;
use scraper::{Html, Selector};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};
use tantivy::directory::MmapDirectory;

pub fn write_index(index_path: &str, bookmark: &Item) -> Result<(), Box<dyn Error>> {
    // Err(Box::from("bang!"))
    println!("Indexing bookmark {:?} to index path {}", bookmark, index_path);
    let client = reqwest::blocking::Client::builder()
        .user_agent("CLIAgent-bookmark-search/0.1")
        .build()?;

    let url_string;
    let path_string;
    let description_string;
    match bookmark {
        Item::Bookmark{ description: description_field, path: path_field, href } => {
            url_string = href.to_string();
            path_string = path_field.to_string();
            description_string = description_field.to_string();
        },
        _ => return Err(Box::from(format!("Not a bookmark: {:?}", bookmark))),
    }

    let result = client.get(url_string).send()?;
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
    schema_builder.add_text_field("description", TEXT | STORED);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    // let index = Index::create_in_dir(&index_path, schema.clone())?;
    let directory = MmapDirectory::open(&index_path)?;
    let index = Index::open_or_create(directory, schema.clone())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let title_field = schema.get_field("title").unwrap();
    let description_field = schema.get_field("description").unwrap();
    let path_field = schema.get_field("path").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut doc = TantivyDocument::default();
    doc.add_text(title_field, &title_string);
    doc.add_text(description_field, &description_string);
    doc.add_text(path_field, &path_string);
    doc.add_text(
        body,
        text
    );
    index_writer.add_document(doc)?;
    index_writer.commit()?;
    Ok(())
}

pub fn search(index_path: &str, query_str: &str) -> Result<(), Box<dyn Error>> {

    println!("Searching...");

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("description", TEXT | STORED);
    schema_builder.add_text_field("path", TEXT | STORED);
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

    let body_field = schema.get_field("body").unwrap();
    let title_field = schema.get_field("title").unwrap();
    let description_field = schema.get_field("description").unwrap();
    let path_field = schema.get_field("path").unwrap();
    
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title_field, body_field, description_field, path_field]);
    let query = query_parser.parse_query(query_str)?;
    // TODO: make limit configurable.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(20))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}\n", retrieved_doc.to_json(&schema));

        let explanation = query.explain(&searcher, doc_address)?;
        println!(" -- explanation: {}", explanation.to_pretty_json());
    }

    Ok(())
}
