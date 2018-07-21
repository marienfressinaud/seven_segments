extern crate seven_segments;

use std::env;
use std::process;

use seven_segments::{parse_digits, render_digits};

fn main() {
    match run() {
        Ok(output) => print!("{}", output),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<String, &'static str> {
    let arg1 = match env::args().nth(1) {
        Some(arg) => arg,
        None => return Err("Command expects at least one argument"),
    };

    match parse_digits(&arg1) {
        Ok(digits) => Ok(render_digits(digits)),
        Err(_) => Err("Command expects the argument to be a number"),
    }
}
