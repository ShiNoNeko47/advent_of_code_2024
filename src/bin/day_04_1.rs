use std::char;

fn main() {
    const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];
    const TARGET_R: [char; 4] = ['S', 'A', 'M', 'X'];
    let input: Vec<Vec<char>> = include_str!("./day_04.input")
        .split("\n")
        .filter(|row| row.len() > 0)
        .map(|row| (row.to_owned() + "0").chars().collect())
        .collect();
    let width = input[0].len();
    println!(
        "{}",
        input.iter().flatten().enumerate().fold(0, |acc, (idx, _)| {
            acc + [TARGET, TARGET_R].iter().fold(0, |acc, target| {
                acc + [1, width + 1, width, width - 1].iter().fold(0, |acc, dir| {
                    acc + if input
                        .iter()
                        .flatten()
                        .skip(idx)
                        .step_by(*dir)
                        .take_while(|c| **c != '0')
                        .take(target.len())
                        .enumerate()
                        .take_while(|(idx, c)| **c == target[*idx])
                        .count()
                        == target.len()
                    {
                        1
                    } else {
                        0
                    }
                })
            })
        })
    );
}
