use std::{error::Error, time::Instant};

fn _print_d(d: &[(i32, i32)]) {
    for (d, n) in d {
        for _ in 0..*n {
            if *d < 0 {
                print!(".")
            } else {
                print!("{d}")
            }
        }
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut d: Vec<(i32, i32)> = Vec::new();

    let mut is_file = true;
    let mut file_id_counter = 0;

    for c in input.chars() {
        let num = c as i32 - '0' as i32;
        if is_file {
            d.push((file_id_counter, num));
            file_id_counter += 1;
        } else {
            d.push((-1, num));
        }
        is_file = !is_file;
    }

    // print_d(&d);

    let mut cursor_end = d.len() - 1;
    let mut cursor_start_start = 0;

    while file_id_counter > 0 {
        file_id_counter -= 1;
        while d[cursor_end].0 != file_id_counter {
            cursor_end -= 1;
        }

        while d[cursor_start_start].0 >= 0 {
            cursor_start_start += 1;
        }

        let mut cursor_start = cursor_start_start;

        loop {
            while d[cursor_start].0 >= 0 {
                cursor_start += 1;
            }

            if cursor_start >= cursor_end {
                break;
            }

            match d[cursor_end].1.cmp(&d[cursor_start].1) {
                std::cmp::Ordering::Equal => {
                    d[cursor_start].0 = d[cursor_end].0;
                    d[cursor_end].0 = -1;
                    break;
                }
                std::cmp::Ordering::Less => {
                    d.insert(cursor_start, d[cursor_end]);
                    d[cursor_start + 1].1 -= d[cursor_start].1;
                    d[cursor_end + 1].0 = -1;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    cursor_start += 1;
                }
            }
        }
        // print_d(&d);
    }

    let mut res = 0;
    let mut i = 0;

    for n in d {
        for _ in 0..n.1 {
            if n.0 >= 0 {
                res += (i * n.0) as i64;
            }
            i += 1;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
