use std::{error::Error, time::Instant};

use glam::{ivec2, IVec2};
use rustc_hash::{FxHashMap, FxHashSet};

fn turn_left(dir: IVec2) -> IVec2 {
    if dir.x == -1 {
        return ivec2(0, 1);
    }
    if dir.x == 1 {
        return ivec2(0, -1);
    }
    if dir.y == -1 {
        return ivec2(-1, 0);
    }
    if dir.y == 1 {
        return ivec2(1, 0);
    }
    panic!()
}

fn turn_right(dir: IVec2) -> IVec2 {
    if dir.x == -1 {
        return ivec2(0, -1);
    }
    if dir.x == 1 {
        return ivec2(0, 1);
    }
    if dir.y == -1 {
        return ivec2(1, 0);
    }
    if dir.y == 1 {
        return ivec2(-1, 0);
    }
    panic!()
}

fn fill(
    pos: IVec2,
    dir: IVec2,
    cost: i32,
    moves: &mut FxHashMap<IVec2, i32>,
    obstacles: &FxHashSet<IVec2>,
) {
    let forward_pos = pos + dir;
    if !obstacles.contains(&forward_pos)
        && (!moves.contains_key(&forward_pos) || moves[&forward_pos] > cost + 1)
    {
        moves.insert(forward_pos, cost + 1);
        fill(forward_pos, dir, cost + 1, moves, obstacles);
    }

    let left_dir = turn_left(dir);
    let left_pos = pos + left_dir;
    if !obstacles.contains(&left_pos)
        && (!moves.contains_key(&left_pos) || moves[&left_pos] > cost + 1001)
    {
        moves.insert(left_pos, cost + 1001);
        fill(left_pos, left_dir, cost + 1001, moves, obstacles);
    }

    let right_dir = turn_right(dir);
    let right_pos = pos + right_dir;
    if !obstacles.contains(&right_pos)
        && (!moves.contains_key(&right_pos) || moves[&right_pos] > cost + 1001)
    {
        moves.insert(right_pos, cost + 1001);
        fill(right_pos, right_dir, cost + 1001, moves, obstacles);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");
    let mut obstacles = FxHashSet::default();
    let mut start = ivec2(0, 0);
    let mut goal = ivec2(0, 0);

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
        }
    }

    let mut moves = FxHashMap::default();

    fill(start, ivec2(1, 0), 0, &mut moves, &obstacles);

    println!("res: {}, {} us", moves[&goal], t.elapsed().as_micros());

    Ok(())
}
