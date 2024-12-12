use std::{collections::HashSet, error::Error, time::Instant};

use glam::{ivec2, IVec2};

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

    fn at(&self, coord: &IVec2) -> u8 {
        if coord.x < 0 || coord.x >= self.w || coord.y < 0 || coord.y >= self.h {
            return 255;
        }
        self.data[(coord.y * (self.w + 1) + coord.x) as usize]
    }
}

fn glob(
    map: &Map,
    contiguous_set: &mut HashSet<IVec2>,
    plant: u8,
    coord: &IVec2,
    border_len: &mut i32,
) {
    for neighbor in [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)] {
        let maybe = coord + neighbor;
        if !contiguous_set.contains(&maybe) {
            if map.at(&maybe) == plant {
                contiguous_set.insert(maybe);
                glob(map, contiguous_set, plant, &maybe, border_len);
            } else {
                *border_len += 1;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");
    let map = Map::new(input);

    let mut processed_coords: HashSet<IVec2> = HashSet::new();

    let mut res = 0;

    for y in 0..map.h {
        for x in 0..map.w {
            let coord = ivec2(x, y);
            if !processed_coords.contains(&coord) {
                let mut contiguos_set = HashSet::new();
                contiguos_set.insert(coord);
                let mut border_len = 0;
                glob(
                    &map,
                    &mut contiguos_set,
                    map.at(&coord),
                    &coord,
                    &mut border_len,
                );
                // println!(
                //     "{}: {} * {} => {}",
                //     map.at(&coord) as char,
                //     contiguos_set.len(),
                //     border_len,
                //     contiguos_set.len() as i32 * border_len,
                // );
                res += contiguos_set.len() as i32 * border_len;
                processed_coords.extend(contiguos_set);
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
