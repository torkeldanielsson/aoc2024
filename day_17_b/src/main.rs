use std::{error::Error, time::Instant};

fn combo_op(operand: u64, reg_a: u64, reg_b: u64, reg_c: u64) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let (mut reg_a, mut reg_b, mut reg_c) = {
        let regs: Vec<u64> = input[0]
            .lines()
            .map(|l| l[12..].parse::<u64>().unwrap())
            .collect();

        (regs[0], regs[1], regs[2])
    };

    let prog: Vec<u64> = input[1][9..]
        .split(",")
        .map(|n| n.trim_ascii().parse::<u64>().unwrap())
        .collect();

    let mut res = Vec::new();

    let mut a_val = 0;

    let mut last_print = Instant::now();

    while res != prog {
        res.clear();
        a_val += 1;
        reg_a = a_val;

        let mut pc: usize = 0;

        while pc < prog.len() {
            match prog[pc] {
                0 => {
                    let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);
                    reg_a = if co >= 64 { 0 } else { reg_a >> co };
                    pc += 2;
                }
                1 => {
                    reg_b ^= prog[pc + 1];
                    pc += 2;
                }
                2 => {
                    reg_b = combo_op(prog[pc + 1], reg_a, reg_b, reg_c) & 7;
                    pc += 2;
                }
                3 => {
                    if reg_a == 0 {
                        pc += 2;
                    } else {
                        pc = prog[pc + 1] as usize;
                    }
                }
                4 => {
                    reg_b ^= reg_c;
                    pc += 2;
                }
                5 => {
                    res.push(combo_op(prog[pc + 1], reg_a, reg_b, reg_c) & 7);
                    pc += 2;
                }
                6 => {
                    let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);
                    reg_b = if co >= 64 { 0 } else { reg_a >> co };
                    pc += 2;
                }
                7 => {
                    let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);
                    reg_c = if co >= 64 { 0 } else { reg_a >> co };
                    pc += 2;
                }
                _ => panic!(),
            }
        }

        if last_print.elapsed().as_secs() >= 1 {
            println!("a_val: {a_val}, {res:?}, regs: {reg_a}, {reg_b}, {reg_c}");
            last_print = Instant::now();
        }
    }

    println!("res: {a_val}, {} us", t.elapsed().as_micros());

    Ok(())
}
