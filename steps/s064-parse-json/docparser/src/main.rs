use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Object {
    field1: String,
    field2: String,
}

impl Object {
    fn show(&self) {
        println!(r#"
            field1: {},
            field2: {}
        "#, self.field1, self.field2);
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let doc = r#"
      "field1": "value1",
      "field2": "value2"  
    "#;

    println!("doc={}", doc);

    let obj: Object = serde_json::from_str(doc)?;
    obj.show();

    Ok(())
}
