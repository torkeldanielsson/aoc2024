use std::{collections::BinaryHeap, error::Error, time::Instant};

use glam::{ivec2, IVec2};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Eq, PartialEq)]
struct Pos {
    f_score: i32,
    pos: IVec2,
}

impl Pos {
    fn new(f_score: i32, pos: IVec2) -> Self {
        Pos { f_score, pos }
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
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

    let mut open_set = BinaryHeap::new();
    open_set.push(Pos::new(goal.x + goal.y, start));

    let mut came_from = FxHashMap::with_capacity_and_hasher(5000, Default::default());

    let mut g_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
    g_score.insert(start, 0);

    let mut f_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
    f_score.insert(start, goal.x + goal.y);

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            break;
        }

        // for y in 0..=max_map.y {
        //     for x in 0..=max_map.x {
        //         let p = ivec2(x, y);
        //         if obstacles.contains(&p) {
        //             print!("#")
        //         } else if current.pos == p {
        //             print!("O")
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }
        // println!();
        // println!();

        for neighbor in &[ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor = current.pos + neighbor;

            if obstacles.contains(&neighbor)
                || neighbor.x < 0
                || neighbor.y < 0
                || neighbor.x > max_map.x
                || neighbor.y > max_map.y
            {
                continue;
            }

            let tentative_g_score = g_score[&current.pos] + 1;

            if tentative_g_score < *g_score.entry(neighbor).or_insert(i32::MAX) {
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g_score);
                let neighbor_f_score =
                    tentative_g_score + goal.x - neighbor.x + goal.y - neighbor.y;
                f_score.insert(neighbor, neighbor_f_score);
                open_set.push(Pos::new(neighbor_f_score, neighbor));
            }
        }
    }

    println!(
        "g_score goal: {}, {} us",
        g_score[&goal],
        t.elapsed().as_micros()
    );

    Ok(())
}
