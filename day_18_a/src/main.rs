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

    let input = include_str!("../test");

    let mut obstacles = FxHashSet::with_capacity_and_hasher(50000, Default::default());

    let mut goal = ivec2(0, 0);

    for line in input.lines() {
        let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        obstacles.insert(ivec2(nums[0], nums[1]));
        goal.x = goal.x.max(nums[0]);
        goal.y = goal.y.max(nums[1]);
    }

    let mut open_set = BinaryHeap::new();
    open_set.push(Pos::new(goal, ivec2(0, 0)));

    let mut came_from = FxHashMap::with_capacity_and_hasher(50000, Default::default());

    let mut g_score = FxHashMap::with_capacity_and_hasher(50000, Default::default());
    g_score.insert(ivec2(0, 0), 0);

    let mut f_score = FxHashMap::with_capacity_and_hasher(50000, Default::default());
    g_score.insert(ivec2(0, 0), goal.x + goal.y);

    while let Some(current) = open_set.pop() {
        if current == goal {
            // reconstruct path
        }

        for neighbor in &[ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor = current.pos + neighbor;

            let tentative_score = g_score[&current.pos];
        }
    }

    Ok(())
}
