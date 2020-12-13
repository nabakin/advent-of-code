use std::io::{self, Write};

use day_1;
 
fn main() {
    println!("Advent of Code 2020");

    loop {
        println!("Which day would you like to run?");
        println!("1) Day 1");
        println!("Press Enter to run all");
        print!("> ");

        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        match buffer.get(0..1).unwrap() {
            "\n" | "\r" => {
                day_1::run("2020/day-1/input.txt");
            },
            "1" => day_1::run("2020/day-1/input.txt"),
            _ => continue
        }

        break;
    }
}
