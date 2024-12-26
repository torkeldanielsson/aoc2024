use glam::{ivec2, IVec2};
use std::{collections::HashMap, error::Error, time::Instant};

fn mutate_walk_numeric(
    pos: IVec2,
    dest: IVec2,
    mut path: Vec<char>,
    all_paths: &mut Vec<Vec<char>>,
    movement: Option<char>,
) {
    if pos == ivec2(0, 3) {
        return;
    }

    if let Some(m) = movement {
        path.push(m);
    }

    if pos == dest {
        path.push('A');
        all_paths.push(path);
        return;
    }

    if pos.x > dest.x {
        mutate_walk_numeric(pos + ivec2(-1, 0), dest, path.clone(), all_paths, Some('<'));
    }
    if pos.x < dest.x {
        mutate_walk_numeric(pos + ivec2(1, 0), dest, path.clone(), all_paths, Some('>'));
    }
    if pos.y > dest.y {
        mutate_walk_numeric(pos + ivec2(0, -1), dest, path.clone(), all_paths, Some('^'));
    }
    if pos.y < dest.y {
        mutate_walk_numeric(pos + ivec2(0, 1), dest, path.clone(), all_paths, Some('v'));
    }
}

fn mutate_walk_directional(
    pos: IVec2,
    dest: IVec2,
    mut path: Vec<char>,
    all_paths: &mut Vec<Vec<char>>,
    movement: Option<char>,
) {
    if pos == ivec2(0, 0) {
        return;
    }

    if let Some(m) = movement {
        path.push(m);
    }

    if pos == dest {
        path.push('A');
        all_paths.push(path);
        return;
    }

    if pos.x > dest.x {
        mutate_walk_directional(pos + ivec2(-1, 0), dest, path.clone(), all_paths, Some('<'));
    }
    if pos.x < dest.x {
        mutate_walk_directional(pos + ivec2(1, 0), dest, path.clone(), all_paths, Some('>'));
    }
    if pos.y > dest.y {
        mutate_walk_directional(pos + ivec2(0, -1), dest, path.clone(), all_paths, Some('^'));
    }
    if pos.y < dest.y {
        mutate_walk_directional(pos + ivec2(0, 1), dest, path.clone(), all_paths, Some('v'));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let directional_positions = [
        ivec2(2, 0), // A
        ivec2(1, 0), // ^
        ivec2(0, 1), // <
        ivec2(1, 1), // v
        ivec2(2, 1), // >
    ];

    let directional_characters = ['A', '^', '<', 'v', '>'];

    let mut level_movement_costs = Vec::new();

    let mut level_0_costs = HashMap::new();
    for i in 0..directional_positions.len() {
        for j in 0..directional_positions.len() {
            level_0_costs.insert((directional_characters[i], directional_characters[j]), 1);
        }
    }
    level_movement_costs.push(level_0_costs);

    for level in 1..3 {
        let mut level_costs = HashMap::new();

        let level_above_costs = &level_movement_costs[level - 1];

        for i in 0..directional_positions.len() {
            for j in 0..directional_positions.len() {
                let pos_1 = directional_positions[i];
                let pos_2 = directional_positions[j];

                let mut all_paths = Vec::new();

                mutate_walk_directional(pos_1, pos_2, Vec::new(), &mut all_paths, None);

                let mut all_paths_costs = Vec::new();

                for path in &all_paths {
                    let mut path_cost = 0;
                    let mut c = 'A';
                    for p in path.iter() {
                        path_cost += level_above_costs[&(c, *p)];
                        c = *p;
                    }

                    all_paths_costs.push(path_cost);
                }

                let mut lowest_cost = i32::MAX;
                let mut lowest_cost_path = Vec::new();

                for i in 0..all_paths.len() {
                    if all_paths_costs[i] < lowest_cost {
                        lowest_cost = all_paths_costs[i];
                        lowest_cost_path = all_paths[i].clone();
                    }
                }

                level_costs.insert(
                    (directional_characters[i], directional_characters[j]),
                    lowest_cost,
                );
            }
        }

        level_movement_costs.insert(level, level_costs);
    }

    let last_directional_level_costs = level_movement_costs.last().unwrap();

    let mut res = 0;

    for line in input.lines() {
        let mut pos = ivec2(2, 3);

        let mut line_cost = 0;

        for c in line.chars() {
            let target_pos = match c {
                '0' => ivec2(1, 3),
                '1' => ivec2(0, 2),
                '2' => ivec2(1, 2),
                '3' => ivec2(2, 2),
                '4' => ivec2(0, 1),
                '5' => ivec2(1, 1),
                '6' => ivec2(2, 1),
                '7' => ivec2(0, 0),
                '8' => ivec2(1, 0),
                '9' => ivec2(2, 0),
                'A' => ivec2(2, 3),
                _ => panic!(),
            };

            let mut all_paths = Vec::new();

            mutate_walk_numeric(pos, target_pos, Vec::new(), &mut all_paths, None);

            pos = target_pos;

            let mut all_paths_costs = Vec::new();

            for path in &all_paths {
                let mut path_cost = 0;
                let mut c = 'A';
                for p in path.iter() {
                    path_cost += last_directional_level_costs[&(c, *p)];
                    c = *p;
                }

                all_paths_costs.push(path_cost);
            }

            let mut lowest_cost = i32::MAX;

            for i in 0..all_paths.len() {
                if all_paths_costs[i] < lowest_cost {
                    lowest_cost = all_paths_costs[i];
                }
            }

            line_cost += lowest_cost;
        }

        let numeric_part = line[0..3].parse::<i32>().unwrap();

        res += numeric_part * line_cost;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
