use std::{
    collections::{HashMap, HashSet},
    error::Error,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut d = HashMap::new();
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                d.entry(c).or_insert(Vec::new()).push((x as i32, y as i32));
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
                let diff = (txs[i].0 - txs[j].0, txs[i].1 - txs[j].1);
                let antinode_a = (txs[i].0 + diff.0, txs[i].1 + diff.1);
                let antinode_b = (txs[j].0 - diff.0, txs[j].1 - diff.1);
                if antinode_a.0 >= 0
                    && antinode_a.0 <= max_x
                    && antinode_a.1 >= 0
                    && antinode_a.1 <= max_y
                {
                    antinodes.insert(antinode_a);
                }
                if antinode_b.0 >= 0
                    && antinode_b.0 <= max_x
                    && antinode_b.1 >= 0
                    && antinode_b.1 <= max_y
                {
                    antinodes.insert(antinode_b);
                }
            }
        }
    }

    // for y in 0..=max_y {
    //     for x in 0..=max_x {
    //         let mut was_antinode = false;
    //         for (c, txs) in &d {
    //             if txs.contains(&(x, y)) {
    //                 print!("{c}");
    //                 was_antinode = true;
    //             }
    //         }
    //         if !was_antinode {
    //             if antinodes.contains(&(x, y)) {
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
