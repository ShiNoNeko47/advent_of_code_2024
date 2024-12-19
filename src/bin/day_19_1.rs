fn check_possible(target: &str, available: &Vec<&str>) -> bool {
    if target.is_empty() {
        true
    } else {
        available
            .iter()
            .filter(|pattern| {
                target.starts_with(*pattern) && check_possible(&target[pattern.len()..], &available)
            })
            .next()
            .is_some()
    }
}

fn main() {
    let (available, targets) = include_str!("./day_19.input").split_once("\n\n").unwrap();
    let available: Vec<&str> = available.split(", ").collect();
    println!(
        "{}",
        targets.lines().fold(0, |acc, target| {
            if check_possible(target, &available) {
                acc + 1
            } else {
                acc
            }
        })
    )
}
