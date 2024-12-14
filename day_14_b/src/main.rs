use std::{collections::HashMap, error::Error, time::Instant};

use glam::i64vec2;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let w = 101;
    let h = 103;

    let mid_w = w / 2;
    let mid_h = h / 2;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mut robot_start_positions = HashMap::new();
    let mut robot_end_positions = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" v=").collect();

        let start_pos: Vec<i64> = parts[0]
            .replace("p=", "")
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let dir: Vec<i64> = parts[1]
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let start_pos = i64vec2(start_pos[0], start_pos[1]);
        let dir = i64vec2(dir[0], dir[1]);

        // {
        //     let mut pos = start_pos;
        //     for _ in 0..6 {
        //         for y in 0..h {
        //             for x in 0..w {
        //                 if pos == i64vec2(x, y) {
        //                     print!("1")
        //                 } else {
        //                     print!(".")
        //                 }
        //             }
        //             println!()
        //         }
        //         println!();
        //         println!();
        //         pos += dir;
        //         println!("{pos}");
        //         pos = i64vec2(pos.x.rem_euclid(w), pos.y.rem_euclid(h));
        //         println!("{pos}");
        //         println!();
        //         println!();
        //     }
        // }

        let ext_pos = start_pos + dir * 100;
        let end_pos = i64vec2(ext_pos.x.rem_euclid(w), ext_pos.y.rem_euclid(h));

        *robot_start_positions.entry(start_pos).or_insert(0) += 1;
        *robot_end_positions.entry(end_pos).or_insert(0) += 1;

        if end_pos.x < mid_w && end_pos.y < mid_h {
            q1 += 1;
        } else if end_pos.x > mid_w && end_pos.y < mid_h {
            q2 += 1;
        } else if end_pos.x < mid_w && end_pos.y > mid_h {
            q3 += 1;
        } else if end_pos.x > mid_w && end_pos.y > mid_h {
            q4 += 1;
        }
    }

    // for y in 0..h {
    //     for x in 0..w {
    //         if let Some(count) = robot_start_positions.get(&i64vec2(x, y)) {
    //             print!("{count}")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    // println!();
    // println!();
    // println!();

    // for y in 0..h {
    //     for x in 0..w {
    //         if let Some(count) = robot_end_positions.get(&i64vec2(x, y)) {
    //             print!("{count}")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    // println!();
    // println!();

    // println!("{q1} {q2} {q3} {q4}");

    let res = q1 * q2 * q3 * q4;

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
