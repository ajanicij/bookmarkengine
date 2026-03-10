use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let d = "./index";
    if let Ok(exists) = fs::exists(d) {
        if exists {
            println!("{} exists", d);
        } else {
            println!("{} doesn't exist", d);
        }
    } else {
        println!("We don't know if {} exists", d);
    }

    // Check if d is a regular file.
    let metadata = fs::metadata(d)?;
    let file_type = metadata.file_type();
    if file_type.is_file() {
        println!("{} is a regular file", d);
    } else if file_type.is_dir() {
        println!("{} is a directory", d);
    } else {
        println!("{} is something else", d);
    }

    Ok(())
}
