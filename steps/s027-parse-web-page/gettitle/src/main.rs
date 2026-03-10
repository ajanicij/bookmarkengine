use reqwest;
use scraper::{Html, Selector};

fn main() {
    let result = reqwest::blocking::get("https://www.wsws.org/en/articles/2022/10/13/vkxz-o13.html")
        .expect("Failed to fetch");
    println!("{:?}", result);
    let text = result.text().expect("Failed to get contents");
    // println!("text: {}", text);

    let document = Html::parse_document(&text);
    let selector = Selector::parse("head title").unwrap();
    for element in document.select(&selector) {
        println!("Title: {}", element.text().collect::<String>());
    }

}
