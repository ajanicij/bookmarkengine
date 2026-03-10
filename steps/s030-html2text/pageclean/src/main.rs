use std::fs;
use html2text;

fn main() {
    let file_path = "./micro-saas.html";
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file");
    let bytes = contents.as_bytes();
    if let Ok(result) = html2text::from_read(bytes, 80) {
        println!("Result: {}", result);
    }
}
