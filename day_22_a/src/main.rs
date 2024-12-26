use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for line in input.lines() {
        let mut val = line.parse::<u64>().unwrap();

        // print!("{val}: ");

        for _ in 0..2000 {
            val = (val ^ (val * 64)) % 16777216;
            val = (val ^ (val / 32)) % 16777216;
            val = (val ^ (val * 2048)) % 16777216;
        }
        // println!("{val}");

        res += val;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
