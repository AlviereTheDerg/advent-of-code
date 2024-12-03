
use std::{env, fs};

mod day01;
mod day02;

fn grab_input(input_name: &str) -> String {
    fs::read_to_string(format!("./inputs/{input_name}.txt")).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).expect("Missing day!")
        .parse::<isize>().expect("Unable to parse day!");

    match day {
        1 => { day01::main(); },
        2 => { day02::main(); },
        _ => { println!("Day not recognized!"); },
    }
}
