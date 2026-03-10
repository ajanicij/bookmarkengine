use chrono::{Utc, DateTime};

#[derive(Debug)]
pub enum Item {
    Folder{name: String},
    Bookmark{ description: String, path: String, href: String, last_modified: DateTime<Utc>, },
    Unfolder, // go up to parent folder
}
