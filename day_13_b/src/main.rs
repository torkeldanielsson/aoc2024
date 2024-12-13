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

    let input = include_str!("../test");

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

        let extended_x = extended_euclidean_algorithm(button_a.x as i32, button_b.x as i32);
        let extended_y = extended_euclidean_algorithm(button_a.y as i32, button_b.y as i32);

        let gcd_x = extended_x.0 as i64;
        let gcd_y = extended_y.0 as i64;

        if prize.x % gcd_x == 0 && prize.y % gcd_y == 0 {
            let x0a_bef = extended_x.1 as i64;
            let x0b_bef = extended_x.2 as i64;

            let x0a = x0a_bef * prize.x / gcd_x;
            let x0b = x0b_bef * prize.x / gcd_x;

            assert!(x0a * button_a.x + x0b * button_b.x == prize.x);

            println!("x0a {x0a}, x0b {x0b}");

            let xa1dd = button_a.x / gcd_x;
            let xb1dd = button_b.x / gcd_x;

            let mut test = -x0a / xb1dd;
            assert!(
                (x0a + xb1dd * test) * button_a.x + (x0b - xa1dd * test) * button_b.x == prize.x
            );
            if x0a + xb1dd * test < 0 {
                test += test.signum();
            }

            println!("{test} {}", x0a + xb1dd * test);


            println!();
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
