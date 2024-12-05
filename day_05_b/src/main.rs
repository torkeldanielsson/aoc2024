use std::{
    collections::{HashMap, HashSet},
    error::Error,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules = HashMap::new();

    for rule_line in parts[0].lines() {
        let rule: Vec<i32> = rule_line
            .split("|")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        rules
            .entry(rule[0])
            .or_insert_with(HashSet::new)
            .insert(rule[1]);
    }

    let mut res = 0;

    for update in parts[1].lines() {
        let mut update: Vec<i32> = update
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut needed_sorting = false;
        let mut do_loop = true;

        while do_loop {
            do_loop = false;
            'sort: for i in 1..update.len() {
                if rules.contains_key(&update[i]) {
                    let rule = &rules[&update[i]];
                    for j in 0..i {
                        if rule.contains(&update[j]) {
                            do_loop = true;
                            needed_sorting = true;
                            update.swap(i, j);
                            continue 'sort;
                        }
                    }
                }
            }
        }

        if needed_sorting {
            res += update[update.len() / 2];
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
