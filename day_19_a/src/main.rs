use std::{collections::BinaryHeap, error::Error, time::Instant};

use rustc_hash::FxHashMap;

#[derive(Eq, PartialEq)]
struct Pos {
    f_score: usize,
    pos: usize,
}

impl Pos {
    fn new(f_score: usize, pos: usize) -> Self {
        Pos { f_score, pos }
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// fn print_vec(v: &[u8]) -> String {
//     let mut res = String::new();
//
//     for v in v {
//         res = format!("{}{}", res, *v as char);
//     }
//     res
// }

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input_parts: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let words: Vec<Vec<u8>> = input_parts[0]
        .split(", ")
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();
    let sentences: Vec<Vec<u8>> = input_parts[1]
        .lines()
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();

    let mut res = 0;

    'sentence_loop: for sentence in sentences {
        let goal = sentence.len();

        let mut open_set = BinaryHeap::new();
        open_set.push(Pos::new(goal, 0));

        let mut g_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
        g_score.insert(0, 0);

        let mut f_score = FxHashMap::with_capacity_and_hasher(5000, Default::default());
        f_score.insert(9, goal);

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                res += 1;
                continue 'sentence_loop;
            }

            for word in &words {
                // println!(
                //     "{}: {} + maybe {}",
                //     print_vec(&sentence),
                //     print_vec(&sentence[0..current.pos]),
                //     print_vec(word)
                // );

                if current.pos + word.len() <= sentence.len()
                    && sentence[current.pos..current.pos + word.len()] == *word
                {
                    // println!("yes");

                    let tentative_g_score = g_score[&current.pos] + word.len();

                    if tentative_g_score
                        < *g_score
                            .entry(current.pos + word.len())
                            .or_insert(usize::MAX)
                    {
                        g_score.insert(current.pos + word.len(), tentative_g_score);
                        let neighbor_f_score = sentence.len() - (current.pos + word.len());
                        f_score.insert(current.pos + word.len(), neighbor_f_score);
                        open_set.push(Pos::new(neighbor_f_score, current.pos + word.len()));
                    }
                }
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
