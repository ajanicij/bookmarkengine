use chrono::{DateTime, Utc};
use std::error::Error;
use scraper::{Html, Selector};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};
use tantivy::directory::MmapDirectory;
use std::thread;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use std::path::PathBuf;
use regex::Regex;
use std::sync::mpsc;

use crate::item;

use crate::db;

const GB: usize = 1024 * 1024 * 1024;
const MB: usize = 1024 * 1024;
const KB: usize = 1024;

pub struct Indexer {
    writer: IndexWriter,
    schema: Schema,
    db: db::Db,
}

pub fn get_page(bookmark: &item::Item) -> Result<String, Box<dyn Error>> {
    // println!("Indexing bookmark {:?}", bookmark);
    let client = reqwest::blocking::Client::builder()
    .user_agent("CLIAgent-bookmark-search/0.1")
    .build()?;

    let url_string;
    // let description_string;
    match bookmark {
        item::Item::Bookmark{ description: _description_field, path: _path_field, href, last_modified: _ } => {
            url_string = href.to_string();
            // path_string = path_field.to_string();
            // description_string = description_field.to_string();
        },
        _ => return Err(Box::from(format!("Not a bookmark: {:?}", bookmark))),
    }

    let result = client.get(url_string).send()?;
    // println!("{:?}", result);
    let text = result.text()?;
    Ok(text)
}

#[derive(Debug, Clone)]
pub struct BookmarkMessage {
    pub bookmark: item::Item,
    pub text: String,
}

impl Indexer {
    pub fn new(index_path: &str, memory_budget: usize, db: db::Db) -> Result<Indexer, Box<dyn Error>> {
        let schema = create_schema();
        let directory = MmapDirectory::open(&index_path)?;
        let index = Index::open_or_create(directory, schema.clone())?;
        let index_writer: IndexWriter = index.writer(memory_budget)?;
        Ok(Indexer{
            writer: index_writer,
            schema,
            db
        })
    }

    pub fn write(&mut self, message: BookmarkMessage)  -> Result<(), Box<dyn Error>> {
        let url_string;
        let path_string;
        let description_string;
        let BookmarkMessage{bookmark, text} = message;
        let last_modified: DateTime<Utc>;

        match bookmark {
            item::Item::Bookmark{ description: description_field, path: path_field, href, last_modified: last_modified_tmp } => {
                url_string = href.to_string();
                path_string = path_field.to_string();
                description_string = description_field.to_string();
                last_modified = last_modified_tmp;
            },
            _ => return Err(Box::from(format!("Not a bookmark: {:?}", bookmark))),
        }

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
        let url_field = self.schema.get_field("url").unwrap();
        let body = self.schema.get_field("body").unwrap();
    
        let mut doc = TantivyDocument::default();
        doc.add_text(title_field, &title_string);
        doc.add_text(description_field, &description_string);
        doc.add_text(path_field, &path_string);
        doc.add_text(url_field, &url_string);
        doc.add_text(
            body,
            text
        );
        self.writer.add_document(doc)?;

        let bookmark = db::Bookmark {
            description: Some(description_string),
            path: path_string,
            href: url_string,
            last_modified: last_modified.timestamp(),
        };
        self.db.insert(&bookmark)?;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.commit()?;
        Ok(())
    }

    pub fn index(&mut self, bookmarks: Vec<item::Item>,
        commit_period: u32, threads: usize) -> Result<(), Box<dyn Error>> {

        let (tx, rx) = mpsc::channel::<BookmarkMessage>();

        // Count the bookmarks.
        let total_count = bookmarks.len();

        // Run the sending thread.
        let handle = thread::spawn(move || {

            let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads) // use one thread per work slice
            .build()
            .unwrap();
    
            // Run the sending functions in the thread pool.
            pool.install(|| {
                bookmarks
                .into_par_iter()
                .for_each_with(tx.clone(), |tx: &mut mpsc::Sender<BookmarkMessage>, bookmark| {
                    match bookmark {
                        item::Item::Bookmark{ description: _, path: _, href: _, last_modified: _, } => {
                            // println!("Fetching page: {:?}", bookmark);
                            let result = get_page(&bookmark);
                            let mut text = "".to_string();
                            if let Ok(result) = result {
                                text = result;
                            }
                            let message = BookmarkMessage {
                                bookmark,
                                text,
                            };
                            // println!("Sending result to the channel");
                            if let Err(err) = tx.send(message) {
                                eprintln!("Error sending bookmark: {}", err);
                            }
                        },
                        _ => {
                            let message = BookmarkMessage {
                                bookmark,
                                text: "".to_string(),
                            };
                            if let Err(err) = tx.send(message) {
                                eprintln!("Error sending bookmark: {}", err);
                            }
                        }
                    }
                    // TODO: do something with error result.
                });
            });
            drop(tx);
        });

        // Process bookmarks.
        let pb = ProgressBar::new(total_count as u64);

        pb.set_style(
            ProgressStyle::with_template("({pos}/{len}) [{elapsed_precise}] ETA {eta} {msg}")
                .unwrap()
        );

        let mut counter = 1;
    
        for message in rx {
            // println!("Got message (counter={})", counter);
            counter += 1;
            pb.set_message(format!("Indexing page (counter={}) {}", counter, message.bookmark.message()));
            pb.inc(1);
            if let Err(_) = self.write(message) {
                continue;
            }
            if commit_period > 0 && counter == commit_period {
                self.commit().expect("Failed to commit");
                counter = 0;
            }
        }
        self.commit().expect("Failed to commit");

        let _ = handle.join();

        Ok(())
    }
}

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("description", TEXT | STORED);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("url", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    schema
}

#[derive(Serialize, Deserialize)]
struct SearchResult {
    description: Vec<String>,
    path: Vec<String>,
    title: Vec<String>,
    url: Vec<String>,
}

impl SearchResult {
    fn show(&self) {
        println!(r#"title: {}
description: {}
path: {}
url: {}
"#, self.title[0], self.description[0], self.path[0], self.url[0]);
    }
}

pub fn search(index_path: &PathBuf, db: &mut db::Db, query_str: &str, num_results: u32) -> Result<(), Box<dyn Error>> {

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
    let mut count = 0;
    let mut ids = HashMap::<i64, ()>::new();
    for (_score, doc_address) in top_docs {
        count += 1;
        if num_results > 0 && count > num_results {
            break;
        }
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        let res_str = retrieved_doc.to_json(&schema);
        let res: SearchResult = serde_json::from_str(&res_str)?;

        if res.path.len() == 0 {
            continue;
        }
        if res.url.len() == 0 {
            continue;
        }

        // Check if we have already displayed this result.
        let bookmark = db::Bookmark{
            description: None, // ignored
            path: res.path[0].clone(),
            href: res.url[0].clone(),
            last_modified: 0, // ignored
        };
        if let Ok(ids_found) = db.search(&bookmark) {
            if ids_found.len() == 0 {
                // This should not happen, because if it does, it means that
                // we found the bookmark in the index, but not in the database.
                continue;
            }
            if ids.contains_key(&ids_found[0]) {
                continue; // This means that the found result is a duplicate, so we skip it.
            }
            ids.insert(ids_found[0], ());
        }

        // println!("TantivyDocument(to JSON): {}", res_str);
        res.show();
        // println!("{}\n", retrieved_doc.to_json(&schema));

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
