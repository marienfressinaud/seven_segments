extern crate seven_segments;

use std::env;
use std::process;

use seven_segments::{parse_number, render_digits, split_into_digits};

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

    let number = match parse_number(&arg1) {
        Ok(number) => number,
        Err(_) => return Err("Command expects the argument to be a number"),
    };

    let digits = split_into_digits(number);
    render_digits(&digits, output);
    Ok(())
}
