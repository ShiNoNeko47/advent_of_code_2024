use std::collections::HashMap;

fn main() {
    let mut input: Vec<(usize, usize)> = include_str!("./day_09.input")
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .enumerate()
        .collect();

    if (input.last().unwrap().0) % 2 != 0 {
        input.pop(); // just in case
    };
    let input_iter = input.iter().rev();
    let mut moved: HashMap<usize, usize> = HashMap::new();
    println!(
        "{}",
        input_iter.step_by(2).fold(0, |mut acc, (idx, size)| {
            let mut space_displacement = 0;
            for (i, s) in input.iter() {
                if i == idx || (s - moved.get(i).unwrap_or(&0) >= *size && i % 2 == 1) {
                    for j in 0..*size {
                        acc += (space_displacement + j + moved.get(i).unwrap_or(&0)) * idx / 2;
                    }
                    if i != idx {
                        let m = moved.entry(*i).or_insert(0);
                        *m += size;
                    }
                    break;
                }
                space_displacement += s;
            }
            acc
        })
    );
}
