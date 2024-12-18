use std::collections::HashMap;

fn traverse(
    position: (i32, i32),
    path: &mut HashMap<(i32, i32), i32>,
    corrupted: &Vec<(i32, i32)>,
    score: i32,
    end: (i32, i32),
) -> Option<i32> {
    path.insert(position, score);
    if position == end {
        Some(score)
    } else {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(|direction| {
                let next = (position.0 + direction.0, position.1 + direction.1);
                if !corrupted.contains(&next)
                    && path.get(&next).unwrap_or(&(score + 2)) - 1 > score
                    && next.0 <= end.0
                    && next.1 <= end.1
                    && next.0 >= 0
                    && next.1 >= 0
                {
                    traverse(next, path, corrupted, score + 1, end)
                } else {
                    None
                }
            })
            .min()
    }
}
fn main() {
    let position = (0, 0);
    let end = (70, 70);
    let mut path: HashMap<(i32, i32), i32> = HashMap::new();
    let corrupted: Vec<(i32, i32)> = include_str!("./day_18.input")
        .split("\n")
        .map(|line| {
            let coords = line.split_once(",").unwrap_or(("0", "0"));
            (coords.0.parse().unwrap(), coords.1.parse().unwrap())
        })
        .take(1024)
        .collect();
    println!(
        "{:?}",
        traverse(position, &mut path, &corrupted, 0, end).unwrap_or(0)
    );
}
