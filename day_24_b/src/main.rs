use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

#[derive(Debug)]
enum Op {
    And,
    Xor,
    Or,
}

struct WireOperation {
    output: u32,
    wire_a: u32,
    op: Op,
    wire_b: u32,
}

fn label_to_u32(label: &str) -> u32 {
    let bytes = label.as_bytes();
    (bytes[0] as u32) | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16)
}

fn u32_to_label(encoded: u32) -> String {
    let b1 = (encoded & 0xFF) as u8;
    let b2 = ((encoded >> 8) & 0xFF) as u8;
    let b3 = ((encoded >> 16) & 0xFF) as u8;
    String::from_utf8(vec![b1, b2, b3]).unwrap()
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
        let wire_a = label_to_u32(&line[0..3]);
        let op = match &line[4..5] {
            "A" => Op::And,
            "X" => Op::Xor,
            "O" => Op::Or,
            _ => panic!(),
        };

        let wire_b = label_to_u32(match op {
            Op::And | Op::Xor => &line[8..11],
            Op::Or => &line[7..10],
        });

        let output = label_to_u32(match op {
            Op::And | Op::Xor => &line[15..18],
            Op::Or => &line[14..17],
        });

        loose_wires.push(WireOperation {
            output,
            wire_a,
            op: Op::And,
            wire_b,
        });
    }

    let mut expected_connections = vec![
        WireOperation {
            output: label_to_u32("c01"),
            wire_a: label_to_u32("x00"),
            op: Op::And,
            wire_b: label_to_u32("y00"),
        },
        WireOperation {
            output: label_to_u32("z00"),
            wire_a: label_to_u32("x00"),
            op: Op::Xor,
            wire_b: label_to_u32("y00"),
        },
    ];

    for i in 1..=45 {
        let x_i = label_to_u32(format!("x{i:02}").as_str());
        let y_i = label_to_u32(format!("y{i:02}").as_str());
        let c_i = label_to_u32(format!("c{i:02}").as_str());
        let c_i_p = label_to_u32(format!("c{:02}", i + 1).as_str());
        let t_i = label_to_u32(format!("t{i:02}").as_str());
        let u_i = label_to_u32(format!("u{i:02}").as_str());
        let v_i = label_to_u32(format!("v{i:02}").as_str());
        let z_i = label_to_u32(format!("z{i:02}").as_str());

        // first XOR result
        expected_connections.push(WireOperation {
            output: t_i,
            wire_a: x_i,
            op: Op::Xor,
            wire_b: y_i,
        });
        // final sum bit for this position
        expected_connections.push(WireOperation {
            output: z_i,
            wire_a: t_i,
            op: Op::Xor,
            wire_b: c_i,
        });

        // carry from direct input bits
        expected_connections.push(WireOperation {
            output: u_i,
            wire_a: x_i,
            op: Op::And,
            wire_b: y_i,
        });
        // carry from XOR with previous carry
        expected_connections.push(WireOperation {
            output: v_i,
            wire_a: t_i,
            op: Op::And,
            wire_b: c_i,
        });
        // final carry out to next position
        expected_connections.push(WireOperation {
            output: c_i_p,
            wire_a: u_i,
            op: Op::Or,
            wire_b: v_i,
        });
    }

    println!("res: , {} us", t.elapsed().as_micros());

    Ok(())
}
