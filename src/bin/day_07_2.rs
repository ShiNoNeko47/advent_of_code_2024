fn main() {
    let mut input: Vec<Vec<u64>> = include_str!("./day_07.input")
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
            let target = eq.remove(0) as u64;
            (0..2_usize.pow(eq.len() as u32 - 1))
                .into_iter()
                .map_while(|mul| {
                    let mut ret = Some(mul);
                    let concat = mul.count_ones();
                    for cat in (0..2_usize.pow(concat)).into_iter() {
                        if eq
                            .iter()
                            .enumerate()
                            .fold((0, 0), |acc, (idx, n)| {
                                if idx > 0 && 2_u64.pow(idx as u32 - 1) & mul as u64 > 0 {
                                    if 2_u64.pow(acc.1 as u32) & cat as u64 > 0 {
                                        (
                                            acc.0
                                                * (10_u64.pow(n.checked_ilog10().unwrap_or(0) + 1))
                                                    as u64
                                                + *n as u64,
                                            acc.1 + 1,
                                        )
                                    } else {
                                        (acc.0 * *n as u64, acc.1 + 1)
                                    }
                                } else {
                                    (acc.0 + *n as u64, acc.1)
                                }
                            })
                            .0 as u64
                            == target
                        {
                            sum += target;
                            ret = None;
                            break;
                        }
                    }
                    ret
                })
                .for_each(drop);
        })
        .collect::<Vec<()>>();
    println!("{sum}");
}
