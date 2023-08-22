use jq_rs::{run, jq};
use serde_json::json;

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        let value = arg.parse::<serde_json::Value>().unwrap();
        println!("{}", run!(jq!( value => "." ).unwrap()));
    } else {
        println!("{}", run!(jq!(42 => ".").unwrap()));
    }
}
