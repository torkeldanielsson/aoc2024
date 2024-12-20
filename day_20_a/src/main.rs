use std::{error::Error, time::Instant};

use glam::ivec2;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");
    let mut obstacles = FxHashSet::default();
    let mut start = ivec2(0, 0);
    let mut goal = ivec2(0, 0);

    let mut max_map = ivec2(0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(ivec2(x as i32, y as i32));
                }
                'S' => start = ivec2(x as i32, y as i32),
                'E' => goal = ivec2(x as i32, y as i32),
                _ => {}
            }
            max_map.x = max_map.x.max(x as i32);
        }
        max_map.y = max_map.y.max(y as i32);
    }

    let mut prev = ivec2(-1, -1);
    let mut p = start;
    let mut steps = 0;

    let mut time_track = FxHashMap::default();
    time_track.insert(start, 0);

    while p != goal {
        for neighbor in &[ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor = p + neighbor;

            if !obstacles.contains(&neighbor) && neighbor != prev {
                prev = p;
                p = neighbor;
                steps += 1;
                time_track.insert(p, steps);
            }
        }
    }

    let mut res = 0;

    for (start_pos, start_score) in &time_track {
        for neighbor in &[ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor_1 = start_pos + neighbor;
            let neighbor_2 = start_pos + neighbor * 2;

            if obstacles.contains(&neighbor_1)
                && time_track.contains_key(&neighbor_2)
                && time_track[&neighbor_2] > start_score + 2
            {
                let cheat_score = time_track[&neighbor_2] - (start_score + 2);
                if cheat_score >= 100 {
                    res += 1;
                }
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
