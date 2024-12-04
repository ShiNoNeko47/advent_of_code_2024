use std::char;

fn main() {
    const TARGET: [char; 3] = ['M', 'A', 'S'];
    const TARGET_R: [char; 3] = ['S', 'A', 'M'];
    let input: Vec<char> = include_str!("./day_04.input").chars().collect();
    let width = input.iter().filter(|c| **c == '\n').count();
    println!(
        "{} ",
        input.iter().enumerate().fold(0, |acc, (idx, _)| {
            acc + [TARGET, TARGET_R].iter().fold(0, |acc, target| {
                let mut first_last = vec![TARGET[0], TARGET[2]];
                acc + if input
                    .iter()
                    .skip(idx)
                    .step_by(width + 2)
                    .take(TARGET.len())
                    .enumerate()
                    .take_while(|(idx, c)| **c != '\n' && **c == target[*idx])
                    .count()
                    == TARGET.len()
                    && input
                        .iter()
                        .skip(idx + 2)
                        .step_by(width * 2)
                        .take(2)
                        .all(|c| {
                            if c == &first_last[0] {
                                first_last.remove(0);
                                true
                            } else if first_last.len() == 2 && c == &first_last[1] {
                                first_last.remove(1);
                                true
                            } else {
                                false
                            }
                        })
                {
                    1
                } else {
                    0
                }
            })
        })
    );
}
