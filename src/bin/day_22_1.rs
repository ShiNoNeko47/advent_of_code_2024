fn main() {
    println!(
        "{:?}",
        include_str!("./day_22.input")
            .lines()
            .map(|line| { line.parse::<usize>().unwrap() })
            .fold(0, |acc, mut secret_number| {
                for _ in 0..2000 {
                    secret_number = ((secret_number << 6) ^ secret_number) & ((1 << 24) - 1);
                    secret_number = ((secret_number >> 5) ^ secret_number) & ((1 << 24) - 1);
                    secret_number = ((secret_number << 11) ^ secret_number) & ((1 << 24) - 1);
                }
                acc + secret_number
            })
    );
}
