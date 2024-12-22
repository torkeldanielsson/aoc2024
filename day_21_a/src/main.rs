use glam::{ivec2, IVec2};
use std::{error::Error, time::Instant};

// v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
// <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

fn mutate_walk_0(
    pos: IVec2,
    dest: IVec2,
    mut path: Vec<char>,
    all_paths: &mut Vec<Vec<char>>,
    movement: char,
) {
    if pos == ivec2(0, 3) {
        path.push('A');
        all_paths.push(path);

        return;
    }

    path.push(movement);

    if pos == dest {
        return;
    }

    if pos.x > dest.x {
        mutate_walk_0(pos + ivec2(-1, 0), dest, path.clone(), all_paths, '<');
    }
    if pos.x < dest.x {
        mutate_walk_0(pos + ivec2(1, 0), dest, path.clone(), all_paths, '>');
    }
    if pos.y > dest.y {
        mutate_walk_0(pos + ivec2(0, -1), dest, path.clone(), all_paths, '^');
    }
    if pos.y < dest.y {
        mutate_walk_0(pos + ivec2(0, 1), dest, path.clone(), all_paths, 'v');
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test1");

    let mut res = 0;

    for line in input.lines() {
        let mut pos_0 = ivec2(2, 3);

        let mut movement_0 = Vec::new();

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

            mutate_walk_0(pos_0, dest, path, all_paths, movement);

            pos_0 = target_pos;
        }

        // for c in &movement_0 {
        //     print!("{c}");
        // }
        // println!();

        let mut movement_1 = Vec::new();

        let mut pos_1 = ivec2(2, 0);

        for c in movement_0 {
            let target_pos = match c {
                'A' => ivec2(2, 0),
                '^' => ivec2(1, 0),
                '<' => ivec2(0, 1),
                'v' => ivec2(1, 1),
                '>' => ivec2(2, 1),
                _ => panic!(),
            };

            let mut movement = target_pos - pos_1;

            while movement.y > 0 {
                movement.y -= 1;
                movement_1.push('v');
            }
            while movement.x < 0 {
                movement.x += 1;
                movement_1.push('<');
            }
            while movement.y < 0 {
                movement.y += 1;
                movement_1.push('^');
            }
            while movement.x > 0 {
                movement.x -= 1;
                movement_1.push('>');
            }

            movement_1.push('A');

            pos_1 = target_pos;
        }

        // for c in &movement_1 {
        //     print!("{c}");
        // }
        // println!();

        let mut movement_2 = Vec::new();

        let mut pos_1 = ivec2(2, 0);

        for c in movement_1 {
            let target_pos = match c {
                'A' => ivec2(2, 0),
                '^' => ivec2(1, 0),
                '<' => ivec2(0, 1),
                'v' => ivec2(1, 1),
                '>' => ivec2(2, 1),
                _ => panic!(),
            };

            let mut movement = target_pos - pos_1;

            while movement.y > 0 {
                movement.y -= 1;
                movement_2.push('v');
            }
            while movement.x > 0 {
                movement.x -= 1;
                movement_2.push('>');
            }
            while movement.y < 0 {
                movement.y += 1;
                movement_2.push('^');
            }
            while movement.x < 0 {
                movement.x += 1;
                movement_2.push('<');
            }

            movement_2.push('A');

            pos_1 = target_pos;
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
