fn main() {
    println!("Hello, world!");
    let text = "123".to_string();
    let x: u32;
    x = text.parse().unwrap();
    println!("x={}", x);
}
