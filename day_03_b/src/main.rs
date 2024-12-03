use std::{error::Error, time::Instant};

enum State {
    Init,
    Mul,
    Num1,
    Num2,
    Do,
    Dont,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input");

    let t = Instant::now();

    let mut state = State::Init;

    let mut i = 0;
    let mut num_str = String::new();

    let mut num1 = 0;

    let mut mul_enabled = true;

    let mut res = 0;

    for c in input.chars() {
        match state {
            State::Init => {
                if c == 'm' && mul_enabled {
                    state = State::Mul;
                    i = 1;
                }
                if c == 'd' {
                    state = State::Do;
                    i = 1;
                }
            }
            State::Mul => {
                if i == 1 && c == 'u' {
                    i = 2;
                } else if i == 2 && c == 'l' {
                    i = 3;
                } else if i == 3 && c == '(' {
                    i = 0;
                    num_str = String::new();
                    state = State::Num1;
                } else {
                    state = State::Init;
                }
            }
            State::Num1 => {
                if c.is_ascii_digit() {
                    i += 1;
                    num_str.push(c);
                } else if c == ',' && i > 0 && i <= 3 {
                    num1 = num_str.parse().unwrap();
                    i = 0;
                    num_str = String::new();
                    state = State::Num2;
                } else {
                    state = State::Init;
                }
            }
            State::Num2 => {
                if c.is_ascii_digit() {
                    i += 1;
                    num_str.push(c);
                } else if c == ')' && i > 0 && i <= 3 {
                    let num2: i32 = num_str.parse().unwrap();
                    res += num1 * num2;
                    num_str = String::new();
                    state = State::Init;
                } else {
                    state = State::Init;
                }
            }
            State::Do => {
                if i == 1 && c == 'o' {
                    i = 2;
                } else if i == 2 && c == '(' {
                    i = 3;
                } else if i == 2 && c == 'n' {
                    state = State::Dont;
                    i = 3;
                } else if i == 3 && c == ')' {
                    mul_enabled = true;
                    state = State::Init;
                } else {
                    state = State::Init;
                }
            }
            State::Dont => {
                if i == 3 && c == '\'' {
                    i = 4;
                } else if i == 4 && c == 't' {
                    i = 5;
                } else if i == 5 && c == '(' {
                    i = 6;
                } else if i == 6 && c == ')' {
                    mul_enabled = false;
                    state = State::Init;
                } else {
                    state = State::Init;
                }
            }
        }
    }

    println!("res: {res}, time: {} us", t.elapsed().as_micros());

    Ok(())
}
