use rustc_hash::FxHashMap;
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut d = FxHashMap::with_capacity_and_hasher(50000, Default::default());
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            d.insert((x as i32, y as i32), c);
            if x as i32 > max_x {
                max_x = x as i32;
            }
        }
        if y as i32 > max_y {
            max_y = y as i32;
        }
    }

    let directions = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut res = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if d.get(&(x, y)) == Some(&'X') {
                for dir in &directions {
                    if d.get(&(x + dir.0, y + dir.1)) == Some(&'M')
                        && d.get(&(x + 2 * dir.0, y + 2 * dir.1)) == Some(&'A')
                        && d.get(&(x + 3 * dir.0, y + 3 * dir.1)) == Some(&'S')
                    {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("res: {res}, time: {} us", t.elapsed().as_micros());

    Ok(())
}
