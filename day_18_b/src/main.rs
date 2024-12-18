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

fn can_solve_maze(all_obstacles: &[IVec2], obstacle_count: usize, goal: IVec2) -> bool {
    let mut obstacles = FxHashSet::with_capacity_and_hasher(5000, Default::default());

    for o in all_obstacles.iter().take(obstacle_count) {
        obstacles.insert(*o);
    }

    let mut open_set = BinaryHeap::new();
    open_set.push(Pos::new(goal.x + goal.y, ivec2(0, 0)));

    let mut came_from = FxHashMap::with_capacity_and_hasher(5000, Default::default());

    let mut g_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
    g_score.insert(ivec2(0, 0), 0);

    let mut f_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
    f_score.insert(ivec2(0, 0), goal.x + goal.y);

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            return true;
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
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g_score);
                let neighbor_f_score =
                    tentative_g_score + goal.x - neighbor.x + goal.y - neighbor.y;
                f_score.insert(neighbor, neighbor_f_score);
                open_set.push(Pos::new(neighbor_f_score, neighbor));
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut all_obstacles = Vec::with_capacity(5000);

    let mut goal = ivec2(0, 0);

    for line in input.lines() {
        let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        all_obstacles.push(ivec2(nums[0], nums[1]));
        goal.x = goal.x.max(nums[0]);
        goal.y = goal.y.max(nums[1]);
    }

    let mut obstacle_count = 1024;
    let mut half_remainder = (all_obstacles.len() - obstacle_count) / 2;

    loop {
        let solvable = can_solve_maze(&all_obstacles, obstacle_count, goal);

        if solvable {
            obstacle_count += half_remainder;
        } else {
            obstacle_count -= half_remainder;
        }

        half_remainder /= 2;

        if half_remainder < 1 {
            break;
        }
    }

    println!(
        "res: {},{}, {} us",
        all_obstacles[obstacle_count - 1].x,
        all_obstacles[obstacle_count - 1].y,
        t.elapsed().as_micros()
    );

    Ok(())
}
