use std::{error::Error, time::Instant};

#[derive(Clone)]
struct State {
    prev_val: u16,
    is_inc: bool,
    is_dec: bool,
    has_done_skip: bool,
}

fn is_safe(nums: &[u16], i: usize, mut state: State) -> bool {
    let a = state.prev_val;
    let b = nums[i];

    if a == b {
        return false;
    }

    if a < b {
        state.is_inc = true;
    }

    if a > b {
        state.is_dec = true;
    }

    if state.is_inc && state.is_dec {
        return false;
    }

    if (a as i32 - b as i32).abs() > 3 {
        return false;
    }

    state.prev_val = b;

    if i + 2 < nums.len() && !state.has_done_skip {
        let s1_safe = is_safe(nums, i + 1, state.clone());
        if s1_safe {
            return true;
        }
        state.has_done_skip = true;
        return is_safe(nums, i + 2, state.clone());
    }

    if i + 1 < nums.len() {
        return is_safe(nums, i + 1, state.clone()) || !state.has_done_skip;
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for line in input.lines() {
        let numbers: Vec<u16> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_safe(
            &numbers,
            1,
            State {
                prev_val: numbers[0],
                is_inc: false,
                is_dec: false,
                has_done_skip: false,
            },
        ) || is_safe(
            &numbers,
            2,
            State {
                prev_val: numbers[1],
                is_inc: false,
                is_dec: false,
                has_done_skip: true,
            },
        ) || is_safe(
            &numbers,
            2,
            State {
                prev_val: numbers[0],
                is_inc: false,
                is_dec: false,
                has_done_skip: true,
            },
        ) {
            res += 1;
        }
    }

    println!("res: {res}, time: {} us", t.elapsed().as_micros());

    Ok(())
}
