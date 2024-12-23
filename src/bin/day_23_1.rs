use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    include_str!("./day_23.input")
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(a, b)| {
            let entry_a = connections.entry(a).or_insert(HashSet::new());
            entry_a.insert(b);
            let entry_b = connections.entry(b).or_insert(HashSet::new());
            entry_b.insert(a);
        });
    println!(
        "{:?}",
        connections
            .iter()
            .filter(|(key, _)| { key.starts_with("t") })
            .fold(HashSet::new(), |mut acc, connected| {
                connected.1.iter().combinations(2).for_each(|a_b| {
                    let a = a_b[0];
                    let b = a_b[1];
                    if connections
                        .get(a)
                        .map(|connected_to_a| connected_to_a.contains(b))
                        == Some(true)
                    {
                        acc.insert(
                            [*connected.0, *a, *b]
                                .into_iter()
                                .sorted()
                                .collect::<Vec<&str>>(),
                        );
                    }
                });
                acc
            })
            .len()
    );
}
