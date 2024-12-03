use regex::Regex;

fn main() {
    let input = include_str!("./day_03.input");
    let result = Regex::new(r"mul\((?<a>[1-9][0-9]{0,2}),(?<b>[1-9][0-9]{0,2})\)")
        .unwrap()
        .captures_iter(input)
        .fold(0, |acc, m| {
            acc + m.name("a").unwrap().as_str().parse::<i32>().unwrap()
                * m.name("b").unwrap().as_str().parse::<i32>().unwrap()
        });
    println!("{result:?}");
}
