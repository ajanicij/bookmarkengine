use reqwest;

fn main() {
    let result = reqwest::blocking::get("https://www.wsws.org/en/articles/2022/10/13/vkxz-o13.html")
        .expect("Failed to fetch");
    println!("{:?}", result);
    let text = result.text().expect("Failed to get contents");
    println!("text: {}", text);
}
