use std::cmp::Ordering;

fn main() {
    let input = include_str!("./day_05.input").split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = input
        .0
        .split("\n")
        .filter_map(|line| {
            if let Some((a, b)) = line.split_once("|") {
                Some((a.parse().unwrap_or(0), b.parse().unwrap_or(0)))
            } else {
                None
            }
        })
        .collect();
    println!(
        "{}",
        input
            .1
            .split("\n")
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(line.split(",").map(|n| n.parse().unwrap_or(0)).collect())
                }
            })
            .fold(0, |acc, mut page: Vec<usize>| {
                let rules = rules
                    .iter()
                    .filter(|rule| page.contains(&rule.0) && page.contains(&rule.1))
                    .collect::<Vec<&(usize, usize)>>();
                if page.iter().enumerate().any(|(idx, n)| {
                    rules
                        .iter()
                        .filter(|m| *n == m.0)
                        .any(|m| !page[idx..].contains(&m.1))
                }) {
                    page.sort_by(|a, b| {
                        if !rules.contains(&&(*a, *b)) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    });
                    acc + page[page.len() / 2]
                } else {
                    acc
                }
            })
    );
}
