use std::{error::Error, time::Instant};

use glam::{ivec2, IVec2};

struct Robot {
    start_pos: IVec2,
    dir: IVec2,
}

impl Robot {
    fn pos_after_t(&self, t: i32) -> IVec2 {
        let pos = self.start_pos + t * self.dir;
        ivec2(pos.x.rem_euclid(101), pos.y.rem_euclid(103))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tim = Instant::now();

    let input = include_str!("../input");

    let w = 101;
    let h = 103;

    let mut robots = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" v=").collect();

        let start_pos: Vec<i32> = parts[0]
            .replace("p=", "")
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let dir: Vec<i32> = parts[1]
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let start_pos = ivec2(start_pos[0], start_pos[1]);
        let dir = ivec2(dir[0], dir[1]);

        robots.push(Robot { start_pos, dir });
    }

    let mut t = 0;

    'test_loop: loop {
        t += 1;

        let mut positions = Vec::new();

        for robot in &robots {
            let pos = robot.pos_after_t(t);
            if positions.contains(&pos) {
                continue 'test_loop;
            } else {
                positions.push(pos);
            }
        }

        break;

        // println!("situation after t={t}:");

        // for y in 0..h {
        //     for x in 0..w {
        //         if positions.contains(&ivec2(x, y)) {
        //             print!("#")
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }

        // println!();
        // println!();
        // println!();
    }

    println!("res: {t}, {} us", tim.elapsed().as_micros());

    Ok(())
}
