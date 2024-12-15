use std::collections::HashMap;

fn robot_move(
    coords: (i32, i32),
    velocity: (i32, i32),
    map: &mut HashMap<(i32, i32), char>,
) -> bool {
    let coords_in_direction = (coords.0 + velocity.0, coords.1 + velocity.1);
    match map.get(&coords_in_direction).unwrap_or(&'#') {
        '#' => false,
        '.' => {
            map.insert(coords_in_direction, *map.get(&coords).unwrap());
            true
        }
        _ => robot_move(coords_in_direction, velocity, map),
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
                        robot = (line.0 as i32, field.0 as i32);
                        ((field.0 as i32, line.0 as i32), '.')
                    } else {
                        ((field.0 as i32, line.0 as i32), field.1)
                    }
                })
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
            if robot_move(robot, velocity, &mut map) {
                robot = (robot.0 + velocity.0, robot.1 + velocity.1);
                map.insert(robot, '.');
            }
        });
    println!(
        "{}",
        map.iter()
            .filter(|(_, v)| { v == &&'O' })
            .fold(0, |acc, (coords, _)| acc + 100 * coords.1 + coords.0)
    );
}
