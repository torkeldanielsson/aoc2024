use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut numbers: Vec<u128> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap() as u128)
        .collect();

    for _ in 0..25 {
        let mut new_numbers = Vec::new();
        for n in numbers {
            if n == 0 {
                new_numbers.push(1);
            } else if n >= 10 {
                let s = n.to_string();
                if s.len() % 2 == 0 {
                    let (s1, s2) = s.split_at(s.len() / 2);
                    new_numbers.push(s1.parse::<i32>().unwrap() as u128);
                    new_numbers.push(s2.parse::<i32>().unwrap() as u128);
                    continue;
                } else {
                    new_numbers.push(n * 2024);
                }
            } else {
                new_numbers.push(n * 2024);
            }
        }
        numbers = new_numbers;
    }

    println!("res: {}, {} us", numbers.len(), t.elapsed().as_micros());

    Ok(())
}
