use std::fs;

fn create_if_not_file(entry: &str) -> Result<&str, &str> {
    if let Ok(metadata) = fs::metadata(entry) {
        let file_type = metadata.file_type();
        if file_type.is_file() {
            return Err("regular file");
        } else if file_type.is_dir() {
            return Ok("existent directory");
        }
    } else {
        // Try to create directory.
        match fs::create_dir(entry) {
            Ok(()) => return Ok("created directory"),
            Err(err) => {
                eprintln!("Error: {}", err);
                return Err("couldn't create directory");
            }
        }
    }
    Err("unknown")
}

fn main() {
    println!("Hello, world!");
    match create_if_not_file("./index") {
        Ok(msg) => println!("Success: {}", msg),
        Err(err) => eprintln!("Error: {}", err),
    }
}
