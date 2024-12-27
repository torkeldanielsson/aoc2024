use std::{error::Error, time::Instant};

use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

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

    let mut cliques = FxHashSet::default();

    bron_kerbosch(
        &connections,
        &mut FxHashSet::default(),
        &mut computers,
        &mut FxHashSet::default(),
        &mut cliques,
    );

    let mut lan_party = Vec::new();

    for clique in &cliques {
        if clique.len() > lan_party.len() {
            lan_party = clique.clone();
        }
    }

    print!("res: ");
    for comp in lan_party {
        print!(
            "{}{},",
            (comp >> 8) as u8 as char,
            (comp & 0xFF) as u8 as char,
        );
    }
    println!(" {} us", t.elapsed().as_micros());

    Ok(())
}

fn bron_kerbosch(
    connections: &FxHashMap<u16, FxHashSet<u16>>,
    potential_clique: &mut FxHashSet<u16>, // R
    candidates: &mut FxHashSet<u16>,       // P
    excluded: &mut FxHashSet<u16>,         // X
    results: &mut FxHashSet<Vec<u16>>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        let mut clique: Vec<u16> = potential_clique.iter().copied().collect();
        clique.sort();
        results.insert(clique);
        return;
    }

    for candidate in candidates.clone().iter() {
        let mut new_clique = potential_clique.clone();
        new_clique.insert(*candidate);
        let candidate_neighbors = &connections[candidate];
        let mut candidate_neighbors_intersected_with_all_candidates = candidates
            .intersection(candidate_neighbors)
            .copied()
            .collect();
        let mut candidate_neighbors_intersected_with_excluded =
            candidates.intersection(excluded).copied().collect();

        bron_kerbosch(
            connections,
            &mut new_clique,
            &mut candidate_neighbors_intersected_with_all_candidates,
            &mut candidate_neighbors_intersected_with_excluded,
            results,
        );

        candidates.remove(candidate);
        excluded.insert(*candidate);
    }
}
