use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for line in input.lines() {
        let mut val = line.parse::<u64>().unwrap();

        for _ in 0..2000 {
            val = (val ^ (val << 6)) & 0xFFFFFF;
            val = (val ^ (val >> 5)) & 0xFFFFFF;
            val = (val ^ (val << 11)) & 0xFFFFFF;
        }

        res += val;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
