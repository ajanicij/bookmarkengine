use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    StartToken{name: String, attributes: HashMap<String, String>},
    EndToken{name: String},
    Text{text: String},
}
