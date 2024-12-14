fn main() {
    const DIMENSIONS: [i32; 2] = [101, 103];
    // const DIMENSIONS: [i32; 2] = [11, 7];
    println!(
        "{:?}",
        include_str!("./day_14.input")
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
            })
            .fold([0, 0, 0, 0], |mut acc, robot| {
                let mut location = (
                    (robot[0] + 100 * robot[2]) % DIMENSIONS[0],
                    (robot[1] + 100 * robot[3]) % DIMENSIONS[1],
                );
                if location.0 < 0 {
                    location.0 += DIMENSIONS[0];
                }
                if location.1 < 0 {
                    location.1 += DIMENSIONS[1];
                }
                if location.0 > DIMENSIONS[0] / 2 {
                    if location.1 > DIMENSIONS[1] / 2 {
                        acc[0] += 1;
                    } else if location.1 < DIMENSIONS[1] / 2 {
                        acc[1] += 1;
                    }
                } else if location.0 < DIMENSIONS[0] / 2 {
                    if location.1 > DIMENSIONS[1] / 2 {
                        acc[2] += 1;
                    } else if location.1 < DIMENSIONS[1] / 2 {
                        acc[3] += 1;
                    }
                }
                acc
            })
            .iter()
            .fold(1, |acc, n| acc * n)
    );
}
