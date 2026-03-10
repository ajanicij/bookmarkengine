use std::fs;
use scraper::{Html, Selector, ElementRef};

fn main() {
    let file_path = "./input.html";
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file");
    // println!("Contents:\n{}", contents);

    // test01(&contents);
    // test02(&contents);
    test03(&contents);
}

fn test03(contents: &str) {
    let document = Html::parse_document(&contents);
    let selector = Selector::parse("body > DL").unwrap();
    let selection = document.select(&selector);
    for el in selection {
        println!("el: {}", el.value().name());
        parse_dl("", el);
    }
}

fn parse_dl(path: &str, el: ElementRef) {
    for inner_el in el.child_elements() {
        let name = inner_el.value().name();
        println!("name: {}", name);
    }
}

fn test02(contents: &str) {
    let document = Html::parse_document(&contents);
    let root_el = document.root_element();
    for el in root_el.child_elements() {
        let v = el.value();
        println!("name: {}", v.name());
    }
}

fn test01(contents: &str) {
    let document = Html::parse_document(&contents);
    let selector = Selector::parse("DL").unwrap();

    let el = document.select(&selector).next().unwrap();

    println!("html: {}", el.html());

    // println!("next: {}", el.text().collect::<String>());

    for element in document.select(&selector) {
        let text = element.text();
        for t in text {
            println!("text: {}", t);
        }
    }

/*
    for element in document.select(&selector) {
        println!("{}", element.text().collect::<String>());
    }
*/
}
