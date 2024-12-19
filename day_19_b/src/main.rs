use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

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

    let words: Vec<Vec<u8>> = input_parts[0]
        .split(", ")
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();
    let sentences: Vec<Vec<u8>> = input_parts[1]
        .lines()
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();

    let mut res: u64 = 0;

    for sentence in sentences {
        let mut jump_counts = FxHashMap::default();

        let mut pos = 0;
        let mut count = 1;

        while pos < sentence.len() {
            for word in &words {
                if sentence.len() >= pos + word.len() && &sentence[pos..pos + word.len()] == word {
                    *jump_counts.entry(pos + word.len()).or_insert(0) += count;
                }
            }

            pos = usize::MAX;
            for jpos in &jump_counts {
                if pos > *jpos.0 {
                    pos = *jpos.0;
                    count = *jpos.1;
                }
            }

            jump_counts.remove(&pos);
        }

        if pos == sentence.len() {
            res += count;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
