use std::{
    collections::{HashMap, HashSet},
    error::Error,
    time::Instant,
};

use glam::ivec2;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut d = HashMap::new();
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                d.entry(c)
                    .or_insert(Vec::new())
                    .push(ivec2(x as i32, y as i32));
            }
            if x as i32 > max_x {
                max_x = x as i32;
            }
        }
        if y as i32 > max_y {
            max_y = y as i32;
        }
    }

    let mut antinodes = HashSet::new();

    for txs in d.values() {
        for i in 0..txs.len() - 1 {
            for j in i + 1..txs.len() {
                let diff = txs[i] - txs[j];
                antinodes.insert(txs[i]);
                antinodes.insert(txs[j]);
                let mut tmp = txs[i] + diff;
                while tmp.x >= 0 && tmp.x <= max_x && tmp.y >= 0 && tmp.y <= max_y {
                    antinodes.insert(tmp);
                    tmp += diff;
                }
                tmp = txs[j] - diff;
                while tmp.x >= 0 && tmp.x <= max_x && tmp.y >= 0 && tmp.y <= max_y {
                    antinodes.insert(tmp);
                    tmp -= diff;
                }
            }
        }
    }

    // for y in 0..=max_y {
    //     for x in 0..=max_x {
    //         let mut was_antinode = false;
    //         for (c, txs) in &d {
    //             if txs.contains(&ivec2(x, y)) {
    //                 print!("{c}");
    //                 was_antinode = true;
    //             }
    //         }
    //         if !was_antinode {
    //             if antinodes.contains(&ivec2(x, y)) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //     }
    //     println!();
    // }

    println!("res: {}, {} us", antinodes.len(), t.elapsed().as_micros());

    Ok(())
}
