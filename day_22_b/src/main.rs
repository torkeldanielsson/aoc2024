use rustc_hash::{FxHashMap, FxHashSet};
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

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
        diff.push(0);
        for pair in ld.windows(2) {
            let d = pair[1] - pair[0];
            diff.push(d);
        }
        diffs.push(diff);
    }

    let mut pattern_sums = FxHashMap::with_capacity_and_hasher(50000, Default::default());

    for ldi in 0..last_digits.len() {
        let ld = &last_digits[ldi];

        let mut seen_patterns = FxHashSet::with_capacity_and_hasher(2000, Default::default());

        for (i, _item) in ld.iter().enumerate().skip(4) {
            let pattern = (
                diffs[ldi][i - 3],
                diffs[ldi][i - 2],
                diffs[ldi][i - 1],
                diffs[ldi][i],
            );
            if !seen_patterns.contains(&pattern) {
                seen_patterns.insert(pattern);
                *pattern_sums.entry(pattern).or_insert(0) += last_digits[ldi][i] as u32;
            }
        }
    }

    let mut res = 0;

    for (_, pattern_res) in pattern_sums {
        if pattern_res > res {
            res = pattern_res;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
