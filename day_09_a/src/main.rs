use std::{error::Error, time::Instant};

fn print_d(d: &[i32]) {
    for d in d {
        if *d < 0 {
            print!(".")
        } else {
            print!("{d}")
        }
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut d: Vec<i32> = Vec::new();

    let mut is_file = true;
    let mut file_id_counter = 0;

    for c in input.chars() {
        let num = c as i32 - '0' as i32;
        if is_file {
            for _ in 0..num {
                d.push(file_id_counter);
            }
            file_id_counter += 1;
        } else {
            for _ in 0..num {
                d.push(-1);
            }
        }
        is_file = !is_file;
    }

    // print_d(&d);

    let mut cursor_start = 0;
    let mut cursor_end = d.len() - 1;
    while cursor_start < cursor_end {
        if d[cursor_start] < 0 && d[cursor_end] >= 0 {
            d[cursor_start] = d[cursor_end];
            d[cursor_end] = -1;
            cursor_start += 1;
            cursor_end -= 1;
        } else {
            if d[cursor_start] >= 0 {
                cursor_start += 1;
            }
            if d[cursor_end] < 0 {
                cursor_end -= 1;
            }
        }
        // print_d(&d);
    }

    let mut res = 0;

    for (i, n) in d.iter().enumerate() {
        if *n < 0 {
            break;
        }
        res += (i as i32 * n) as i64;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
