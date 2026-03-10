extern crate html5ever;

use std::cell::Cell;
use std::cell::RefCell;

use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::{CharacterTokens, EndTag, NullCharacterToken, StartTag, TagToken};
use html5ever::tokenizer::{
    ParseError, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

use crate::bookmark_token;
use crate::bookmark_item;
use crate::utils;

use crate::bookmark_token::*;

use std::borrow::Borrow;
use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub struct BookmarkScanner {
    pub bookmarks: Vec<bookmark_item::Item>,
    tokens: Vec<bookmark_token::BookmarkToken>, // vector of tokens from one line from a bookmark file
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

    pub fn scan(&mut self, line: &str) {
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
            self.bookmarks.push(item);
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

        if let BookmarkToken::Text{text} = &self.tokens[self.index] {
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
        if let BookmarkToken::StartToken{name: _, attributes} = &self.tokens[self.index] {
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
        if let BookmarkToken::Text{text: _} = &self.tokens[self.index] {
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
        if let BookmarkToken::StartToken{name, attributes: _} = &self.tokens[self.index] {
            if name.eq_ignore_ascii_case(tag) {
                return Some(());
            }
        }
        None
    }

    fn check_end_tag(&mut self, tag: &str) -> Option<()> {
        self.check_not_end()?;
        if let BookmarkToken::EndToken{name} = &self.tokens[self.index] {
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
        if let BookmarkToken::StartToken{name: _, attributes} = &self.tokens[self.index] {
            let last_modified = if let Some(last_modified) = attributes.get("last_modified") {
                last_modified.clone()
            } else {
                "".to_string()
            };
            self.last_modified = utils::date_time_from_str(&last_modified).ok()?;
        }
        self.next()?;
        if let BookmarkToken::Text{text} = &self.tokens[self.index] {
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
    tokens: RefCell<Vec<bookmark_token::BookmarkToken>>,
}

impl TokenPrinter {
    fn is_char(&self, is_char: bool) {
        match (self.in_char_run.get(), is_char) {
            (false, true) => {
                // print!("CHAR : \"");
                self.text.borrow_mut().clear();
            }
            (true, false) => {
                let token = bookmark_token::BookmarkToken::Text{text: self.text.borrow().clone()};
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
                        let token = bookmark_token::BookmarkToken::StartToken{name: tag.name.as_ref().to_string(), attributes: attributes};
                        self.tokens.borrow_mut().push(token);
                    }
                    EndTag => {
                        let token = bookmark_token::BookmarkToken::EndToken{name: tag.name.as_ref().to_string()};
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
