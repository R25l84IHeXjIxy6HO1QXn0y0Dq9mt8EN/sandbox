use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_string};

#[derive(Deserialize, Serialize)]
struct Foo {
    a: u64
}

fn main() {
    let f: Foo = from_value(json!({
        "a": 10,
        "b": 20
    })).unwrap();
    println!("Value: {}", to_string(&f).unwrap())
}
