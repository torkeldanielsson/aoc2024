use std::{error::Error, time::Instant};

use glam::ivec2;
use rustc_hash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let mut obstacles = FxHashSet::default();
    let mut boxes = FxHashSet::default();
    let mut pos = ivec2(0, 0);

    let mut map_max = ivec2(0, 0);

    for (y, line) in input[0].lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert(ivec2(x as i32, y as i32));
            }
            if c == 'O' {
                boxes.insert(ivec2(x as i32, y as i32));
            }
            if c == '@' {
                pos = ivec2(x as i32, y as i32);
            }
            map_max.x = map_max.x.max(x as i32);
            map_max.y = map_max.y.max(y as i32);
        }
    }

    for c in input[1].lines().flat_map(|line| line.chars()) {
        let dir = match c {
            '<' => ivec2(-1, 0),
            '>' => ivec2(1, 0),
            '^' => ivec2(0, -1),
            'v' => ivec2(0, 1),
            _ => panic!(),
        };

        let maybe_pos = pos + dir;

        if !obstacles.contains(&maybe_pos) {
            if !boxes.contains(&maybe_pos) {
                pos = maybe_pos;
            } else {
                let mut d = 2;
                while boxes.contains(&(pos + d * dir)) {
                    d += 1;
                }
                if !obstacles.contains(&(pos + d * dir)) {
                    boxes.remove(&maybe_pos);
                    boxes.insert(pos + d * dir);
                    pos = maybe_pos;
                }
            }
        }

        // for y in 0..=map_max.y {
        //     for x in 0..=map_max.x {
        //         let p = ivec2(x, y);
        //         if obstacles.contains(&p) {
        //             print!("#");
        //         } else if boxes.contains(&p) {
        //             print!("O");
        //         } else if pos == p {
        //             print!("@");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();
        // println!();
    }

    let mut res = 0;

    for o in boxes {
        res += 100 * o.y + o.x;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
