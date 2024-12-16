use std::collections::HashMap;

fn traverse(
    position: (i32, i32),
    path: &mut HashMap<(i32, i32), i32>,
    score: i32,
    front: (i32, i32),
    end: (i32, i32),
) -> Option<i32> {
    path.insert(position, score);
    if position == end {
        Some(score)
    } else {
        let (left, right) = if front.0 == 0 {
            ((1, 0, 1001), (-1, 0, 1001))
        } else {
            ((0, 1, 1001), (0, -1, 1001))
        };
        [(front.0, front.1, 1), left, right]
            .iter()
            .filter_map(|direction| {
                let next = (position.0 + direction.0, position.1 + direction.1);
                if path.get(&next).is_some_and(|s| s + 1001 > score || s == &0) {
                    traverse(
                        next,
                        path,
                        score + direction.2,
                        (direction.0, direction.1),
                        end,
                    )
                } else {
                    None
                }
            })
            .min()
    }
}
fn main() {
    let mut position = (0, 0);
    let mut end = (0, 0);
    let direction = (1, 0);
    let mut input: HashMap<(i32, i32), i32> = include_str!("./day_16.input")
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|line| {
            line.1
                .char_indices()
                .filter_map(|field| match field.1 {
                    '#' => None,
                    _ => {
                        if field.1 == 'S' {
                            position = (field.0 as i32, line.0 as i32);
                        } else if field.1 == 'E' {
                            end = (field.0 as i32, line.0 as i32);
                        }
                        Some((field.0 as i32, line.0 as i32))
                    }
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .fold(HashMap::new(), |mut map, field| {
            map.insert(field, 0);
            map
        });
    println!(
        "{:?}",
        traverse(position, &mut input, 0, direction, end).unwrap_or(0)
    );
}
