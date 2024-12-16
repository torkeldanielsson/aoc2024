use std::{error::Error, time::Instant};

use glam::{ivec2, IVec2};
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashSet;

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

    let dijkstra_res = dijkstra(
        &(start, ivec2(1, 0)),
        |p| {
            let mut successors: Vec<((IVec2, IVec2), usize)> = Vec::new();

            let forward = (p.0 + p.1, p.1);
            if !obstacles.contains(&forward.0) {
                successors.push((forward, 1));
            }

            let left_dir = if p.1.x == 0 {
                ivec2(p.1.y, 0)
            } else {
                ivec2(0, -p.1.x)
            };

            let left = (p.0 + left_dir, left_dir);
            let right = (p.0 - left_dir, -left_dir);

            if !obstacles.contains(&left.0) {
                successors.push((left, 1001));
            }
            if !obstacles.contains(&right.0) {
                successors.push((right, 1001));
            }

            successors
        },
        |p| p.0 == goal,
    )
    .unwrap();

    println!("res: {}, {} us", dijkstra_res.1, t.elapsed().as_micros());

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
