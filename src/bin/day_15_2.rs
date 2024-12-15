use std::collections::HashMap;

fn check_move(coords: (i32, i32), velocity: (i32, i32), map: &HashMap<(i32, i32), char>) -> bool {
    let coords_in_direction = (coords.0 + velocity.0, coords.1 + velocity.1);
    match map.get(&coords_in_direction).unwrap_or(&'#') {
        '#' => false,
        '[' => {
            check_move(coords_in_direction, velocity, map)
                && check_move(
                    (coords_in_direction.0 + 1, coords_in_direction.1),
                    velocity,
                    map,
                )
        }
        ']' => {
            check_move(coords_in_direction, velocity, map)
                && check_move(
                    (coords_in_direction.0 - 1, coords_in_direction.1),
                    velocity,
                    map,
                )
        }
        _ => true,
    }
}

fn robot_move(
    coords: (i32, i32),
    velocity: (i32, i32),
    map: &mut HashMap<(i32, i32), char>,
    vertical_can_move: Option<bool>,
) -> bool {
    let coords_in_direction = (coords.0 + velocity.0, coords.1 + velocity.1);
    if velocity.0 != 0 {
        match map.get(&coords_in_direction).unwrap_or(&'#') {
            '#' => false,
            '.' => {
                map.insert(coords_in_direction, *map.get(&coords).unwrap());
                true
            }
            _ => {
                if robot_move(coords_in_direction, velocity, map, None) {
                    map.insert(coords_in_direction, *map.get(&coords).unwrap());
                    true
                } else {
                    false
                }
            }
        }
    } else {
        match vertical_can_move {
            None => robot_move(
                coords,
                velocity,
                map,
                Some(check_move(coords, velocity, map)),
            ),
            Some(false) => false,
            Some(true) => {
                match map.get(&coords_in_direction).unwrap_or(&'#') {
                    '[' => {
                        robot_move(coords_in_direction, velocity, map, Some(true));
                        robot_move(
                            (coords_in_direction.0 + 1, coords_in_direction.1),
                            velocity,
                            map,
                            Some(true),
                        );
                    }
                    ']' => {
                        robot_move(coords_in_direction, velocity, map, Some(true));
                        robot_move(
                            (coords_in_direction.0 - 1, coords_in_direction.1),
                            velocity,
                            map,
                            Some(true),
                        );
                    }
                    _ => {}
                }
                map.insert(coords_in_direction, *map.get(&coords).unwrap());
                map.insert(coords, '.');
                true
            }
        }
    }
}

fn main() {
    let mut robot: (i32, i32) = (0, 0);
    let (map, directions) = include_str!("./day_15.input").split_once("\n\n").unwrap();
    let mut map: HashMap<(i32, i32), char> = map
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|line| {
            line.1
                .char_indices()
                .map(|field| {
                    if field.1 == '@' {
                        robot = (field.0 as i32 * 2, line.0 as i32);
                        [
                            ((field.0 as i32 * 2, line.0 as i32), '.'),
                            ((field.0 as i32 * 2 + 1, line.0 as i32), '.'),
                        ]
                    } else if field.1 == 'O' {
                        [
                            ((field.0 as i32 * 2, line.0 as i32), '['),
                            ((field.0 as i32 * 2 + 1, line.0 as i32), ']'),
                        ]
                    } else {
                        [
                            ((field.0 as i32 * 2, line.0 as i32), field.1),
                            ((field.0 as i32 * 2 + 1, line.0 as i32), field.1),
                        ]
                    }
                })
                .flatten()
                .collect::<Vec<((i32, i32), char)>>()
        })
        .fold(HashMap::new(), |mut map, field| {
            map.insert(field.0, field.1);
            map
        });

    directions
        .trim()
        .replace("\n", "")
        .chars()
        .for_each(|direction| {
            let velocity = match direction {
                '>' => (1, 0),
                '<' => (-1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => {
                    panic!("Invalid direction")
                }
            };
            if robot_move(robot, velocity, &mut map, None) {
                robot = (robot.0 + velocity.0, robot.1 + velocity.1);
                map.insert(robot, '.');
            }
        });

    println!(
        "{}",
        map.iter()
            .filter(|(_, v)| { v == &&'[' })
            .fold(0, |acc, (coords, _)| acc + 100 * coords.1 + coords.0)
    );
}
