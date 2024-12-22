use std::collections::HashMap;

fn main() {
    let mut sequences: HashMap<(i8, i8, i8, i8), isize> = HashMap::new();
    include_str!("./day_22.input")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .for_each(|mut secret_number| {
            let mut current_sequence: Vec<i8> = vec![];
            let mut current_sequences: HashMap<(i8, i8, i8, i8), isize> = HashMap::new();
            for _ in 0..2000 {
                let prev = secret_number % 10;
                secret_number = ((secret_number << 6) ^ secret_number) & ((1 << 24) - 1);
                secret_number = ((secret_number >> 5) ^ secret_number) & ((1 << 24) - 1);
                secret_number = ((secret_number << 11) ^ secret_number) & ((1 << 24) - 1);
                if current_sequence.len() < 4 {
                    current_sequence.push((secret_number % 10 - prev) as i8);
                } else {
                    current_sequence.remove(0);
                    current_sequence.push((secret_number % 10 - prev) as i8);
                }
                if current_sequence.len() >= 4 {
                    current_sequences
                        .entry((
                            current_sequence[0],
                            current_sequence[1],
                            current_sequence[2],
                            current_sequence[3],
                        ))
                        .or_insert(secret_number % 10);
                }
            }
            current_sequences.iter().for_each(|(key, value)| {
                sequences.insert(*key, *sequences.get(key).unwrap_or(&0) + *value);
            });
        });
    println!("{:?}", sequences.values().max());
}
