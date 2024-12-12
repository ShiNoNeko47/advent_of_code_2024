use std::collections::HashMap;

fn compute(n: usize, depth: u8, memo: &mut HashMap<(u8, usize), usize>) -> usize {
    if depth == 0 {
        1
    } else if n == 0 {
        if let Some(entry) = memo.get(&(depth, n)) {
            *entry
        } else {
            let res = compute(n + 1, depth - 1, memo);
            memo.insert((depth, n), res);
            res
        }
    } else {
        let len = format!("{n}").len();
        if len % 2 == 0 {
            if let Some(entry) = memo.get(&(depth, n)) {
                *entry
            } else {
                let res = compute(n / 10_usize.pow(len as u32 / 2), depth - 1, memo)
                    + compute(n % 10_usize.pow(len as u32 / 2), depth - 1, memo);
                memo.insert((depth, n), res);
                res
            }
        } else {
            if let Some(entry) = memo.get(&(depth, n)) {
                *entry
            } else {
                let res = compute(n * 2024, depth - 1, memo);
                memo.insert((depth, n), res);
                res
            }
        }
    }
}
fn main() {
    let mut memo: HashMap<(u8, usize), usize> = HashMap::new();
    println!(
        "{}",
        include_str!("./day_11.input")
            .trim_end()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .fold(0, |acc, n| { acc + compute(n, 75, &mut memo) })
    )
}
