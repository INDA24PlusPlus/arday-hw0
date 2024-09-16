use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    let vector: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let x: u64 = vector[0];
    let y: u64 = vector[1];

    for row in 0..x {
        let mut rowStr = String::new();

        for col in 0..y {
            let mut result: u64 = min(min(row, x - row - 1), min(col, y - col - 1));
            result += 1;

            if (result <= 9) {
                rowStr.push_str(&result.to_string());
            } else {
                rowStr.push_str(".");
            }
        }

        println!("{}", rowStr);
    }
}

fn min(x: u64, y: u64) -> u64 {
    if x > y {
        return y;
    }

    x
}
