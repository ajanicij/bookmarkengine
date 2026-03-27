extern crate html5ever;

use std::cell::Cell;
use std::cell::RefCell;

use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::{CharacterTokens, EndTag, NullCharacterToken, StartTag, TagToken};
use html5ever::tokenizer::{
    ParseError, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

use crate::token;
use crate::bookmark_item;
use crate::utils;

use std::borrow::Borrow;
use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::bookmark_item::Item;
use crate::utils::days_from;

pub struct BookmarkScanner {
    pub bookmarks: Vec<bookmark_item::Item>,
    tokens: Vec<token::Token>, // vector of tokens from one line from a bookmark file
    index: usize, // index of current token we are processing
    last_modified: DateTime<Utc>,
    folder: String,
    path: Vec<String>,
    href: String,
}

impl BookmarkScanner {
    pub fn new() -> Self {
        BookmarkScanner {
            bookmarks: Vec::new(),
            tokens: Vec::new(),
            index: 0,
            last_modified: Utc::now(),
            folder: "".to_string(),
            path: Vec::new(),
            href: "".to_string(),
        }
    }

    pub fn scan(&mut self, line: &str, max_age: Option<u32>) {
        let sink = TokenPrinter {
            in_char_run: Cell::new(false),
            text: RefCell::new("".to_string()),
            tokens: RefCell::new(Vec::new()),
        };
    
        let contents = line;
    
        let chunk: ByteTendril = Tendril::from_slice(contents.as_bytes());
    
        let input = BufferQueue::default();
        input.push_back(chunk.try_reinterpret().unwrap());
    
        let tok = Tokenizer::new(
            sink,
            TokenizerOpts {
                profile: true,
                ..Default::default()
            },
        );
        let _ = tok.feed(&input);
    
        assert!(input.is_empty());
        tok.sink.is_char(false);
        self.tokens = tok.sink.tokens.borrow().clone();
        if let Some(item) = self.process_tokens() {
            if not_older(&item, max_age) {
                self.bookmarks.push(item);
            }
        }
    }

    fn process_tokens(&mut self) -> Option<bookmark_item::Item> {
        if let Some(item) = self.try_parse_dl_p() {
            return Some(item);
        }
        if let Some(_) = self.try_parse_dt_h3() {
            // No bookmark item; fields updated.
            return None;
        }
        if let Some(item) = self.try_parse_dt_a() {
            // No bookmark item; fields updated.
            return Some(item);
        }
        // TODO: parse </DL><p> (move up one folder).
        if let Some(item) = self.try_parse_end_dl() {
            return Some(item);
        }
        None
    }

    pub fn try_parse_end_dl(&mut self) -> Option<bookmark_item::Item> {
        self.index = 0;
        self.skip_text()?;
        self.check_end_tag("dl")?;
        let item = bookmark_item::Item::Unfolder;
        Some(item)
    }

    pub fn try_parse_dl_p(&mut self) -> Option<bookmark_item::Item> {
        self.index = 0;
        self.skip_text()?;
        self.check_start_tag("dl")?;
        self.next()?;
        self.check_start_tag("p")?;
        let folder = if self.folder == "" {
            "".to_string()
        } else {
            let result = self.folder.clone();
            self.folder = "".to_string();
            result
        };
        self.path.push(folder.clone());
        let item = bookmark_item::Item::Folder { name: folder };
        Some(item)
    }

    pub fn try_parse_dt_h3(&mut self) -> Option<()> {
        self.index = 0;
        self.skip_text()?;
        self.check_start_tag("dt")?;
        self.next()?;
        self.parse_h3()?;
        Some(())
    }

    pub fn try_parse_dt_a(&mut self) -> Option<bookmark_item::Item> {
        self.index = 0;
        self.skip_text()?;
        self.check_start_tag("dt")?;
        self.next()?;
        self.parse_a()?;
        self.next()?;

        if let token::Token::Text{text} = &self.tokens[self.index] {
            return Some(bookmark_item::Item::Bookmark {
                description: text.clone(),
                path: self.path.join("/"),
                href: self.href.clone(),
                last_modified: self.last_modified,
            });
        }
        None
    }

    fn parse_a(&mut self) -> Option<()> {
        self.check_not_end()?;
        self.check_start_tag("a")?;
        if let token::Token::StartToken{name: _, attributes} = &self.tokens[self.index] {
            let href = if let Some(href) = attributes.get("href") {
                href.clone()
            } else {
                "".to_string()
            };
            self.href = href;

            let last_modified = if let Some(last_modified) = attributes.get("last_modified") {
                last_modified.clone()
            } else {
                "".to_string()
            };
            self.last_modified = utils::date_time_from_str(&last_modified).ok()?;

            return Some(());
        }
        None
    }

    fn skip_text(&mut self) -> Option<()> {
        self.check_not_end()?;
        if let token::Token::Text{text: _} = &self.tokens[self.index] {
            self.next()?;
            return Some(());
        }
        Some(())
    }

    fn check_not_end(&mut self) -> Option<()> {
        if self.index >= self.tokens.len() {
            return None;
        }
        Some(())
    }

    fn check_start_tag(&mut self, tag: &str) -> Option<()> {
        self.check_not_end()?;
        if let token::Token::StartToken{name, attributes: _} = &self.tokens[self.index] {
            if name.eq_ignore_ascii_case(tag) {
                return Some(());
            }
        }
        None
    }

    fn check_end_tag(&mut self, tag: &str) -> Option<()> {
        self.check_not_end()?;
        if let token::Token::EndToken{name} = &self.tokens[self.index] {
            if name.eq_ignore_ascii_case(tag) {
                self.path.pop();
                return Some(());
            }
        }
        None
    }

    fn parse_h3(&mut self) -> Option<()> {
        self.check_not_end()?;
        self.check_start_tag("h3")?;
        if let token::Token::StartToken{name: _, attributes} = &self.tokens[self.index] {
            let last_modified = if let Some(last_modified) = attributes.get("last_modified") {
                last_modified.clone()
            } else {
                "".to_string()
            };
            self.last_modified = utils::date_time_from_str(&last_modified).ok()?;
        }
        self.next()?;
        if let token::Token::Text{text} = &self.tokens[self.index] {
            self.folder = text.clone();
            return Some(());
        }
        None
    }

    fn next(&mut self) -> Option<()> {
        self.index += 1;
        self.check_not_end()
    }
}

#[derive(Clone)]
struct TokenPrinter {
    in_char_run: Cell<bool>,
    text: RefCell<String>,
    tokens: RefCell<Vec<token::Token>>,
}

impl TokenPrinter {
    fn is_char(&self, is_char: bool) {
        match (self.in_char_run.get(), is_char) {
            (false, true) => {
                // print!("CHAR : \"");
                self.text.borrow_mut().clear();
            }
            (true, false) => {
                let token = token::Token::Text{text: self.text.borrow().clone()};
                self.tokens.borrow_mut().push(token);

            }
            _ => (),
        }
        self.in_char_run.set(is_char);
    }

    fn do_char(&self, c: char) {
        self.is_char(true);
        // print!("{}", c.escape_default().collect::<String>());
        self.text.borrow_mut().push(c);
    }
}

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            CharacterTokens(b) => {
                for c in b.chars() {
                    self.do_char(c);
                }
            },
            NullCharacterToken => self.do_char('\0'),
            TagToken(tag) => {
                self.is_char(false);
                // This is not proper HTML serialization, of course.
                match tag.kind {
                    StartTag => {
                        let mut attributes = HashMap::new();
                        // for attr in &tag.attrs {
                        for attr in tag.attrs.iter() {
                            let value: &[u8] = attr.value.borrow();
                            let value_str = match std::str::from_utf8(value) {
                                Ok(s) => s,
                                Err(_) => "<invalid URL>",
                            };
                            let key = format!("{}", attr.name.local).to_string();
                            let value = value_str.to_string();
                            attributes.insert(key, value);
                        }
                        let token = token::Token::StartToken{name: tag.name.as_ref().to_string(), attributes: attributes};
                        self.tokens.borrow_mut().push(token);
                    }
                    EndTag => {
                        let token = token::Token::EndToken{name: tag.name.as_ref().to_string()};
                        self.tokens.borrow_mut().push(token);
                    }
                }
            },
            ParseError(_err) => {
                self.is_char(false);
                // println!("ERROR: {err}");
            },
            _ => {
                self.is_char(false);
                // println!("OTHER: {token:?}");
            },
        }
        TokenSinkResult::Continue
    }
}

/// not_older returns false ONLY if:
///   - item is a bookmark and
///   - max_age is not None and
///   - the age of the item is greater than max_age
fn not_older(item: &Item, max_age: Option<u32>) -> bool {
    match *item {
        Item::Bookmark{
            description: ref _description,
            path: ref _path,
            href: ref _href,
            last_modified} => {
                if let Some(max_age_num) = max_age {
                    if (max_age_num as i64) < days_from(last_modified) {
                        return false;
                    }
                }
            },
        _ => (),
    }
    true
}
