use std::cmp::Ordering;

fn main() {
    let mut input: Vec<Vec<i32>> = include_str!("./day_02.input")
        .split("\n")
        .filter(|x| x != &"")
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();
    let mut result = 0;
    for line in input.iter_mut() {
        let mut possible_problem: Option<usize> = None;
        for i in 0..4 {
            if i == 1 && Some(0) == possible_problem {
                continue;
            }
            let mut direction: Result<Option<Ordering>, ()> = Ok(None);
            let mut line = line.clone();
            if i > 0 {
                line.remove(possible_problem.unwrap() + i - 2);
            }
            let mut counter = 0;
            line.sort_by(|a, b| {
                match (a.cmp(b), direction) {
                    (Ordering::Equal, _) => {
                        if i == 0 && possible_problem.is_none() {
                            possible_problem = Some(counter);
                        }
                        direction = Err(());
                    }
                    (x, Ok(None)) => {
                        direction = Ok(Some(x));
                    }
                    (x, Ok(Some(y))) => {
                        if x != y {
                            if i == 0 && possible_problem.is_none() {
                                possible_problem = Some(counter);
                            }
                            direction = Err(());
                        }
                    }
                    (_, Err(_)) => (),
                };
                if a.abs_diff(*b) > 3 {
                    if i == 0 && possible_problem.is_none() {
                        possible_problem = Some(counter);
                    }
                    direction = Err(())
                };
                counter += 1;
                Ordering::Greater
            });
            if direction.is_ok() {
                result += 1;
                break;
            }
        }
    }
    println!("{:?}", result);
}
