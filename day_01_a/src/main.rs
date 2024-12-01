use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut res = 0;

    for i in 0..left.len() {
        res += (left[i] - right[i]).abs();
    }

    println!("res: {res}");

    Ok(())
}
