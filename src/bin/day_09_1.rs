fn main() {
    let mut input: Vec<(usize, usize)> = include_str!("./day_09.input")
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .enumerate()
        .collect();

    if (input.last().unwrap().0) % 2 != 0 {
        input.pop(); // just in case
    };
    let mut input_iter_r = input.iter().step_by(2).rev().peekable();
    let mut left_at_end = input_iter_r.peek().unwrap().1;
    let space_taken = input.iter().step_by(2).map(|(_, size)| size).sum();
    println!(
        "{}",
        input
            .iter()
            .fold((0, 0), |mut acc, (idx, size)| {
                if acc.0 > space_taken {
                    acc
                } else {
                    for _ in 0..*size {
                        if acc.0 == space_taken {
                            break;
                        }
                        if idx % 2 == 0 {
                            acc.1 += idx / 2 * acc.0;
                        } else {
                            acc.1 += input_iter_r.peek().unwrap().0 / 2 * acc.0;
                            left_at_end -= 1;
                            if left_at_end == 0 {
                                input_iter_r.next();
                                left_at_end = input_iter_r.peek().unwrap().1;
                            }
                        }
                        acc.0 += 1;
                    }
                    acc
                }
            })
            .1
    );
}
