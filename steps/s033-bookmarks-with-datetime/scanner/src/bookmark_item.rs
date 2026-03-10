#[derive(Debug)]
pub enum Item {
    Folder{name: String},
    Bookmark{ description: String, path: String, href: String, add_date: u32, },
    Unfolder, // go up to parent folder
}
