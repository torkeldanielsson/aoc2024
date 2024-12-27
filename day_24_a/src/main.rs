use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

#[derive(Debug)]
enum Op {
    And,
    Xor,
    Or,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut wires = FxHashMap::default();

    for line in parts[0].lines() {
        let wire_id = &line[0..3];
        let val = &line[5..6] == "1";

        wires.insert(wire_id, val);
    }

    let mut loose_wires = Vec::new();

    for line in parts[1].lines() {
        let wire_a = &line[0..3];
        let op = match &line[4..5] {
            "A" => Op::And,
            "X" => Op::Xor,
            "O" => Op::Or,
            _ => panic!(),
        };

        let wire_b = match op {
            Op::And | Op::Xor => &line[8..11],
            Op::Or => &line[7..10],
        };

        let wire_c = match op {
            Op::And | Op::Xor => &line[15..18],
            Op::Or => &line[14..17],
        };

        loose_wires.push((wire_a, wire_b, wire_c, op));
    }

    while !loose_wires.is_empty() {
        let mut loose_wires_pong = Vec::new();
        for loose_wire in loose_wires {
            if let (Some(&val_a), Some(&val_b)) = (wires.get(loose_wire.0), wires.get(loose_wire.1))
            {
                let result = match loose_wire.3 {
                    Op::And => val_a && val_b,
                    Op::Xor => val_a ^ val_b,
                    Op::Or => val_a || val_b,
                };
                wires.insert(loose_wire.2, result);
            } else {
                loose_wires_pong.push(loose_wire);
            }
        }
        loose_wires = loose_wires_pong;
    }

    // println!("wires: {wires:?}");

    let mut res = 0_u64;

    for z in 0..64 {
        let z_id = format!("z{z:02}");
        if let Some(z_val) = wires.get(z_id.as_str()) {
            if *z_val {
                res |= 1 << z;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
