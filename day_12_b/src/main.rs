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
        let h = data.chars().filter(|&c| c == '\n').count() as i32;

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
    borders: &mut Vec<(IVec2, IVec2)>,
) {
    for neighbor in [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)] {
        let maybe = coord + neighbor;
        if !contiguous_set.contains(&maybe) {
            if map.at(&maybe) == plant {
                contiguous_set.insert(maybe);
                glob(map, contiguous_set, plant, &maybe, borders);
            } else {
                borders.push((*coord, maybe));
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

    println!("map dim: {}x{}", map.h, map.w);

    for y in 0..map.h {
        for x in 0..map.w {
            let coord = ivec2(x, y);
            if !processed_coords.contains(&coord) {
                let mut contiguos_set = HashSet::new();
                contiguos_set.insert(coord);
                let mut borders = Vec::new();
                glob(
                    &map,
                    &mut contiguos_set,
                    map.at(&coord),
                    &coord,
                    &mut borders,
                );

                let mut processed_border_indices = Vec::new();

                let mut border_count = 0;

                for i in 0..borders.len() {
                    if !processed_border_indices.contains(&i) {
                        let origin_border = borders[i];
                        let border_dir = if origin_border.0.x == origin_border.1.x {
                            ivec2(1, 0)
                        } else {
                            ivec2(0, 1)
                        };
                        processed_border_indices.push(i);

                        let mut test_pos =
                            (origin_border.0 + border_dir, origin_border.1 + border_dir);

                        // println!("searching for {test_pos:?} in {borders:?}");
                        while let Some(index) = borders
                            .iter()
                            .position(|&p| p.0 == test_pos.0 && p.1 == test_pos.1)
                        {
                            test_pos = (test_pos.0 + border_dir, test_pos.1 + border_dir);
                            processed_border_indices.push(index);
                        }

                        test_pos = (origin_border.0 - border_dir, origin_border.1 - border_dir);
                        // println!("and searching for {test_pos:?} (origin: {origin_border:?})");

                        while let Some(index) = borders
                            .iter()
                            .position(|&p| p.0 == test_pos.0 && p.1 == test_pos.1)
                        {
                            test_pos = (test_pos.0 - border_dir, test_pos.1 - border_dir);
                            processed_border_indices.push(index);
                        }

                        border_count += 1;

                        // for y in -1..map.h + 1 {
                        //     'x_loop: for x in -1..map.w + 1 {
                        //         for i in 0..borders.len() {
                        //             if borders[i].1.x == x
                        //                 && borders[i].1.y == y
                        //                 && processed_border_indices.contains(&i)
                        //             {
                        //                 print!("+");
                        //                 continue 'x_loop;
                        //             }
                        //         }
                        //
                        //         if map.at(&ivec2(x, y)) == map.at(&coord) {
                        //             print!("{}", map.at(&coord) as char);
                        //         } else {
                        //             print!(".");
                        //         }
                        //     }
                        //     println!();
                        // }
                        // println!();
                        // println!();
                    }
                }

                //  println!(
                //      "{} {} {}",
                //      map.at(&coord) as char,
                //      contiguos_set.len(),
                //      border_count
                //  );

                res += contiguos_set.len() as i32 * border_count;
                processed_coords.extend(contiguos_set);
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
