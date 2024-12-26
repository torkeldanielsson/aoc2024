use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test1");

    let mut last_digits = Vec::new();

    for line in input.lines() {
        let mut val = line.parse::<i64>().unwrap();

        let mut val_last_digits = Vec::new();
        val_last_digits.push((val % 10) as i8);

        for _ in 0..2000 {
            val = (val ^ (val * 64)) % 16777216;
            val = (val ^ (val / 32)) % 16777216;
            val = (val ^ (val * 2048)) % 16777216;

            let last_digit = (val % 10) as i8;
            val_last_digits.push(last_digit);
        }

        last_digits.push(val_last_digits);
    }

    let mut diffs = Vec::new();

    for ld in &last_digits {
        let mut diff = Vec::new();
        for pair in ld.windows(2) {
            let d = pair[1] - pair[0];
            diff.push(d);
        }
        diffs.push(diff);
    }

    for i in 0..10 {
        println!("{} {}", last_digits[0][i + 1], diffs[0][i]);
    }

    // println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
