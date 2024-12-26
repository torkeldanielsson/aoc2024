use glam::{ivec2, IVec2};
use std::{collections::HashMap, error::Error, time::Instant};

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

    let characters = ['A', '^', '<', 'v', '>'];

    let mut level_movement_costs = Vec::new();

    let mut level_0_costs = HashMap::new();
    for i in 0..directional_positions.len() {
        for j in i..directional_positions.len() {
            level_0_costs.insert((characters[i], characters[j]), 1);
        }
    }
    level_movement_costs.push(level_0_costs);

    for level in 1..3 {
        let mut level_costs = HashMap::new();
        for i in 0..directional_positions.len() {
            for j in i..directional_positions.len() {
                    let pos_1 = directional_positions[i];
                    let pos_2 = directional_positions[j];
                    let mut all_paths = Vec::new();

                    mutate_walk_directional(pos_1,
                        pos_2,
                        Vec::new(),
                        &mut all_paths,
                        None,
                    );

                    let level_above_costs = &level_movement_costs[level-1];

                    let all_paths_costs = Vec::new();

                    for path in &all_paths {
                        for pi in 0..path.len()-1 {
                            let from = path[pi];
                            let to = path[pi+1];
                            
                        }
                    }
            }
        }
    }

    Ok(())
}
