use std::{error::Error, time::Instant};

use rustc_hash::FxHashSet;

fn _print_word(word: &[u8]) -> String {
    let mut res = String::new();
    for d in word {
        res = format!("{}{}", res, *d as char);
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input_parts: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let mut words: Vec<Vec<u8>> = input_parts[0]
        .split(", ")
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();
    let sentences: Vec<Vec<u8>> = input_parts[1]
        .lines()
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();

    words.sort_by_key(|f| -(f.len() as i32));

    let mut res: u64 = 0;

    for sentence in sentences {
        let mut jump_positions = FxHashSet::with_capacity_and_hasher(50, Default::default());
        let mut tried = FxHashSet::with_capacity_and_hasher(50, Default::default());

        let mut pos = 0;

        while pos < sentence.len() {
            for word in &words {
                if sentence.len() >= pos + word.len() && &sentence[pos..pos + word.len()] == word {
                    jump_positions.insert(pos + word.len());
                }
            }

            tried.insert(pos);

            pos = 0;
            for jpos in &jump_positions {
                if !tried.contains(jpos) {
                    pos = pos.max(*jpos);
                }
            }

            if pos == 0 {
                break;
            }
        }

        if pos == sentence.len() {
            res += 1;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
