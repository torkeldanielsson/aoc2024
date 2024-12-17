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

fn compute(prog: &[u64], mut reg_a: u64, mut reg_b: u64, mut reg_c: u64) -> Vec<u64> {
    let mut res = Vec::new();

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
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);

                reg_b = co & 7;
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
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);

                res.push(co & 7);
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

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let (mut reg_a, reg_b, reg_c) = {
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

    let mut res = compute(&prog, reg_a, reg_b, reg_c);

    let mut a_val = reg_a;

    while res.len() < prog.len() {
        a_val = (a_val as f32 * 1.1) as u64;
        reg_a = a_val;
        res = compute(&prog, reg_a, reg_b, reg_c);
    }

    let mut diff_val = (a_val as f32 * 0.0001) as u64;

    for i in 0..prog.len() - 1 {
        let limit = prog.len() - 1 - i;

        diff_val = (diff_val as f32 / 4.0) as u64;

        while res[limit..] != prog[limit..] {
            if res.len() > prog.len() {
                println!("div!");
                a_val /= 1000;
            }
            a_val += diff_val;
            reg_a = a_val;
            res = compute(&prog, reg_a, reg_b, reg_c);
            // println!(
            //     "{limit} {a_val} => res: {res:?}, {} us",
            //     t.elapsed().as_micros()
            // );
        }

        // println!(
        //     "{limit} reached {a_val} => res: {res:?}, {} us",
        //     t.elapsed().as_micros()
        // );
    }

    while res != prog {
        a_val += 1;
        reg_a = a_val;
        res = compute(&prog, reg_a, reg_b, reg_c);
    }

    println!("res: {a_val}, {} us", t.elapsed().as_micros());

    Ok(())
}
