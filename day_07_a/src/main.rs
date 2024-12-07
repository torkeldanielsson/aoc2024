use std::{error::Error, time::Instant};

fn test_ops(nums: &[u64], pos: usize, acc_val: u64, target: u64) -> bool {
    if acc_val > target {
        return false;
    }
    if pos == nums.len() {
        return acc_val == target;
    }

    test_ops(nums, pos + 1, acc_val + nums[pos], target)
        || test_ops(nums, pos + 1, acc_val * nums[pos], target)
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();

        let target = parts[0].parse::<u64>().unwrap();
        let nums: Vec<u64> = parts[1]
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        if test_ops(&nums, 0, 0, target) {
            res += target;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
