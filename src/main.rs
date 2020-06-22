use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};

#[derive(Debug, Deserialize, Serialize)]
enum Foo {
    #[serde(rename = "FOOFOO")]
    A,
    B,
    C
}

#[derive(Debug, Deserialize, Serialize)]
struct FooContainer {
    contents: Foo
}

fn main() {
    let foo = Foo::A;
    let container = json!({
        "contents": foo
    });

    println!("{:?}", from_str::<FooContainer>(&to_string(&container).unwrap()).unwrap());
}
