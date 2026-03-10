#[derive(Debug)]
pub enum Item {
    Folder{name: String},
    Bookmark{ description: String, path: String, href: String, last_modified: u32, },
    Unfolder, // go up to parent folder
}
