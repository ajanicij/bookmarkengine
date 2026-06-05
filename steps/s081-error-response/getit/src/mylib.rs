#[derive(Debug)]
pub enum MyLibError {
    InvalidName,
}

pub fn myfun(name: String, greeting: String) -> Result<String, MyLibError> {
    if name == "xyz" {
        return Err(MyLibError::InvalidName);
    }

    Ok(format!("{} {}", greeting, name))
}
