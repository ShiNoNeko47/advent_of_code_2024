fn compute(n: usize, depth: u8) -> usize {
    if depth == 0 {
        1
    } else if n == 0 {
        compute(n + 1, depth - 1)
    } else {
        let number_of_digits = n.checked_ilog10().unwrap_or(0) + 1;
        let pow10 = 10_usize.pow(number_of_digits / 2);
        if number_of_digits % 2 == 0 {
            compute(n / pow10, depth - 1) + compute(n % pow10, depth - 1)
        } else {
            compute(n * 2024, depth - 1)
        }
    }
}
fn main() {
    println!(
        "{}",
        include_str!("./day_11.input")
            .trim_end()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .fold(0, |acc, n| { acc + compute(n, 25) })
    )
}
