extern crate seven_segments;

use std::env;
use std::process;

use seven_segments::parse_number;

fn main() {
    match run() {
        Ok(number) => println!("{}", number),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<i32, &'static str> {
    let arg1 = match env::args().nth(1) {
        Some(arg) => arg,
        None => return Err("Command expects at least one argument"),
    };

    match parse_number(&arg1) {
        Ok(number) => Ok(number),
        Err(_) => Err("Command expects the argument to be a number"),
    }
}
