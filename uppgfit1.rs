use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let numbers: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let diff: u64 = if numbers[0] > numbers[1] {
            numbers[0] - numbers[1]
        } else {
            numbers[1] - numbers[0]
        };

        println!("{}", diff);
    }
}
