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
                    [(0, 1), (1, 0)].iter().for_each(|dir| {
                        let mut count = 0;
                        loop {
                            let pos = (
                                c[dir.0].0 as i32 + (c[dir.0].0 as i32 - c[dir.1].0 as i32) * count,
                                c[dir.0].1 as i32 + (c[dir.0].1 as i32 - c[dir.1].1 as i32) * count,
                            );
                            count += 1;
                            if pos.0 >= 0
                                && pos.1 >= 0
                                && pos.0 < width as i32 - 1
                                && pos.1 < row as i32
                            {
                                acc.insert(pos);
                            } else {
                                break;
                            }
                        }
                    });
                });
                acc
            })
            .iter()
            .count()
    );
}
