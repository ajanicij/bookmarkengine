use crate::bookmark_token::*;

pub enum Item {
    Folder{name: String},
    Bookmark{description: String, path: String},
    None,
}

fn skip_empty_text(tokens: &Vec<BookmarkToken>, i: &mut usize) {
    if let BookmarkToken::Text{ref text} = tokens[*i] {
        *i += 1;
    }
}

fn parse_line(tokens: &Vec<BookmarkToken>) -> Item {
    let mut i: usize = 0;
    loop {
        skip_empty_text(tokens, &mut i);
        if i >= tokens.len() {
            break;
        }
        // TODO: check if current token is DL or DT.
    }
    Item::None
}