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

    'outer: for update in parts[1].lines() {
        let update: Vec<i32> = update
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for i in 1..update.len() {
            if rules.contains_key(&update[i]) {
                let rule = &rules[&update[i]];
                for j in update.iter().take(i) {
                    if rule.contains(j) {
                        continue 'outer;
                    }
                }
            }
        }

        res += update[update.len() / 2];
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
