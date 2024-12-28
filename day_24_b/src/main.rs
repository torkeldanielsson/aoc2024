use std::{error::Error, time::Instant};

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    And,
    Xor,
    Or,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

fn match_wire_operation(
    to_match: &WireOperation,
    existing_ops: &mut Vec<WireOperation>,
    translation_table: &mut FxHashMap<u32, u32>,
    master_corrections: &mut FxHashMap<u32, u32>,
) -> bool {
    let to_match = WireOperation {
        output: to_match.output,
        wire_a: if let Some(w) = translation_table.get(&to_match.wire_a) {
            *w
        } else {
            to_match.wire_a
        },
        op: to_match.op,
        wire_b: if let Some(w) = translation_table.get(&to_match.wire_b) {
            *w
        } else {
            to_match.wire_b
        },
    };

    for ex_op in existing_ops.iter() {
        if ex_op.op == to_match.op
            && ((ex_op.wire_a == to_match.wire_a && ex_op.wire_b == to_match.wire_b)
                || (ex_op.wire_a == to_match.wire_b && ex_op.wire_b == to_match.wire_a))
        {
            if ex_op.output == to_match.output {
                // println!(
                //     "matched {} {:?} {} -> {}",
                //     u32_to_label(to_match.wire_a),
                //     to_match.op,
                //     u32_to_label(to_match.wire_b),
                //     u32_to_label(to_match.output)
                // );
            } else if ((to_match.output) & 0xFF) as u8 as char == 'z'
                || ((ex_op.output) & 0xFF) as u8 as char == 'z'
            {
                println!(
                    "    ex_op: {} {:?} {} -> {}, to_match: {} {:?} {} -> {}",
                    u32_to_label(ex_op.wire_a),
                    ex_op.op,
                    u32_to_label(ex_op.wire_b),
                    u32_to_label(ex_op.output),
                    u32_to_label(to_match.wire_a),
                    to_match.op,
                    u32_to_label(to_match.wire_b),
                    u32_to_label(to_match.output)
                );

                println!(
                    "    mismatch on z? {} <> {}",
                    u32_to_label(to_match.output),
                    u32_to_label(ex_op.output)
                );

                master_corrections.insert(to_match.output, ex_op.output);
                master_corrections.insert(ex_op.output, to_match.output);
                return false;
            } else {
                translation_table.insert(to_match.output, ex_op.output);

                // print!("translation_table: ");
                // for tr in translation_table.iter() {
                //     print!("{}>{} ",u32_to_label(*tr.0),u32_to_label(*tr.1))
                // }
                // println!();
            }

            return true;
        }
    }

    for ex_op in existing_ops.iter() {
        if ex_op.op == to_match.op && ex_op.output == to_match.output {
            if ex_op.wire_a == to_match.wire_a {
                println!(
                    "    patching wrong input {} <> {}",
                    u32_to_label(ex_op.wire_b),
                    u32_to_label(to_match.wire_b)
                );

                master_corrections.insert(to_match.wire_b, ex_op.wire_b);
                master_corrections.insert(ex_op.wire_b, to_match.wire_b);
                return false;
            }

            if ex_op.wire_a == to_match.wire_b {
                println!(
                    "    patching wrong input {} <> {}",
                    u32_to_label(ex_op.wire_b),
                    u32_to_label(to_match.wire_a)
                );

                master_corrections.insert(to_match.wire_a, ex_op.wire_b);
                master_corrections.insert(ex_op.wire_b, to_match.wire_a);
                return false;
            }

            if ex_op.wire_b == to_match.wire_a {
                println!(
                    "    patching wrong input {} <> {}",
                    u32_to_label(ex_op.wire_a),
                    u32_to_label(to_match.wire_b)
                );

                master_corrections.insert(to_match.wire_b, ex_op.wire_a);
                master_corrections.insert(ex_op.wire_a, to_match.wire_b);
                return false;
            }

            if ex_op.wire_b == to_match.wire_b {
                println!(
                    "    patching wrong input {} <> {}",
                    u32_to_label(ex_op.wire_a),
                    u32_to_label(to_match.wire_a)
                );

                master_corrections.insert(to_match.wire_a, ex_op.wire_a);
                master_corrections.insert(ex_op.wire_a, to_match.wire_a);
                return false;
            }
        }
    }

    println!(
        "NO MATCH?! {} {:?} {} -> {}",
        u32_to_label(to_match.wire_a),
        to_match.op,
        u32_to_label(to_match.wire_b),
        u32_to_label(to_match.output)
    );
    existing_ops.push(to_match);
    false
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

    let mut provided_wire_ops = Vec::new();

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

        provided_wire_ops.push(WireOperation {
            output,
            wire_a,
            op,
            wire_b,
        });
    }

    let mut master_corrections: FxHashMap<u32, u32> = FxHashMap::default();

    let mut count_down = 4;

    'repeat_loop: loop {
        if count_down == 0 {
            break;
        }
        count_down -= 1;

        let mut provided_wire_ops_copy = provided_wire_ops.clone();

        print!("output_translation_table: ");
        for tr in master_corrections.iter() {
            print!("{}>{} ", u32_to_label(*tr.0), u32_to_label(*tr.1));
        }
        println!();

        for wro in provided_wire_ops_copy.iter_mut() {
            if let Some(other_output) = master_corrections.get(&wro.output) {
                println!(
                    "swapping output {} <> {}",
                    u32_to_label(wro.output),
                    u32_to_label(*other_output)
                );
                wro.output = *other_output;
            }
        }

        let mut translation_table: FxHashMap<u32, u32> = FxHashMap::default();

        match_wire_operation(
            &WireOperation {
                output: label_to_u32("c01"),
                wire_a: label_to_u32("x00"),
                op: Op::And,
                wire_b: label_to_u32("y00"),
            },
            &mut provided_wire_ops_copy,
            &mut translation_table,
            &mut master_corrections,
        );

        match_wire_operation(
            &WireOperation {
                output: label_to_u32("z00"),
                wire_a: label_to_u32("x00"),
                op: Op::Xor,
                wire_b: label_to_u32("y00"),
            },
            &mut provided_wire_ops_copy,
            &mut translation_table,
            &mut master_corrections,
        );

        for i in 1..45 {
            let x_i = label_to_u32(format!("x{i:02}").as_str());
            let y_i = label_to_u32(format!("y{i:02}").as_str());
            let c_i = label_to_u32(format!("c{i:02}").as_str());
            let c_i_p = label_to_u32(format!("c{:02}", i + 1).as_str());
            let t_i = label_to_u32(format!("t{i:02}").as_str());
            let u_i = label_to_u32(format!("u{i:02}").as_str());
            let v_i = label_to_u32(format!("v{i:02}").as_str());
            let z_i = label_to_u32(format!("z{i:02}").as_str());

            // first XOR result
            if !match_wire_operation(
                &WireOperation {
                    output: t_i,
                    wire_a: x_i,
                    op: Op::Xor,
                    wire_b: y_i,
                },
                &mut provided_wire_ops_copy,
                &mut translation_table,
                &mut master_corrections,
            ) {
                continue 'repeat_loop;
            }
            // final sum bit for this position
            if !match_wire_operation(
                &WireOperation {
                    output: z_i,
                    wire_a: t_i,
                    op: Op::Xor,
                    wire_b: c_i,
                },
                &mut provided_wire_ops_copy,
                &mut translation_table,
                &mut master_corrections,
            ) {
                continue 'repeat_loop;
            }

            // carry from direct input bits
            if !match_wire_operation(
                &WireOperation {
                    output: u_i,
                    wire_a: x_i,
                    op: Op::And,
                    wire_b: y_i,
                },
                &mut provided_wire_ops_copy,
                &mut translation_table,
                &mut master_corrections,
            ) {
                continue 'repeat_loop;
            }
            // carry from XOR with previous carry
            if !match_wire_operation(
                &WireOperation {
                    output: v_i,
                    wire_a: t_i,
                    op: Op::And,
                    wire_b: c_i,
                },
                &mut provided_wire_ops_copy,
                &mut translation_table,
                &mut master_corrections,
            ) {
                continue 'repeat_loop;
            }
            // final carry out to next position
            if !match_wire_operation(
                &WireOperation {
                    output: c_i_p,
                    wire_a: u_i,
                    op: Op::Or,
                    wire_b: v_i,
                },
                &mut provided_wire_ops_copy,
                &mut translation_table,
                &mut master_corrections,
            ) {
                continue 'repeat_loop;
            }
        }

        break;
    }

    let mut res_set = FxHashSet::default();

    for x in master_corrections {
        res_set.insert(x.0);
        res_set.insert(x.1);
    }

    let mut res_vec: Vec<String> = res_set.iter().map(|v| u32_to_label(*v)).collect();
    res_vec.sort();

    print!("res: ");

    for rv in res_vec {
        print!("{rv},");
    }

    println!(" {} us", t.elapsed().as_micros());

    Ok(())
}
