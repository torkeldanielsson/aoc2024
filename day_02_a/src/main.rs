use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut res = 0;

    'outer: for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut is_inc = false;
        let mut is_dec = false;

        for pair in numbers.windows(2) {
            if pair[0] == pair[1] {
                continue 'outer;
            }

            if pair[0] < pair[1] {
                is_inc = true;
            }

            if pair[0] > pair[1] {
                is_dec = true;
            }

            if is_inc && is_dec {
                continue 'outer;
            }

            if (pair[0] - pair[1]).abs() > 3 {
                continue 'outer;
            }
        }

        res += 1;
    }

    println!("res: {res}");

    Ok(())
}
