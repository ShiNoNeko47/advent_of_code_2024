use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve(
    racetrack: &mut HashMap<(usize, usize), Option<usize>>,
    position: (usize, usize),
    idx: usize,
) -> usize {
    racetrack.insert(position, Some(idx));
    DIRECTIONS
        .iter()
        .find_map(|direction| {
            if Some(&None)
                == racetrack.get(&(
                    (position.0 as i32 + direction.0) as usize,
                    (position.1 as i32 + direction.1) as usize,
                ))
            {
                Some(solve(
                    racetrack,
                    (
                        (position.0 as i32 + direction.0) as usize,
                        (position.1 as i32 + direction.1) as usize,
                    ),
                    idx + 1,
                ))
            } else {
                None
            }
        })
        .unwrap_or(0)
        + DIRECTIONS
            .iter()
            .filter(|direction| {
                racetrack
                    .get(&(
                        (position.0 as i32 + direction.0 * 2) as usize,
                        (position.1 as i32 + direction.1 * 2) as usize,
                    ))
                    .unwrap_or(&None)
                    .unwrap_or(idx)
                    + 102
                    <= idx
            })
            .count()
}

fn main() {
    let mut start = (0, 0);
    let mut racetrack: HashMap<(usize, usize), Option<usize>> = include_str!("./day_20.input")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '#' => None,
                _ => Some((x, y, c)),
            })
        })
        .fold(HashMap::new(), |mut acc, (x, y, c)| {
            if c == 'S' {
                start = (x, y);
            }
            acc.insert((x, y), None);
            acc
        });
    println!("{}", solve(&mut racetrack, start, 0));
}
