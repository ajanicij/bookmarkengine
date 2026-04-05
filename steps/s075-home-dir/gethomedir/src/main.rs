fn main() {
    println!("Hello, world!");
    let home_dir = std::env::home_dir().unwrap();
    println!("Home directory is {}", home_dir.display());
}
