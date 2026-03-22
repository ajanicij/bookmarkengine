// use rusqlite::{Connection, Result, params};
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;

mod bookmarkdb;

fn s(text: &str) -> String {
    text.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut db = bookmarkdb::BookmarkDb::new("bookmarks.db")?;

    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time before UNIX_EPOCH");
    let now_epoch: i64 = duration_since_epoch.as_secs() as i64;

    db.create_db()?;
    let bm = bookmarkdb::Bookmark{
        description: Some(s("What is Rust?")),
        path: s("/my bookmarks"),
        href: s("https://www.howtogeek.com/what-is-the-rust-programming-language-and-how-to-get-started/"),
        last_modified: now_epoch,
    };

    let _ = db.delete(&bm)?;

    let mut exists = db.exists(&bm)?;
    println!("Before inserting: exists: {}", exists);

    let id = db.insert(&bm)?;
    println!("Inserted record with id = {}", id);
    println!("Searching");
    let ids = db.search(&bm)?;
    for id in ids {
        println!("Found id = {}", id);
    }

    exists = db.exists(&bm)?;
    println!("After inserting: exists: {}", exists);

    let res = db.insert(&bm);
    assert!(res.is_err());

    println!("Calling load_all");
    let rows = db.load_all()?;
    for row in rows {
        println!("Found row {:?}", row);
    }
    Ok(())
}
