use jq_rs::{run, macros::jq};
use serde_json::json;

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        let value = json!(arg);
        println!("{}", run!(jq!( value | . )));
    } else {
        println!("{}", run!(jq!(42 | .)));
    }
}
