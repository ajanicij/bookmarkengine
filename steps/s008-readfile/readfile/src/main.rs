use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file");
    println!("Contents:\n{}", contents);
}
