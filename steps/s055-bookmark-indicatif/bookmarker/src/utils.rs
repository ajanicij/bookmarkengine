use chrono::{DateTime, Utc};
use std::error::Error;
use crate::bookmark_item::*;
use scraper::{Html, Selector};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};
use tantivy::directory::MmapDirectory;

use std::path::PathBuf;
use regex::Regex;

const GB: usize = 1024 * 1024 * 1024;
const MB: usize = 1024 * 1024;
const KB: usize = 1024;

pub struct Indexer {
    writer: IndexWriter,
    schema: Schema,
}

impl Indexer {
    pub fn new(index_path: &str, memory_budget: usize) -> Result<Indexer, Box<dyn Error>> {
        let schema = create_schema();
        let directory = MmapDirectory::open(&index_path)?;
        let index = Index::open_or_create(directory, schema.clone())?;
        let index_writer: IndexWriter = index.writer(memory_budget)?;
        Ok(Indexer{
            writer: index_writer,
            schema
        })
    }

    pub fn write(&self, bookmark: &Item)  -> Result<(), Box<dyn Error>> {
        // println!("Indexing bookmark {:?}", bookmark);
        let client = reqwest::blocking::Client::builder()
            .user_agent("CLIAgent-bookmark-search/0.1")
            .build()?;
    
        let url_string;
        let path_string;
        let description_string;
        match bookmark {
            Item::Bookmark{ description: description_field, path: path_field, href, last_modified: _ } => {
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

        let title_field = self.schema.get_field("title").unwrap();
        let description_field = self.schema.get_field("description").unwrap();
        let path_field = self.schema.get_field("path").unwrap();
        let body = self.schema.get_field("body").unwrap();
    
        let mut doc = TantivyDocument::default();
        doc.add_text(title_field, &title_string);
        doc.add_text(description_field, &description_string);
        doc.add_text(path_field, &path_string);
        doc.add_text(
            body,
            text
        );
        self.writer.add_document(doc)?;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.commit()?;
        Ok(())
    }
}

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("description", TEXT | STORED);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    schema
}

pub fn search(index_path: &PathBuf, query_str: &str) -> Result<(), Box<dyn Error>> {

    println!("Searching...");

    let schema = create_schema();
    let directory = MmapDirectory::open(index_path)?;
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

        // let explanation = query.explain(&searcher, doc_address)?;
        // println!(" -- explanation: {}", explanation.to_pretty_json());
    }

    Ok(())
}

pub fn date_time_from_str(ts_str: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let ts: i64 = ts_str.parse::<i64>()?;
    let dt = DateTime::from_timestamp(ts, 0).ok_or(format!("bad timestamp: {}", ts_str))?;
    Ok(dt)
}

pub fn days_from(dt: DateTime<Utc>) -> i64 {
    let now = Utc::now();
    let duration = now - dt;
    let days = duration.num_days();
    days
}

pub fn days_from_str(ts_str: &str) -> Result<i64, Box<dyn Error>> {
    let dt = date_time_from_str(ts_str)?;
    Ok(days_from(dt))
}

fn capture_rx(text: &str, rx: Regex) -> Option<usize> {
    if let Some(caps) = rx.captures(text) {
        let num_str = &caps[1];
        if let Ok(num) = num_str.parse::<usize>() {
            return Some(num)
        }
    }
    None
}

fn capture_gb(text: &str) -> Option<usize> {
    let rx = Regex::new(r"(\d+)GB").unwrap();
    capture_rx(text, rx)
}

fn capture_mb(text: &str) -> Option<usize> {
    let rx = Regex::new(r"(\d+)MB").unwrap();
    capture_rx(text, rx)
}

fn capture_kb(text: &str) -> Option<usize> {
    let rx = Regex::new(r"(\d+)KB").unwrap();
    capture_rx(text, rx)
}

fn capture_num(text: &str) -> Option<usize> {
    if let Ok(num) = text.parse::<usize>() {
        return Some(num)
    }
    None
}

pub fn parse_size(text: &str) -> Result<usize, String> {
    if let Some(num) = capture_gb(text) {
        return Ok(num * GB);
    }
    if let Some(num) = capture_mb(text) {
        return Ok(num * MB);
    }
    if let Some(num) = capture_kb(text) {
        return Ok(num * KB);
    }
    if let Some(num) = capture_num(text) {
        return Ok(num);
    }
    Err(format!("Bad size: {}", text))
}
