use std::env;
use regex::Regex;

const GB: u32 = 1024 * 1024 * 1024;
const MB: u32 = 1024 * 1024;
const KB: u32 = 1024;

fn capture_rx(text: &str, rx: Regex) -> Option<u32> {
    if let Some(caps) = rx.captures(text) {
        let num_str = &caps[1];
        if let Ok(num) = num_str.parse::<u32>() {
            return Some(num)
        }
    }
    None
}

fn capture_gb(text: &str) -> Option<u32> {
    let rx = Regex::new(r"(\d+)GB").unwrap();
    capture_rx(text, rx)
}

fn capture_mb(text: &str) -> Option<u32> {
    let rx = Regex::new(r"(\d+)MB").unwrap();
    capture_rx(text, rx)
}

fn capture_kb(text: &str) -> Option<u32> {
    let rx = Regex::new(r"(\d+)KB").unwrap();
    capture_rx(text, rx)
}

fn capture_num(text: &str) -> Option<u32> {
    if let Ok(num) = text.parse::<u32>() {
        return Some(num)
    }
    None
}

fn parse_size(text: &str) -> Result<u32, String> {
    if let Some(num) = capture_gb(text) {
        return Ok(num * GB);
    }
    if let Some(num) = capture_mb(text) {
        return Ok(num * MB);
    }
    if let Some(num) = capture_kb(text) {
        return Ok(num * KB);
    }
    if let Some(num) = capture_num(text) {
        return Ok(num);
    }
    Err(format!("Bad size: {}", text))
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        println!("{}", arg);
    }
    let size = &args[1];
    println!("size: {}", size);
    let res = parse_size(size);
    println!("res: {:?}", res);
}
