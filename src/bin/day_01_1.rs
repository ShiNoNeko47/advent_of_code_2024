fn main() {
    let mut input: Vec<(i32, u8)> = include_str!("./day_01.input")
        .split("\n")
        .filter(|x| x != &"")
        .map(|x| {
            let item: Vec<i32> = x.split("   ").map(|n| n.parse().unwrap()).collect();
            [(item[0], 0u8), (item[1], 1u8)]
        })
        .flatten()
        .collect::<Vec<(i32, u8)>>();
    input.sort_by_key(|item| (item.1, item.0));
    let mut result = 0;
    for i in 0..input.len() / 2 {
        result += (input[i].0 - input[i + input.len() / 2].0).abs();
    }
    println!("{result}");
}
