

fn main() {
    println!("Hello, world!");
    let mut stack = Vec::new();
    stack.push("John".to_string());
    stack.push("George".to_string());
    let path = stack.join("/");
    println!("path (join of stack): {}", path);
    let x = stack.pop();
    println!("{:?}", x);
    let x = stack.pop();
    println!("{:?}", x);
    let x = stack.pop();
    println!("{:?}", x);
    println!("stack is empty: {}", stack.is_empty());
}
