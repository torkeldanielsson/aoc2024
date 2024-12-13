use std::{error::Error, time::Instant};

use glam::i64vec2;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for part in input.split("\n\n") {
        let lines: Vec<&str> = part.lines().collect();
        let button_a: Vec<i64> = lines[0][12..]
            .split(", Y+")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let button_b: Vec<i64> = lines[1][12..]
            .split(", Y+")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let prize: Vec<i64> = lines[2][9..]
            .split(", Y=")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let button_a = i64vec2(button_a[0], button_a[1]);
        let button_b = i64vec2(button_b[0], button_b[1]);
        let prize = i64vec2(prize[0] + 10000000000000, prize[1] + 10000000000000);

        let left = prize.x * button_a.y - prize.y * button_a.x;
        let right = button_b.x * button_a.y - button_b.y * button_a.x;

        if left % right == 0 {
            let b = left / right;

            let left = button_a.y * button_a.x;
            let right = prize.y * button_a.x - b * button_b.y * button_a.x;

            if right % left == 0 {
                let a = right / left;
                res += a * 3 + b;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
