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
    goal: IVec2,
) {
    if pos == goal {
        return;
    }

    let forward_pos = pos + dir;
    if !obstacles.contains(&forward_pos)
        && (!moves.contains_key(&forward_pos) || moves[&forward_pos] > cost + 1)
    {
        moves.insert(forward_pos, cost + 1);
        fill(forward_pos, dir, cost + 1, moves, obstacles, goal);
    }

    let left_dir = turn_left(dir);
    let left_pos = pos + left_dir;
    if !obstacles.contains(&left_pos)
        && (!moves.contains_key(&left_pos) || moves[&left_pos] > cost + 1001)
    {
        moves.insert(left_pos, cost + 1001);
        fill(left_pos, left_dir, cost + 1001, moves, obstacles, goal);
    }

    let right_dir = turn_right(dir);
    let right_pos = pos + right_dir;
    if !obstacles.contains(&right_pos)
        && (!moves.contains_key(&right_pos) || moves[&right_pos] > cost + 1001)
    {
        moves.insert(right_pos, cost + 1001);
        fill(right_pos, right_dir, cost + 1001, moves, obstacles, goal);
    }
}

#[allow(clippy::too_many_arguments)]
fn fill2(
    pos: IVec2,
    dir: IVec2,
    cost: i32,
    moves: &mut FxHashMap<IVec2, i32>,
    obstacles: &FxHashSet<IVec2>,
    unique_moves: Vec<IVec2>,
    besties: &mut FxHashSet<IVec2>,
    goal: IVec2,
    goal_target_cost: i32,
) {
    if cost > goal_target_cost {
        return;
    }

    if pos == goal && cost == goal_target_cost {
        for b in &unique_moves {
            besties.insert(*b);
        }
    }

    let forward_pos = pos + dir;
    if !obstacles.contains(&forward_pos)
        && (!moves.contains_key(&forward_pos) || moves[&forward_pos] >= cost + 1)
    {
        moves.insert(pos, cost + 1000);
        moves.insert(forward_pos, cost + 1);
        let mut new_unique = unique_moves.clone();
        new_unique.push(forward_pos);
        fill2(
            forward_pos,
            dir,
            cost + 1,
            moves,
            obstacles,
            new_unique,
            besties,
            goal,
            goal_target_cost,
        );
    }

    let left_dir = turn_left(dir);
    let left_pos = pos + left_dir;
    if !obstacles.contains(&left_pos)
        && (!moves.contains_key(&left_pos) || moves[&left_pos] >= cost + 1001)
    {
        moves.insert(left_pos, cost + 1001);
        moves.insert(pos, cost + 1000);
        let mut new_unique = unique_moves.clone();
        new_unique.push(left_pos);
        fill2(
            left_pos,
            left_dir,
            cost + 1001,
            moves,
            obstacles,
            new_unique,
            besties,
            goal,
            goal_target_cost,
        );
    }

    let right_dir = turn_right(dir);
    let right_pos = pos + right_dir;
    if !obstacles.contains(&right_pos)
        && (!moves.contains_key(&right_pos) || moves[&right_pos] >= cost + 1001)
    {
        moves.insert(right_pos, cost + 1001);
        let mut new_unique = unique_moves.clone();
        new_unique.push(right_pos);
        fill2(
            right_pos,
            right_dir,
            cost + 1001,
            moves,
            obstacles,
            new_unique,
            besties,
            goal,
            goal_target_cost,
        );
    }
}

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

    let mut moves = FxHashMap::default();
    moves.insert(start, 0);

    fill(start, ivec2(1, 0), 0, &mut moves, &obstacles, goal);

    // println!("A res: {}", moves[&goal]);

    let mut besties = FxHashSet::default();
    besties.insert(start);

    let mut moves2 = FxHashMap::default();
    moves2.insert(start, 0);

    fill2(
        start,
        ivec2(1, 0),
        0,
        &mut moves2,
        &obstacles,
        Vec::new(),
        &mut besties,
        goal,
        moves[&goal],
    );

    // for y in 0..=max_map.y {
    //     for x in 0..=max_map.x {
    //         let p = ivec2(x, y);
    //         if obstacles.contains(&p) {
    //             print!("#")
    //         } else if besties.contains(&p) {
    //             print!("O")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }

    println!("res: {}, {} us", besties.len(), t.elapsed().as_micros());

    Ok(())
}
