use std::io::{self, stdout, Read};

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("Error");
    io::stdin().read_line(&mut input2).expect("Error");

    let x: u64 = input1.trim().parse().expect("Error");

    let mut nums: Vec<u64> = input2
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let numbers = if x % 2 == 0 { x / 2 } else { (x + 1) / 2 };

    nums.sort();

    let mut result: u64 = 0;

    for i in (0..x).rev() {
        result += nums[i as usize];

        if (x - i == numbers) {
            break;
        }
    }

    println!("{:?}", result);
}
