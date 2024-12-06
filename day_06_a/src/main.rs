use rustc_hash::FxHashSet;
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut obstacles = FxHashSet::with_capacity_and_hasher(5000, Default::default());
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert((x as i32, y as i32));
            }
            if c == '^' {
                pos = (x as i32, y as i32);
            }
            if x as i32 > max_x {
                max_x = x as i32;
            }
        }
        if y as i32 > max_y {
            max_y = y as i32;
        }
    }

    let mut visited = FxHashSet::with_capacity_and_hasher(5000, Default::default());

    let mut direction = (0, -1);

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 <= max_x && pos.1 <= max_y {
        visited.insert(pos);
        if obstacles.contains(&(pos.0 + direction.0, pos.1 + direction.1)) {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            }
        } else {
            pos = (pos.0 + direction.0, pos.1 + direction.1);
        }
    }

    println!("res: {}, {} us", visited.len(), t.elapsed().as_micros());

    Ok(())
}
