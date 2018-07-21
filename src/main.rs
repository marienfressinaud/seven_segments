extern crate seven_segments;

use std::env;
use std::process;

use seven_segments::{parse_digits, render_digits};

fn main() {
    let mut output = String::new();
    match run(&mut output) {
        Ok(_) => print!("{}", output),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn run(output: &mut String) -> Result<(), &'static str> {
    let arg1 = match env::args().nth(1) {
        Some(arg) => arg,
        None => return Err("Command expects at least one argument"),
    };

    let digits = match parse_digits(&arg1) {
        Ok(digits) => digits,
        Err(_) => return Err("Command expects the argument to be a number"),
    };
    render_digits(&digits, output);
    Ok(())
}
