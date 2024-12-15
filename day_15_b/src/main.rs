use std::{error::Error, time::Instant};

use glam::{ivec2, IVec2};

struct Map {
    data: Vec<char>,
    w: i32,
    h: i32,
}

impl Map {
    fn new(data: Vec<char>, w: i32, h: i32) -> Self {
        Map { data, w, h }
    }

    fn clone_from(&mut self, data: &Vec<char>) {
        self.data.clone_from(data);
    }

    fn at(&self, coord: &IVec2) -> char {
        // if coord.x < 0 || coord.x > self.w || coord.y < 0 || coord.y > self.h {
        //     return 'F';
        // }
        self.data[(coord.y * (self.w) + coord.x) as usize]
    }

    fn set(&mut self, coord: &IVec2, val: char) {
        self.data[(coord.y * (self.w) + coord.x) as usize] = val;
    }
}

fn maybe_push(pushed_map: &mut Map, dir: &IVec2, pos: &IVec2) -> bool {
    match pushed_map.at(pos) {
        '.' => return true,
        '#' => return false,
        _ => {}
    }

    let (left, right) = match pushed_map.at(pos) {
        '[' => (*pos, *pos + ivec2(1, 0)),
        ']' => (*pos + ivec2(-1, 0), *pos),
        _ => panic!(),
    };

    if dir.x == -1 {
        if maybe_push(pushed_map, dir, &(left + dir)) {
            pushed_map.set(&(left + dir), '[');
            pushed_map.set(&(right + dir), ']');
            pushed_map.set(&right, '.');
            return true;
        } else {
            return false;
        }
    } else if dir.x == 1 {
        if maybe_push(pushed_map, dir, &(right + dir)) {
            pushed_map.set(&(left + dir), '[');
            pushed_map.set(&(right + dir), ']');
            pushed_map.set(&left, '.');
            return true;
        } else {
            return false;
        }
    } else if maybe_push(pushed_map, dir, &(left + dir))
        && maybe_push(pushed_map, dir, &(right + dir))
    {
        pushed_map.set(&(left + dir), '[');
        pushed_map.set(&(right + dir), ']');
        pushed_map.set(&left, '.');
        pushed_map.set(&right, '.');
        return true;
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let mut map = Vec::new();
    let mut pos = ivec2(0, 0);
    let mut map_max = ivec2(0, 0);

    for (y, line) in input[0].lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.push('#');
                map.push('#');
            } else if c == 'O' {
                map.push('[');
                map.push(']');
            } else {
                if c == '@' {
                    pos = ivec2(2 * x as i32, y as i32);
                }
                map.push('.');
                map.push('.');
            }
            map_max.x = map_max.x.max(2 * x as i32);
            map_max.y = map_max.y.max(y as i32);
        }
    }

    let mut map_a = Map::new(map, map_max.x + 2, map_max.y + 1);
    let mut map_b = Map::new(map_a.data.clone(), map_max.x + 2, map_max.y + 1);

    /*
    for y in 0..map_a.h {
        for x in 0..map_a.w {
            let p = ivec2(x, y);
            if pos == p {
                print!("@");
            } else {
                print!("{}", map_a.at(&p));
            }
        }
        println!();
    }
    println!();
    println!();
    */

    for c in input[1].lines().flat_map(|line| line.chars()) {
        let dir = match c {
            '<' => ivec2(-1, 0),
            '>' => ivec2(1, 0),
            '^' => ivec2(0, -1),
            'v' => ivec2(0, 1),
            _ => panic!(),
        };

        let maybe_pos = pos + dir;

        if maybe_push(&mut map_b, &dir, &maybe_pos) {
            pos = maybe_pos;
            map_a.clone_from(&map_b.data);
        } else {
            map_b.clone_from(&map_a.data);
        }

        /*
        for y in 0..map_a.h {
            for x in 0..map_a.w {
                let p = ivec2(x, y);
                if pos == p {
                    print!("@");
                } else {
                    print!("{}", map_a.at(&p));
                }
            }
            println!();
        }
        println!();
        println!();
        */
    }

    let mut res = 0;

    for y in 0..map_a.h {
        for x in 0..map_a.w {
            if map_a.at(&ivec2(x, y)) == '[' {
                res += 100 * y + x;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
