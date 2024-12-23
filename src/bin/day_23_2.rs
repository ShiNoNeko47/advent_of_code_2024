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
    let lan_party = connections
        .iter()
        .fold(vec![], |mut acc, connected| -> Vec<HashSet<&str>> {
            let mut new_lans_found: Vec<HashSet<&str>> = vec![];
            acc = acc
                .iter_mut()
                .map(|lan| {
                    if lan.iter().all(|comp| connected.1.contains(comp)) {
                        lan.insert(connected.0);
                    } else {
                        let mut new_lan = HashSet::from([*connected.0]);
                        lan.iter()
                            .filter(|comp| connected.1.contains(*comp))
                            .for_each(|comp| {
                                new_lan.insert(*comp);
                            });
                        if new_lan.len() > 1 {
                            new_lans_found.push(new_lan);
                        }
                    }
                    lan.clone()
                })
                .collect::<Vec<HashSet<&str>>>();
            acc.push(HashSet::from([*connected.0]));
            new_lans_found.iter().for_each(|lan| acc.push(lan.clone()));
            acc
        })
        .iter()
        .map(|lan| (lan.len(), lan))
        .max_by_key(|(len, _)| *len)
        .unwrap()
        .1
        .clone();
    let mut lan_party = lan_party.iter().collect::<Vec<&&str>>();
    lan_party.sort();
    println!("{}", lan_party.iter().join(","));
}
