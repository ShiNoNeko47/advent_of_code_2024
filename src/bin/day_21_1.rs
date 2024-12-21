fn get_sequence_len(dist: (i8, i8), depth: u8, x_first: Option<bool>) -> usize {
    if depth == 0 {
        1
    } else {
        let dist_x = dist.0.abs();
        let dist_y = dist.1.abs();
        let mut possible_routes = vec![
            (
                (dist_x, [(0, -1), (-2, -1)], dist.0),
                (dist_y, [(-1, 0), (-1, -1)], dist.1),
            ),
            (
                (dist_y, [(-1, 0), (-1, -1)], dist.1),
                (dist_x, [(0, -1), (-2, -1)], dist.0),
            ),
        ];
        match x_first {
            Some(true) => {
                possible_routes.pop();
            }
            Some(false) => {
                possible_routes.remove(0);
            }
            _ => {}
        }
        possible_routes
            .iter()
            .map(
                |((dist_1, [dir_1, dir_2], axis_1), (dist_2, [dir_3, dir_4], axis_2))| {
                    let mut pos = (0, 0);
                    let pos1;
                    0 + if *dist_1 > 0 {
                        *dist_1 as usize - 1
                            + get_sequence_len(
                                if *axis_1 > 0 {
                                    pos = *dir_1;
                                    pos
                                } else {
                                    pos = *dir_2;
                                    pos
                                },
                                depth - 1,
                                None,
                            )
                    } else {
                        0
                    } + if *dist_2 > 0 {
                        *dist_2 as usize - 1
                            + get_sequence_len(
                                if *axis_2 > 0 {
                                    pos1 = (dir_3.0 - pos.0, dir_3.1 - pos.1);
                                    pos = (pos.0 + pos1.0, pos.1 + pos1.1);
                                    pos1
                                } else {
                                    pos1 = (dir_4.0 - pos.0, dir_4.1 - pos.1);
                                    pos = (pos.0 + pos1.0, pos.1 + pos1.1);
                                    pos1
                                },
                                depth - 1,
                                None,
                            )
                    } else {
                        0
                    } + get_sequence_len((-pos.0, -pos.1), depth - 1, None)
                },
            )
            .min()
            .unwrap()
    }
}

fn solve(current_position: &mut i8, target: i8) -> usize {
    let dist = (
        (target % 3).abs() - (*current_position % 3).abs(),
        target / 3 - *current_position / 3,
    );
    let x_first = if *current_position % 3 == 0 && target < 0 {
        Some(true)
    } else if *current_position < 0 && target % 3 == 0 {
        Some(false)
    } else {
        None
    };
    *current_position = target;
    get_sequence_len(dist, 3, x_first)
}

fn main() {
    println!(
        "{}",
        include_str!("./day_21.input")
            .lines()
            .map(|line| {
                (
                    line[..3].parse().unwrap_or(0),
                    line.chars()
                        .map(|c| match c {
                            '0' => -4,
                            'A' => -5,
                            _ => c.to_digit(10).unwrap() as i8 - 1,
                        })
                        .collect::<Vec<i8>>(),
                )
            })
            .fold(0, |acc, (num, parsed)| {
                let mut position: i8 = -5;
                let a = parsed
                    .iter()
                    .fold(0, |acc, x| acc + solve(&mut position, *x));
                acc + num * a
            })
    );
}
