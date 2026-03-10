extern crate html5ever;

use std::cell::Cell;
use std::cell::RefCell;

use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::{CharacterTokens, EndTag, NullCharacterToken, StartTag, TagToken};
use html5ever::tokenizer::{
    ParseError, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

#[derive(Clone)]
struct TokenPrinter {
    in_char_run: Cell<bool>,
    text: RefCell<String>,
}

impl TokenPrinter {
    fn is_char(&self, is_char: bool) {
        match (self.in_char_run.get(), is_char) {
            (false, true) => {
                // print!("CHAR : \"");
                self.text.borrow_mut().clear();
            }
            (true, false) => {
                // println!("\"");
                println!("text: `{}'", self.text.borrow());
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
                        // print!("TAG  : <\x1b[32m{}\x1b[0m", tag.name);
                        println!("StartTag: {}", tag.name);
                    }
                    EndTag => {
                        // print!("TAG  : <\x1b[31m/{}\x1b[0m", tag.name);
                        println!("EndTag: {}", tag.name);
                    }
                }
                for attr in tag.attrs.iter() {
                    print!(
                        " \x1b[36m{}\x1b[0m='\x1b[34m{}\x1b[0m'",
                        attr.name.local, attr.value
                    );
                }
                if tag.self_closing {
                    print!(" \x1b[31m/\x1b[0m");
                }
                // println!(">");
            },
            ParseError(err) => {
                self.is_char(false);
                println!("ERROR: {err}");
            },
            _ => {
                self.is_char(false);
                println!("OTHER: {token:?}");
            },
        }
        TokenSinkResult::Continue
    }
}

/// In this example we implement the TokenSink trait in such a way that each token is printed.
/// If a there's an error while processing a token it is printed as well.
pub fn scan(line: &str) {
    let sink = TokenPrinter {
        in_char_run: Cell::new(false),
        text: RefCell::new("".to_string()),
    };

    let contents = line;

    // Read HTML from standard input
    // let mut chunk = ByteTendril::new();
    // chunk.from_slice(contents);
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
    // tok.end();
    tok.sink.is_char(false);
}
