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

            println!("button_a {button_a}");
            println!("x0a {x0a}, x0b {x0b}");

            let xa1dd = button_a.x / gcd_x;
            let xb1dd = button_b.x / gcd_x;

            let mut t_x = -x0a / xb1dd;
            assert!((x0a + xb1dd * t_x) * button_a.x + (x0b - xa1dd * t_x) * button_b.x == prize.x);
            if x0a + xb1dd * t_x < 0 {
                t_x += t_x.signum();
            }
            assert!((x0a + xb1dd * t_x) * button_a.x + (x0b - xa1dd * t_x) * button_b.x == prize.x);

            println!("t_x {t_x}");

            let y0a_bef = extended_y.1 as i64;
            let y0b_bef = extended_y.2 as i64;

            let y0a = y0a_bef * prize.y / gcd_y;
            let y0b = y0b_bef * prize.y / gcd_y;

            assert!(y0a * button_a.y + y0b * button_b.y == prize.y);

            println!("y0a {y0a}, y0b {y0b}");

            let ya1dd = button_a.y / gcd_y;
            let yb1dd = button_b.y / gcd_y;

            let mut t_y = -y0a / yb1dd;
            assert!((y0a + yb1dd * t_y) * button_a.y + (y0b - ya1dd * t_y) * button_b.y == prize.y);
            if y0a + yb1dd * t_y < 0 {
                t_y += t_y.signum();
            }
            assert!((y0a + yb1dd * t_y) * button_a.y + (y0b - ya1dd * t_y) * button_b.y == prize.y);

            println!("t_y {t_y}");

            assert!((x0a + xb1dd * t_x) * button_a.x + (x0b - xa1dd * t_x) * button_b.x == prize.x);
            assert!((y0a + yb1dd * t_y) * button_a.y + (y0b - ya1dd * t_y) * button_b.y == prize.y);

            println!("A = {}", x0a + xb1dd * t_x); // 1
            println!("A = {}", y0a + yb1dd * t_y); // 2
            println!("B = {}", x0b - xa1dd * t_x); // 3
            println!("B = {}", y0b - ya1dd * t_y); // 4

            // x0a + xb1dd * t_x = y0a + yb1dd * t_y
            // t_x = (y0a + yb1dd * t_y - x0a) / xb1dd
            // x0b - xa1dd * t_x = y0b - ya1dd * t_y
            // x0b - xa1dd * ((y0a + yb1dd * t_y - x0a) / xb1dd) = y0b - ya1dd * t_y
            // x0b - xa1dd / xb1dd * y0a + xa1dd / xb1dd * yb1dd * t_y - xa1dd / xb1dd * x0a = y0b - ya1dd * t_y
            // x0b - xa1dd / xb1dd * y0a - xa1dd / xb1dd * x0a = y0b - ya1dd * t_y - xa1dd / xb1dd * yb1dd * t_y
            // x0b - xa1dd / xb1dd * y0a - xa1dd / xb1dd * x0a = y0b - t_y * (ya1dd + xa1dd / xb1dd * yb1dd)
            // - t_y * (ya1dd + xa1dd / xb1dd * yb1dd) = x0b - xa1dd / xb1dd * y0a - xa1dd / xb1dd * x0a - y0b
            // t_y = - (x0b - xa1dd / xb1dd * y0a - xa1dd / xb1dd * x0a - y0b) / (ya1dd + xa1dd / xb1dd * yb1dd)

            let  t_y = - (x0b - xa1dd / xb1dd * y0a - xa1dd / xb1dd * x0a - y0b) / (ya1dd + xa1dd / xb1dd * yb1dd);
            let t_x = (y0a + yb1dd * t_y - x0a) / xb1dd;

            println!("A = {}", x0a + xb1dd * t_x); // 1
            println!("A = {}", y0a + yb1dd * t_y); // 2
            println!("B = {}", x0b - xa1dd * t_x); // 3
            println!("B = {}", y0b - ya1dd * t_y); // 4

            println!();
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
