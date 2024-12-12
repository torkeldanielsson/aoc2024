use std::{collections::HashMap, error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let numbers_vec: Vec<u128> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap() as u128)
        .collect();

    let mut numbers = HashMap::new();

    for n in numbers_vec {
        numbers.insert(n, 1);
    }

    for _ in 0..75 {
        let mut new_numbers = HashMap::new();
        for (n, count) in numbers {
            if n == 0 {
                *new_numbers.entry(1).or_insert(0) += count;
            } else if n >= 10 {
                let s = n.to_string();
                if s.len() % 2 == 0 {
                    let (s1, s2) = s.split_at(s.len() / 2);
                    *new_numbers
                        .entry(s1.parse::<u32>().unwrap() as u128)
                        .or_insert(0) += count;
                    *new_numbers
                        .entry(s2.parse::<u32>().unwrap() as u128)
                        .or_insert(0) += count;
                    continue;
                } else {
                    *new_numbers.entry(n * 2024).or_insert(0) += count;
                }
            } else {
                *new_numbers.entry(n * 2024).or_insert(0) += count;
            }
        }
        numbers = new_numbers;
    }

    let mut res = 0_u64;

    for (_, count) in numbers {
        res += count;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
