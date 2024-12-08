use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let mut width = 0;
    let mut row = 0;
    let input: Vec<(char, Vec<(usize, usize)>)> = include_str!("./day_08.input")
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, c| {
            if c.1 == '\n' {
                if width == 0 {
                    width = c.0 + 1;
                }
                row += 1;
                acc
            } else if c.1 == '.' {
                acc
            } else {
                if let Some(idx) = acc.iter().position(|freq| freq.0 == c.1) {
                    acc[idx].1.push((c.0 - width * row, row));
                    acc
                } else {
                    acc.push((c.1, vec![(c.0 - width * row, row)]));
                    acc
                }
            }
        });
    println!(
        "{:#?}",
        input
            .iter()
            .fold(HashSet::new(), |mut acc, (_, positions)| {
                positions.iter().combinations(2).for_each(|c| {
                    [
                        (
                            c[0].0 as i32 + (c[0].0 as i32 - c[1].0 as i32),
                            c[0].1 as i32 + (c[0].1 as i32 - c[1].1 as i32),
                        ),
                        (
                            c[1].0 as i32 + (c[1].0 as i32 - c[0].0 as i32),
                            c[1].1 as i32 + (c[1].1 as i32 - c[0].1 as i32),
                        ),
                    ]
                    .into_iter()
                    .filter(|pos| {
                        pos.0 >= 0 && pos.1 >= 0 && pos.0 < width as i32 - 1 && pos.1 < row as i32
                    })
                    .for_each(|position| {
                        acc.insert(position);
                    });
                });
                acc
            })
            .iter()
            .count()
    );
}
