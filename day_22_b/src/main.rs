use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test1");

    for line in input.lines() {
        let mut val = line.parse::<i64>().unwrap();

        let mut prev_last_digit = val % 10;

        for _ in 0..10 {
            val = (val ^ (val * 64)) % 16777216;
            val = (val ^ (val / 32)) % 16777216;
            val = (val ^ (val * 2048)) % 16777216;

            let last_digit = val % 10;

            println!("{val}: {last_digit} ({})", last_digit - prev_last_digit);

            prev_last_digit = last_digit;
        }
    }

    // println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
