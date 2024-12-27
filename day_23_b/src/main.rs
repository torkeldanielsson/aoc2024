use std::{error::Error, time::Instant};

use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test");

    let mut connections = FxHashMap::default();
    let mut computers = FxHashSet::default();

    for line in input.lines() {
        let line_bytes: &[u8] = line.as_bytes();
        let comp_1 = ((line_bytes[0] as u16) << 8) | (line_bytes[1] as u16);
        let comp_2 = ((line_bytes[3] as u16) << 8) | (line_bytes[4] as u16);

        connections
            .entry(comp_1)
            .or_insert(FxHashSet::default())
            .insert(comp_2);
        connections
            .entry(comp_2)
            .or_insert(FxHashSet::default())
            .insert(comp_1);
        computers.insert(comp_1);
        computers.insert(comp_2);
    }

    let mut largest_lan_party = FxHashSet::default();

    for computer_1 in &computers {
        let connections_1 = &connections[computer_1];
        let mut lan_party = connections[computer_1].clone();
        lan_party.insert(*computer_1);

        println!(
            "checking computer {}{}",
            (computer_1 >> 8) as u8 as char,
            (computer_1 & 0xFF) as u8 as char,
        );

        print!("OG lan party: ");
        for comp in &lan_party {
            print!(
                "{}{}, ",
                (comp >> 8) as u8 as char,
                (comp & 0xFF) as u8 as char,
            );
        }
        println!();

        for computer_2 in connections_1 {
            let mut lan_party_2 = connections[computer_2].clone();
            lan_party_2.insert(*computer_2);

            print!("lan_party_2: ");
            for comp in &lan_party_2 {
                print!(
                    "{}{}, ",
                    (comp >> 8) as u8 as char,
                    (comp & 0xFF) as u8 as char,
                );
            }
            println!();

            lan_party = lan_party.intersection(&lan_party_2).copied().collect();

            print!("intersected lan party: ");
            for comp in &lan_party {
                print!(
                    "{}{}, ",
                    (comp >> 8) as u8 as char,
                    (comp & 0xFF) as u8 as char,
                );
            }
            println!();
        }
        if lan_party.len() > largest_lan_party.len() {
            largest_lan_party = lan_party;
        }
    }

    println!(
        "largest_lan_party: {largest_lan_party:?}, {} us",
        t.elapsed().as_micros()
    );

    //  println!("res: {}, {} us", t_triplets.len(), t.elapsed().as_micros());

    Ok(())
}

fn bron_kerbosch(
    graph: &FxHashMap<i32, FxHashSet<i32>>,
    potential_clique: &mut FxHashSet<i32>, // R
    candidates: &mut FxHashSet<i32>,       // P
    excluded: &mut FxHashSet<i32>,         // X
    results: &mut Vec<FxHashSet<i32>>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        results.push(potential_clique.clone());
        return;
    }

    for candidate in candidates.iter() {
        let mut new_clique = potential_clique.clone();
        new_clique.insert(*candidate);
        let candidate_neighbors = &graph[candidate];
        let mut candidate_neighbors_intersected_with_all_candidates = candidates
            .intersection(candidate_neighbors)
            .copied()
            .collect();
        let mut candidate_neighbors_intersected_with_excluded =
            candidates.intersection(excluded).copied().collect();

        bron_kerbosch(
            graph,
            &mut new_clique,
            &mut candidate_neighbors_intersected_with_all_candidates,
            &mut candidate_neighbors_intersected_with_excluded,
            results,
        );
    }
}
