pub mod parse;

use jq_rs::jq;
use serde_json::json;

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        println!("{}", jq!(json!(arg) => "."));
    } else {
        println!("{}", jq!(42 => "."));
    }
}
