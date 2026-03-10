use std::fs;
use std::env;
use html2md;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let file_path = "./micro-saas.html";
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file");
    let md = html2md::rewrite_html(&contents, false);
    println!("md: {}", md);
}
