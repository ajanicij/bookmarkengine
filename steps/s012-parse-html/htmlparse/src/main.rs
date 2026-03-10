use std::fs;
use scraper::{Html, Selector};

fn main() {
    let file_path = "./index.html";
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file");
    println!("Contents:\n{}", contents);

    let document = Html::parse_document(&contents);
    let selector = Selector::parse("body h2").unwrap();
    for element in document.select(&selector) {
        println!("{}", element.text().collect::<String>());
    }
}
