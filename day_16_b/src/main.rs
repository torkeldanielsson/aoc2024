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

fn count_besties(pos: IVec2, cost: i32, moves: &FxHashMap<IVec2, i32>, besties: &mut Vec<IVec2>) {
    besties.push(pos);

    // if *bestie_count > 30 {
    //     return;
    // }

    let mut lowest_cost = cost;
    let mut bestie_neighbors = Vec::new();
    for neighbor in &[ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
        if let Some(&maybe_best_neighbor_cost) = moves.get(&(pos + neighbor)) {
            match maybe_best_neighbor_cost.cmp(&lowest_cost) {
                std::cmp::Ordering::Less => {
                    lowest_cost = maybe_best_neighbor_cost;
                    bestie_neighbors.clear();
                    bestie_neighbors.push(pos + neighbor);
                }
                std::cmp::Ordering::Equal => {
                    bestie_neighbors.push(pos + neighbor);
                }
                std::cmp::Ordering::Greater => {}
            }
        }
    }

    println!("pos {pos}, cost {cost}, bestie_neighbors: {bestie_neighbors:?}");

    for bestie_neighbor in bestie_neighbors {
        count_besties(bestie_neighbor, lowest_cost, moves, besties);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test1");
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

    fill(start, ivec2(1, 0), 0, &mut moves, &obstacles);

    let mut besties = Vec::new();
    count_besties(goal, moves[&goal], &moves, &mut besties);

    for y in 0..=max_map.y {
        for x in 0..=max_map.x {
            let p = ivec2(x, y);
            if obstacles.contains(&p) {
                print!("#")
            } else if besties.contains(&p) {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!()
    }

    println!("res: {}, {} us", besties.len(), t.elapsed().as_micros());

    let p = ivec2(1,10);
    println!("{p}: {}", moves[&p]);
    let p = ivec2(2,11);
    println!("{p}: {}", moves[&p]);
    let p = ivec2(1,10);
    println!("{p}: {}", moves[&p]);
    let p = ivec2(1,10);
    println!("{p}: {}", moves[&p]);

    Ok(())
}
