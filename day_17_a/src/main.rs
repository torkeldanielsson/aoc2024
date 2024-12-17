use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input: Vec<&str> = include_str!("../test1").split("\n\n").collect();

    let (mut reg_a, mut reg_b, mut reg_c) = {
        let regs: Vec<i32> = input[0]
            .lines()
            .map(|l| l[12..].parse::<i32>().unwrap())
            .collect();

        (regs[0], regs[1], regs[2])
    };

    let prog: Vec<i32> = input[1][9..]
        .split(",")
        .map(|n| n.trim_ascii().parse::<i32>().unwrap())
        .collect();

    let mut pc = 0;

    while pc < prog.len() {
        match prog[pc] {
            0 => 
            _=>panic!()
        }
    }

    Ok(())
}
