use regex::Regex;

fn main() {
    let input: String = include_str!("./day_03.input")
        .split("don't()")
        .enumerate()
        .map(|(idx, block)| {
            if idx == 0 {
                block
            } else {
                block.split_once("do()").map(|value| value.1).unwrap_or("")
            }
        })
        .fold(String::new(), |acc, str| format!("{acc}{str}"));
    let result = Regex::new(r"mul\((?<a>[1-9][0-9]{0,2}),(?<b>[1-9][0-9]{0,2})\)")
        .unwrap()
        .captures_iter(input.as_str())
        .fold(0, |acc, m| {
            acc + m.name("a").unwrap().as_str().parse::<i32>().unwrap()
                * m.name("b").unwrap().as_str().parse::<i32>().unwrap()
        });
    println!("{result:?}");
}
