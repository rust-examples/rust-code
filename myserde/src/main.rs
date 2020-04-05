use serde::{Deserialize, Serialize};
use serde_json;

// You'll need the `Deserialize` trait on your struct so serde can deserialize it.
#[derive(Serialize, Deserialize)]
struct MyStruct {
    somestring: String,
    someint: u64,
}

fn main() {
    // Simple deserialize

    // Note: use r#"string"# for raw strings with double-quotes
    let incoming = r#"
    {
      "somestring": "Hello",
      "someint": 4
    }
    "#;

    // The type of the variable is explicitly given because serde doesn't know by itself what type to use.
    let result: MyStruct = serde_json::from_str(incoming).unwrap();

    // Use the resulting structure.
    println!("{}", result.somestring);
    println!("{}", result.someint);

    // Simple serialize

    let data = MyStruct {
        somestring: "mystring".to_string(),
        someint: 42,
    };

    println!("{}", serde_json::to_string(&data).unwrap());
}
