use glam::{ivec2, IVec2};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut num_locs = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let n = c as u8 - 48;
            num_locs
                .entry(n)
                .or_insert_with(HashSet::new)
                .insert(ivec2(x as i32, y as i32));
        }
    }

    let mut res = 0;

    for start_pos in &num_locs[&0] {
        let mut prev_set: HashSet<IVec2> = HashSet::new();
        prev_set.insert(*start_pos);
        for i in 1..10 {
            let mut next_set = HashSet::new();
            for p in &prev_set {
                next_set.insert(*p + ivec2(-1, 0));
                next_set.insert(*p + ivec2(0, -1));
                next_set.insert(*p + ivec2(1, 0));
                next_set.insert(*p + ivec2(0, 1));
            }
            prev_set = num_locs[&i].intersection(&next_set).cloned().collect();
        }
        res += prev_set.len();
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
