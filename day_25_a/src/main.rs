use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for x in input.split("\n\n") {
        let mut pattern: u64 = 0;
        let mut bit_position = 0;

        for c in x.chars() {
            if c != '\n' {
                if c == '#' {
                    pattern |= 1 << bit_position;
                }
                bit_position += 1;
            }
        }

        if x.starts_with("#") {
            locks.push(pattern);
        } else {
            keys.push(pattern);
        }
    }

    let mut res = 0;

    for key in &keys {
        for lock in &locks {
            if key & lock == 0 {
                res += 1;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
