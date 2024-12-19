use std::collections::HashMap;

fn check_possible(target: &str, available: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if target.is_empty() {
        1
    } else {
        if let Some(val) = memo.get(target) {
            *val
        } else {
            let result = available.iter().fold(0, |acc, pattern| {
                if !target.starts_with(*pattern) {
                    acc
                } else {
                    acc + check_possible(&target[pattern.len()..], &available, memo)
                }
            });
            memo.insert(target.to_string(), result);
            result
        }
    }
}

fn main() {
    let (available, targets) = include_str!("./day_19.input").split_once("\n\n").unwrap();
    let mut memo = HashMap::new();
    println!(
        "{}",
        targets.lines().fold(0, |acc, target| {
            let available: Vec<&str> = available
                .split(", ")
                .filter(|pattern| target.contains(pattern))
                .collect();
            acc + check_possible(target, &available, &mut memo)
        })
    )
}
