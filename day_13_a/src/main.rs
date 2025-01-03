use std::{error::Error, time::Instant};

use glam::uvec2;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for part in input.split("\n\n") {
        let lines: Vec<&str> = part.lines().collect();
        let button_a: Vec<u32> = lines[0][12..]
            .split(", Y+")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let button_b: Vec<u32> = lines[1][12..]
            .split(", Y+")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let prize: Vec<u32> = lines[2][9..]
            .split(", Y=")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let button_a = uvec2(button_a[0], button_a[1]);
        let button_b = uvec2(button_b[0], button_b[1]);
        let prize = uvec2(prize[0], prize[1]);

        let mut b_pushes = prize.x / button_b.x;
        let mut a_pushes = 0;
        loop {
            let curr_val = a_pushes * button_a + b_pushes * button_b;
            if curr_val == prize {
                res += 3 * a_pushes + b_pushes;
                break;
            } else if curr_val.x < prize.x && curr_val.y < prize.y {
                a_pushes += 1;
            } else if b_pushes == 0 {
                break;
            } else {
                b_pushes -= 1;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
