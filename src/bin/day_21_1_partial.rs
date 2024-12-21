fn get_sequence_len(dist: (i8, i8), depth: u8) -> usize {
    if depth == 0 {
        1
    } else {
        let dist_x = dist.0.abs();
        let dist_y = dist.1.abs();
        let mut pos = (0, 0);
        let pos1;
        0 + if dist_x > 0 {
            dist_x as usize - 1
                + get_sequence_len(
                    if dist.0 > 0 {
                        pos = (0, -1);
                        pos
                    } else {
                        pos = (-2, -1);
                        pos
                    },
                    depth - 1,
                )
        } else {
            0
        } + if dist_y > 0 {
            dist_y as usize - 1
                + get_sequence_len(
                    if dist.1 > 0 {
                        pos1 = (-1 - pos.0, 0 - pos.1);
                        pos = (pos.0 + pos1.0, pos.1 + pos1.1);
                        pos1
                    } else {
                        pos1 = (-1 - pos.0, -1 - pos.1);
                        pos = (pos.0 + pos1.0, pos.1 + pos1.1);
                        pos1
                    },
                    depth - 1,
                )
        } else {
            0
        } + get_sequence_len((-pos.0, -pos.1), depth - 1)
    }
}

fn solve(current_position: &mut i8, target: i8) -> usize {
    let dist = (
        (target % 3).abs() - (*current_position % 3).abs(),
        target / 3 - *current_position / 3,
    );
    *current_position = target;
    get_sequence_len(dist, 3)
}

fn main() {
    let codes: usize = include_str!("./day_21.input")
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
            println!("{num} * {a}");
            acc + num * a
        });
    println!("{}", codes);
    // println!("{}", solve(&mut position, -4));
    // position = -4;
    // println!("{}", solve(&mut position, 1));
    // position = 1;
    // println!("{}", solve(&mut position, 8));
    // position = 8;
    // println!("{}", solve(&mut position, -5));
    // for i in 0..8 {
    //     print!("A - {}", i + 1);
    //     solve(-5, i);
    //     print!("0 - {}", i + 1);
    //     solve(-4, i);
    // }
}
