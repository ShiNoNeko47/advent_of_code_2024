use std::collections::HashMap;

fn main() {
    let mut width = 0;
    let mut row = 0;
    let input: HashMap<(i32, i32), usize> = include_str!("./day_10.input")
        .chars()
        .enumerate()
        .filter_map(|(idx, height)| {
            if height == '\n' {
                if width == 0 {
                    width = idx + 1;
                }
                row += 1;
            }
            height
                .to_string()
                .parse()
                .ok()
                .map(|height| (((idx - width * row) as i32, row as i32), height))
        })
        .fold(HashMap::new(), |mut acc, (coords, height)| {
            acc.insert(coords, height);
            acc
        });
    println!(
        "{}",
        input
            .iter()
            .filter(|(_, height)| height == &&0)
            .fold(0, |acc, (coords, _)| {
                let mut paths: Vec<(i32, i32)> = vec![*coords];
                for i in 1..=9 {
                    paths = paths.iter().fold(vec![], |mut acc, (x, y)| {
                        [(0, 1), (0, -1), (1, 0), (-1, 0)]
                            .iter()
                            .for_each(|(xx, yy)| {
                                if input.get(&(x + xx, y + yy)) == Some(&i) {
                                    acc.push((x + xx, y + yy));
                                }
                            });
                        acc
                    });
                }
                acc + paths.len()
            })
    );
}
