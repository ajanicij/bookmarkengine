use scraper::Html;

fn main() {
    let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

    let document = Html::parse_document(html);
    let html = document.html();
    println!("html: {}", html);
}
