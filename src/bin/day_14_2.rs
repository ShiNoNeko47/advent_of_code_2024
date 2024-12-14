fn main() {
    const DIMENSIONS: [i32; 2] = [101, 103];
    let mut second = 1;
    let input = include_str!("./day_14.input")
        .trim_end()
        .split("\n")
        .map(|robot| {
            robot
                .split_terminator(&['=', ',', ' '])
                .filter_map(|n| {
                    if n.starts_with("-") {
                        n.split_once("-")
                            .unwrap()
                            .1
                            .parse::<i32>()
                            .ok()
                            .map(|num| -num)
                    } else {
                        n.parse().ok()
                    }
                })
                .collect::<Vec<i32>>()
        });
    loop {
        if input
            .clone()
            .fold(["."; 103 * 101], |mut acc, robot| {
                let mut location = (
                    (robot[0] + second * robot[2]) % DIMENSIONS[0],
                    (robot[1] + second * robot[3]) % DIMENSIONS[1],
                );
                if location.0 < 0 {
                    location.0 += DIMENSIONS[0];
                }
                if location.1 < 0 {
                    location.1 += DIMENSIONS[1];
                }
                let idx: usize = location.1 as usize * DIMENSIONS[0] as usize + location.0 as usize;
                acc[idx] = "#";
                acc
            })
            .concat()
            .contains("##########")
        {
            println!("{}", second);
            break;
        }
        second += 1;
    }
}
