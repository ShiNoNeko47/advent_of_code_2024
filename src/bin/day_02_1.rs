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
        let mut direction: Result<Option<Ordering>, ()> = Ok(None);
        line.sort_by(|a, b| {
            match (a.cmp(b), direction) {
                (Ordering::Equal, _) => {
                    direction = Err(());
                }
                (x, Ok(None)) => {
                    direction = Ok(Some(x));
                }
                (x, Ok(Some(y))) => {
                    if x != y {
                        direction = Err(());
                    }
                }
                (_, Err(_)) => (),
            };
            if a.abs_diff(*b) > 3 {
                direction = Err(())
            };
            Ordering::Greater
        });
        result += match direction {
            Ok(_) => 1,
            Err(_) => 0,
        };
    }
    println!("{:?}", result);
}
