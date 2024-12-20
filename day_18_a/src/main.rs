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

    let mut obstacles = FxHashSet::with_capacity_and_hasher(2000, Default::default());

    let mut goal = ivec2(0, 0);

    let mut obstacle_count = 1024;

    for line in input.lines() {
        let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        obstacles.insert(ivec2(nums[0], nums[1]));
        goal.x = goal.x.max(nums[0]);
        goal.y = goal.y.max(nums[1]);
        obstacle_count -= 1;
        if obstacle_count == 0 {
            break;
        }
    }

    let mut open_set = BinaryHeap::new();
    open_set.push(Pos::new(goal.x + goal.y, ivec2(0, 0)));

    let mut g_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
    g_score.insert(ivec2(0, 0), 0);

    let mut f_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
    f_score.insert(ivec2(0, 0), goal.x + goal.y);

    let mut res = 0;

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            res = g_score[&goal];
            break;
        }

        for neighbor in &[ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor = current.pos + neighbor;

            if obstacles.contains(&neighbor)
                || neighbor.x < 0
                || neighbor.y < 0
                || neighbor.x > goal.x
                || neighbor.y > goal.y
            {
                continue;
            }

            let tentative_g_score = g_score[&current.pos] + 1;

            if tentative_g_score < *g_score.entry(neighbor).or_insert(i32::MAX) {
                g_score.insert(neighbor, tentative_g_score);
                let neighbor_f_score =
                    tentative_g_score + goal.x - neighbor.x + goal.y - neighbor.y;
                f_score.insert(neighbor, neighbor_f_score);
                open_set.push(Pos::new(neighbor_f_score, neighbor));
            }
        }
    }

    // for y in 0..=goal.y {
    //     for x in 0..=goal.x {
    //         if obstacles.contains(&ivec2(x, y)) {
    //             print!("#")
    //         } else if path_back.contains(&ivec2(x, y)) {
    //             print!("O")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }
    // println!();

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
