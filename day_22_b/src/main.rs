use rustc_hash::FxHashSet;
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

    let mut nine_patterns = FxHashSet::default();

    for ldi in 0..last_digits.len() {
        let ld = &last_digits[ldi];

        for (i, item) in ld.iter().enumerate().skip(4) {
            if *item == 9 {
                nine_patterns.insert((
                    diffs[ldi][i - 3],
                    diffs[ldi][i - 2],
                    diffs[ldi][i - 1],
                    diffs[ldi][i],
                ));
                break;
            }
        }
    }

    let mut res = 0;

    for pattern in nine_patterns {
        let mut pattern_res = 0;
        for i in 0..diffs.len() {
            let diff = &diffs[i];
            for j in 1..diff.len() - 3 {
                if diff[j] == pattern.0
                    && diff[j + 1] == pattern.1
                    && diff[j + 2] == pattern.2
                    && diff[j + 3] == pattern.3
                {
                    pattern_res += last_digits[i][j + 3] as u32;
                    break;
                }
            }
        }
        if pattern_res > res {
            res = pattern_res;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
