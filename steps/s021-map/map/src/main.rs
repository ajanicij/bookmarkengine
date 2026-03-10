use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert("alpha", "first");
    map.insert("beta", "second");

    if let Some(value) = map.get("alpha") {
        println!("Found: {}", value);
    }

    println!("Iterating");
    for (key, value) in &map {
        println!("{} -> {}", key, value);
    }
    println!("");

    let value = map.get("beta").unwrap();
    println!("Found: {}", value);
}
