use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        *left.entry(parts[0].parse::<i32>().unwrap()).or_insert(0) += 1;
        *right.entry(parts[1].parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    let mut res = 0;

    for l in left {
        res += l.0 * l.1 * *right.entry(l.0).or_insert(0);
    }

    println!("res: {res}");

    Ok(())
}
