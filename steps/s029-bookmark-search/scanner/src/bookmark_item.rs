#[derive(Debug)]
pub enum Item {
    Folder{name: String},
    Bookmark{ description: String, path: String, href: String },
    Unfolder, // go up to parent folder
}
