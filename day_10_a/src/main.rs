use glam::{ivec2, IVec2};
use std::{error::Error, time::Instant};

struct Map<'a> {
    data: &'a [u8],
    w: i32,
    h: i32,
}

impl<'a> Map<'a> {
    fn new(data: &'a str) -> Self {
        let w = data.find('\n').unwrap() as i32;
        let h = data.len() as i32 / w - 1;

        Map {
            data: data.as_bytes(),
            w,
            h,
        }
    }

    fn at(&self, coord: IVec2) -> u8 {
        if coord.x < 0 || coord.x >= self.w || coord.y < 0 || coord.y >= self.h {
            return 255;
        }
        self.data[(coord.y * (self.w + 1) + coord.x) as usize] - 48
    }
}

fn is_path(map: &Map, pos: IVec2, next_level: u8, acc_res: &mut i32) -> bool {
    if next_level == 10 {
        return true;
    }

    for dir in [ivec2(-1, 0), ivec2(0, -1), ivec2(1, 0), ivec2(0, 1)] {
        if map.at(pos + dir) == next_level && is_path(map, pos + dir, next_level + 1) {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test");

    let map = Map::new(input);

    let mut res = 0;

    for y in 0..map.h {
        for x in 0..map.w {
            if map.at(ivec2(x, y)) == 0 && is_path(&map, ivec2(x, y), 1) {
                res += 1;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
