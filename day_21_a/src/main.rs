use glam::{ivec2, IVec2};
use std::{error::Error, time::Instant};

// v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
// <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

fn mutate_walk_0(
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
        mutate_walk_0(pos + ivec2(-1, 0), dest, path.clone(), all_paths, Some('<'));
    }
    if pos.x < dest.x {
        mutate_walk_0(pos + ivec2(1, 0), dest, path.clone(), all_paths, Some('>'));
    }
    if pos.y > dest.y {
        mutate_walk_0(pos + ivec2(0, -1), dest, path.clone(), all_paths, Some('^'));
    }
    if pos.y < dest.y {
        mutate_walk_0(pos + ivec2(0, 1), dest, path.clone(), all_paths, Some('v'));
    }
}

fn mutate_walk_12(
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
        mutate_walk_0(pos + ivec2(-1, 0), dest, path.clone(), all_paths, Some('<'));
    }
    if pos.x < dest.x {
        mutate_walk_0(pos + ivec2(1, 0), dest, path.clone(), all_paths, Some('>'));
    }
    if pos.y > dest.y {
        mutate_walk_0(pos + ivec2(0, -1), dest, path.clone(), all_paths, Some('^'));
    }
    if pos.y < dest.y {
        mutate_walk_0(pos + ivec2(0, 1), dest, path.clone(), all_paths, Some('v'));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test1");

    let mut res = 0;

    for line in input.lines() {
        let mut pos_0 = ivec2(2, 3);

        let mut movement_2: Vec<char> = Vec::new();

        // 379A

        for c in line.chars() {
            println!("c: {c}");

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

            let mut all_paths_0 = Vec::new();

            mutate_walk_0(pos_0, target_pos, Vec::new(), &mut all_paths_0, None);

            // 3->7 => <<^^A,  <^<^A,  <^^<A,  ^<<^A,  ^<^<A,  ^^<<A

            println!("    all_paths_0  for {c} {pos_0} -> {target_pos}: ");
            for path in &all_paths_0 {
                print!("    ");
                for c in path {
                    print!("{c}");
                }
                println!();
            }

            let mut all_paths_2: Vec<Vec<char>> = Vec::new();

            for path_0 in &all_paths_0 {
                let mut path_2_full: Vec<char> = Vec::new();

                // e.g. <<^^A
                print!("    path_0: ");
                for c in path_0 {
                    print!("{c}");
                }
                println!();

                let mut pos_1 = ivec2(2, 0);

                for c in path_0 {
                    println!("    processing c in path_0: {c}");

                    let target_pos_1 = match c {
                        'A' => ivec2(2, 0),
                        '^' => ivec2(1, 0),
                        '<' => ivec2(0, 1),
                        'v' => ivec2(1, 1),
                        '>' => ivec2(2, 1),
                        _ => panic!(),
                    };

                    let mut all_paths_1 = Vec::new();

                    mutate_walk_12(pos_1, target_pos_1, Vec::new(), &mut all_paths_1, None);

                    let mut all_path_2s_inner = Vec::new();

                    println!("        all_paths_1 (for c {c}) for {path_0:?} {pos_1} -> {target_pos_1}: ");
                    for path_1 in &all_paths_1 {
                        print!("        ");
                        for c in path_1 {
                            print!("{c}");
                        }
                        println!();
                    }

                    for path_1 in &all_paths_1 {
                        print!("        path_1: ");
                        for c in path_1 {
                            print!("{c}");
                        }
                        println!();

                        let mut pos_2 = ivec2(2, 0);

                        let mut path_2_inner_from_p1: Vec<char> = Vec::new();

                        for c in path_1 {
                            let target_pos_2 = match c {
                                'A' => ivec2(2, 0),
                                '^' => ivec2(1, 0),
                                '<' => ivec2(0, 1),
                                'v' => ivec2(1, 1),
                                '>' => ivec2(2, 1),
                                _ => panic!(),
                            };

                            let mut all_paths_2_inner = Vec::new();

                            mutate_walk_12(
                                pos_2,
                                target_pos_2,
                                Vec::new(),
                                &mut all_paths_2_inner,
                                None,
                            );

                            path_2_inner_from_p1.extend(all_paths_2_inner.first().unwrap());

                            pos_2 = target_pos_2;
                        }

                        all_path_2s_inner.push(path_2_inner_from_p1);
                    }

                    println!("selecting shortes inner path2 from {all_path_2s_inner:?}");
                    let shortest = all_path_2s_inner.iter().min_by_key(|path| path.len()).unwrap();
                    println!("  => {shortest:?}");
                    path_2_full.extend(shortest);

                    pos_1 = target_pos_1;
                }

                all_paths_2.push(path_2_full);
            }

            println!("{c}: {all_paths_2:?}");

            let shortest = all_paths_2.iter().min_by_key(|path| path.len()).unwrap();

            movement_2.extend(shortest);

            pos_0 = target_pos;
        }

        for c in &movement_2 {
            print!("{c}");
        }
        println!();

        let numeric_part = line[0..3].parse::<i32>().unwrap();

        print!("{} * {numeric_part} + ", movement_2.len() as i32);

        res += numeric_part * movement_2.len() as i32;
    }
    println!();

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
