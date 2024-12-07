fn main() {
    let mut input: Vec<Vec<usize>> = include_str!("./day_07.input")
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.replace(":", "")
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect();
    let mut sum = 0;
    let _ = input
        .iter_mut()
        .map(|eq| {
            let target = eq.remove(0);
            (0..2_usize.pow(eq.len() as u32 - 1))
                .into_iter()
                .map_while(|mul| {
                    if eq.iter().enumerate().fold(0, |acc, (idx, n)| {
                        if idx > 0 && 2_u32.pow(idx as u32 - 1) & mul as u32 > 0 {
                            acc * n
                        } else {
                            acc + n
                        }
                    }) == target
                    {
                        sum += target;
                        None
                    } else {
                        Some(mul)
                    }
                })
                .for_each(drop);
        })
        .collect::<Vec<()>>();
    println!("{sum}");
}
