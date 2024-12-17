use std::{collections::BinaryHeap, error::Error, time::Instant};

use glam::{ivec2, IVec2};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Eq, PartialEq)]
struct Pos {
    cost: i32,
    pos: IVec2,
    dir: IVec2,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

    let mut visited = FxHashMap::default();
    visited.insert((start, ivec2(1, 0)), 0);

    let mut unvisited = BinaryHeap::new();

    unvisited.push(Pos {
        cost: 0,
        pos: start,
        dir: ivec2(1, 0),
    });

    while let Some(p) = unvisited.pop() {
        let mut neighbors: Vec<((IVec2, IVec2), i32)> = Vec::new();

        let forward = (p.pos + p.dir, p.dir);
        if !obstacles.contains(&forward.0) {
            neighbors.push((forward, 1));
        }

        let left_dir = if p.dir.x == 0 {
            ivec2(p.dir.y, 0)
        } else {
            ivec2(0, -p.dir.x)
        };

        let left = (p.pos + left_dir, left_dir);
        let right = (p.pos - left_dir, -left_dir);

        if !obstacles.contains(&left.0) {
            neighbors.push((left, 1001));
        }
        if !obstacles.contains(&right.0) {
            neighbors.push((right, 1001));
        }

        for ((npos, ndir), move_cost) in neighbors {
            let new_cost = p.cost + move_cost;
            match visited.entry((npos, ndir)) {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    if *occupied_entry.get() > new_cost {
                        visited.insert((npos, ndir), new_cost);
                    } else {
                        continue;
                    }
                }
                std::collections::hash_map::Entry::Vacant(_vacant_entry) => {
                    visited.insert((npos, ndir), new_cost);
                }
            }
            unvisited.push(Pos {
                cost: new_cost,
                pos: npos,
                dir: ndir,
            });
        }
    }

    let mut res_1 = i32::MAX;
    if let Some(r) = visited.get(&(goal, ivec2(1, 0))) {
        res_1 = res_1.min(*r);
    }
    if let Some(r) = visited.get(&(goal, ivec2(-1, 0))) {
        res_1 = res_1.min(*r);
    }
    if let Some(r) = visited.get(&(goal, ivec2(0, 1))) {
        res_1 = res_1.min(*r);
    }
    if let Some(r) = visited.get(&(goal, ivec2(0, -1))) {
        res_1 = res_1.min(*r);
    }

    println!("A res: {res_1}, {} us", t.elapsed().as_micros());

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

    Ok(())
}
