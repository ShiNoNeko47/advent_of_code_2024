fn main() {
    let (keys, locks): (Vec<[u8; 5]>, Vec<[u8; 5]>) = include_str!("./day_25.input")
        .split("\n\n")
        .fold((vec![], vec![]), |mut acc, schematic| {
            let parsed = schematic
                .lines()
                .skip(1)
                .take(5)
                .fold([0; 5], |mut acc, line| {
                    for (idx, column) in acc.iter_mut().enumerate() {
                        *column += if line.chars().nth(idx).unwrap() == '#' {
                            1
                        } else {
                            0
                        };
                    }
                    acc
                });
            if schematic.starts_with("#####") {
                acc.1.push(parsed);
            } else if schematic.starts_with(".....") {
                acc.0.push(parsed);
            }
            acc
        });
    println!(
        "{}",
        locks.iter().fold(0, |acc, lock| {
            acc + keys
                .iter()
                .filter(|key| {
                    key.iter()
                        .zip(lock.iter())
                        .all(|(key, lock)| key + lock <= 5)
                })
                .count()
        })
    );
}
