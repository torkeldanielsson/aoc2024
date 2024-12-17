use std::{error::Error, time::Instant};

fn combo_op(operand: u64, reg_a: u64, reg_b: u64, reg_c: u64) -> u64 {
    match operand {
        0 => println!("combo op 0 => 0"),
        1 => println!("combo op 1 => 1"),
        2 => println!("combo op 2 => 2"),
        3 => println!("combo op 3 => 3"),
        4 => println!("combo op 4 => reg_a ({reg_a})"),
        5 => println!("combo op 5 => reg_b ({reg_b})"),
        6 => println!("combo op 6 => reg_c ({reg_c})"),
        _ => panic!(),
    };
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

    let mut pc: usize = 0;
    let mut res = Vec::new();

    while pc < prog.len() {
        match prog[pc] {
            0 => {
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);

                reg_a = if co >= 64 {
                    println!("{pc}\t0 right shift A co: {co}, reg_a: {reg_a}\t=> reg_a=0");
                    0
                } else {
                    println!(
                        "{pc}\t0 right shift A co: {co}, reg_a: {reg_a}\t=> reg_a={}",
                        reg_a >> co
                    );
                    reg_a >> co
                };
                pc += 2;
            }
            1 => {
                println!(
                    "{pc}\t1 XOR literal^B op: {}, reg_b: {reg_b}\t=> reg_b={}",
                    prog[pc + 1],
                    reg_b ^ prog[pc + 1]
                );

                reg_b ^= prog[pc + 1];
                pc += 2;
            }
            2 => {
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);

                println!("co: {:#b}", co);
                println!("res: {:#05b}", co & 7);
                println!("{pc}\t2 truncate      co: {co},\t=> reg_b={}", co & 7);

                reg_b = co & 7;
                pc += 2;
            }
            3 => {
                if reg_a == 0 {
                    println!("{pc}\t3 no jump, reg_a = 0");

                    pc += 2;
                } else {
                    println!(
                        "{pc}\t3 JUMP          reg_a: {reg_a} => pc={}",
                        prog[pc + 1]
                    );

                    pc = prog[pc + 1] as usize;
                }
            }
            4 => {
                println!(
                    "{pc}\t4 XOR  B=B^C    reg_b: {reg_b}, reg_c: {reg_c} => reg_b={}",
                    reg_b ^ reg_c
                );

                reg_b ^= reg_c;
                pc += 2;
            }
            5 => {
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);

                println!("{pc}\t5 print         co: {co} => printing {}", co & 7);

                res.push(co & 7);
                pc += 2;
            }
            6 => {
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);
                reg_b = if co >= 64 { 0 } else { reg_a >> co };
                println!("{pc}\t6 right shift B co: {co}, reg_a {reg_a} => reg_b={reg_b}");
                pc += 2;
            }
            7 => {
                let co = combo_op(prog[pc + 1], reg_a, reg_b, reg_c);
                reg_c = if co >= 64 { 0 } else { reg_a >> co };
                println!("{pc}\t6 right shift C co: {co}, reg_a {reg_a} => reg_c={reg_c} (reg_a: {reg_a:#b}, reg_c: {reg_c:#b})");
                pc += 2;
            }
            _ => panic!(),
        }
    }

    println!("res: {res:?}, {} us", t.elapsed().as_micros());

    Ok(())
}
