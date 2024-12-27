use std::{error::Error, time::Instant};

use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut connections = FxHashMap::default();
    let mut computers = FxHashSet::default();

    for line in input.lines() {
        let line_bytes: &[u8] = line.as_bytes();
        let comp_1 = ((line_bytes[0] as u16) << 8) | (line_bytes[1] as u16);
        let comp_2 = ((line_bytes[3] as u16) << 8) | (line_bytes[4] as u16);

        connections
            .entry(comp_1)
            .or_insert(FxHashSet::default())
            .insert(comp_2);
        connections
            .entry(comp_2)
            .or_insert(FxHashSet::default())
            .insert(comp_1);
        computers.insert(comp_1);
        computers.insert(comp_2);
    }

    let mut unique_triplets = FxHashSet::default();

    for computer_1 in &computers {
        let connections_1 = &connections[computer_1];
        for computer_2 in connections_1 {
            let connections_2 = &connections[computer_2];
            let intersection: Vec<&u16> = connections_1.intersection(connections_2).collect();
            for computer_3 in intersection {
                let mut triplet = [*computer_1, *computer_2, *computer_3];
                triplet.sort();
                unique_triplets.insert((triplet[0], triplet[1], triplet[2]));
            }
        }
    }

    let mut t_triplets = Vec::new();

    for triplet in &unique_triplets {
        if (triplet.0 >> 8) as u8 == b't'
            || (triplet.1 >> 8) as u8 == b't'
            || (triplet.2 >> 8) as u8 == b't'
        {
            t_triplets.push(triplet);
        }
    }

    // for triplet in &t_triplets {
    //     println!(
    //         "triplet: {}{},{}{},{}{}",
    //         (triplet.0 >> 8) as u8 as char,
    //         (triplet.0 & 0xFF) as u8 as char,
    //         (triplet.1 >> 8) as u8 as char,
    //         (triplet.1 & 0xFF) as u8 as char,
    //         (triplet.2 >> 8) as u8 as char,
    //         (triplet.2 & 0xFF) as u8 as char,
    //     );
    // }

    // 301 is too low

    println!("res: {}, {} us", t_triplets.len(), t.elapsed().as_micros());

    Ok(())
}
