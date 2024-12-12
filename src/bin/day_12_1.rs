use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<(usize, usize)>> = include_str!("./day_12.input")
        .split("\n")
        .enumerate()
        .map(|(y, row)| {
            row.char_indices()
                .map(move |(x, c)| (x, y, c))
                .fold::<Vec<(char, Vec<(usize, usize)>)>, _>(vec![], |mut acc, c| {
                    if let Some(last_vec) = acc.last_mut() {
                        if last_vec.0 == c.2 {
                            last_vec.1.push((c.0 + 1, c.1 + 1));
                        } else {
                            acc.push((c.2, vec![(c.0 + 1, c.1 + 1)]));
                        }
                    } else {
                        acc.push((c.2, vec![(c.0 + 1, c.1 + 1)]));
                    }
                    acc
                })
        })
        .fold::<HashMap<char, Vec<Vec<(usize, usize)>>>, _>(
            HashMap::new(),
            |mut acc, row_divided| {
                row_divided.iter().for_each(|part| {
                    let mut areas_connected = vec![part.1.clone()];
                    let areas = acc.entry(part.0).or_insert(vec![]);
                    areas.iter().for_each(|area| {
                        if area.iter().any(|position| {
                            areas_connected[0].contains(&(position.0, position.1 + 1))
                        }) {
                            area.iter().for_each(|node| areas_connected[0].push(*node));
                        } else {
                            areas_connected.push(area.to_vec());
                        }
                    });
                    acc.insert(part.0, areas_connected);
                });
                acc
            },
        )
        .into_values()
        .flatten()
        .collect();
    println!(
        "{:#?}",
        input.iter().fold(0, |acc, area| {
            let mut perimeter = 0;
            area.iter().for_each(|node| {
                [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().for_each(|dir| {
                    if !area.contains(&(
                        (node.0 as isize + dir.0) as usize,
                        (node.1 as isize + dir.1) as usize,
                    )) {
                        perimeter += 1;
                    }
                })
            });
            acc + perimeter * area.len()
        })
    );
}
