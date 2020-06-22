#[macro_use]
extern crate map_struct;

use serde::Serialize;
use serde_json::to_string;

#[map(Option, #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Serialize)]
pub struct Foo {
    #[serde(rename = "foofoo")]
    foo: u32,
    bar: u32
}

#[derive(Debug, Serialize)]
pub struct Bar {
    bar: u64
}

fn main() {
    let foo: Foo = Foo {
        foo: None,
        bar: Some(20)
    };

    println!("{}", to_string(&foo).unwrap());
}
