use std::{error::Error, time::Instant};

use glam::i64vec2;

fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}

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

        println!("button_a {button_a}, button_b {button_b}");

        println!("A * {} + B * {} = {}", button_a.x, button_b.x, prize.x);
        println!("A * {} + B * {} = {}", button_a.y, button_b.y, prize.y);

        println!(
            "A * {} + B * {} = {}",
            button_a.x * button_a.y,
            button_b.x * button_a.y,
            prize.x * button_a.y
        );
        println!(
            "A * {} + B * {} = {}",
            button_a.y * button_a.x,
            button_b.y * button_a.x,
            prize.y * button_a.x
        );

        println!(
            "A * {} = {} - B * {}",
            button_a.x * button_a.y,
            prize.x * button_a.y,
            button_b.x * button_a.y
        );
        println!(
            "A * {} = {} - B * {}",
            button_a.y * button_a.x,
            prize.y * button_a.x,
            button_b.y * button_a.x
        );

        println!(
            "{} - B * {} = {} - B * {}",
            prize.x * button_a.y,
            button_b.x * button_a.y,
            prize.y * button_a.x,
            button_b.y * button_a.x
        );

        println!(
            "{} - {} =  - B * {} + B * {}",
            prize.x * button_a.y,
            prize.y * button_a.x,
            button_b.y * button_a.x,
            button_b.x * button_a.y
        );

        println!(
            "{} = B * ({} - {})",
            prize.x * button_a.y - prize.y * button_a.x,
            button_b.x * button_a.y,
            button_b.y * button_a.x
        );

        println!(
            "{} = B * {}",
            prize.x * button_a.y - prize.y * button_a.x,
            button_b.x * button_a.y - button_b.y * button_a.x
        );

        let left = prize.x * button_a.y - prize.y * button_a.x;
        let right = button_b.x * button_a.y - button_b.y * button_a.x;

        if left % right != 0 {
            println!("no good");
        } else {
            let b = left / right;
            println!("B = {b}");

            println!(
                "A * {} + {b} * {} = {}",
                button_a.y * button_a.x,
                button_b.y * button_a.x,
                prize.y * button_a.x
            );

            println!(
                "A * {} = {} - {b} * {}",
                button_a.y * button_a.x,
                prize.y * button_a.x,
                button_b.y * button_a.x
            );

            println!(
                "A * {} = {} - {}",
                button_a.y * button_a.x,
                prize.y * button_a.x,
                b * button_b.y * button_a.x
            );

            println!(
                "A * {} = {}",
                button_a.y * button_a.x,
                prize.y * button_a.x - b * button_b.y * button_a.x
            );

            let left = button_a.y * button_a.x;
            let right = prize.y * button_a.x - b * button_b.y * button_a.x;

            if right % left != 0 {
                println!("NO GOOD");
            } else {
                let a = right / left;
                res += a * 3 + b;
            }
        }

        println!();
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
