use rustc_hash::FxHashSet;
use std::{error::Error, time::Instant};

fn is_loop(
    obstacles: &FxHashSet<(i32, i32)>,
    mut pos: (i32, i32),
    mut dir: (i32, i32),
    max_x: i32,
    max_y: i32,
) -> bool {
    let mut visited = FxHashSet::with_capacity_and_hasher(5000, Default::default());

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 <= max_x && pos.1 <= max_y {
        if visited.contains(&(pos, dir)) {
            return true;
        }
        visited.insert((pos, dir));
        if obstacles.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            }
        } else {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }

    false
}

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

    let mut dir = (0, -1);

    let start_pos_dir = (pos, dir);

    let mut loop_positions = FxHashSet::with_capacity_and_hasher(5000, Default::default());

    let mut visited = FxHashSet::with_capacity_and_hasher(5000, Default::default());

    while pos.0 >= 0 && pos.1 >= 0 && pos.0 <= max_x && pos.1 <= max_y {
        visited.insert(pos);
        if obstacles.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            };
        } else {
            if (pos, dir) != start_pos_dir && !visited.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
                let mut tmp_obstacles = obstacles.clone();
                tmp_obstacles.insert((pos.0 + dir.0, pos.1 + dir.1));
                if is_loop(&tmp_obstacles, pos, dir, max_x, max_y) {
                    loop_positions.insert((pos.0 + dir.0, pos.1 + dir.1));
                }
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }

    println!(
        "res: {}, {} us",
        loop_positions.len(),
        t.elapsed().as_micros()
    );

    Ok(())
}
