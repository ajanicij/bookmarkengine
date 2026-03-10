use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum BookmarkToken {
    StartToken{name: String, attributes: HashMap<String, String>},
    EndToken{name: String},
    Text{text: String},
}
