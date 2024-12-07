fn main() {
    let input: Vec<char> = include_str!("./day_06.input").chars().collect();
    let mut position: i32 = input.iter().position(|c| c == &'^').unwrap() as i32;
    let initial_position = position;
    let width = (input.iter().position(|c| c == &'\n').unwrap() + 1) as i32;
    let directions: [i32; 4] = [-width, 1, width, -1];
    let mut path: Vec<(i32, i32)> = vec![];
    let _ = directions
        .iter()
        .cycle()
        .take_while(|dir| {
            let mut ret: Option<bool> = None;
            while ret.is_none() {
                match input
                    .iter()
                    .nth((position + **dir) as usize)
                    .unwrap_or(&'\n')
                {
                    '\n' => ret = Some(false),
                    '#' => ret = Some(true),
                    _ => {
                        position += **dir;
                        path.append(&mut vec![(position, **dir)]);
                    }
                }
            }
            ret.unwrap()
        })
        .collect::<Vec<&i32>>();
    path.sort_by_key(|p| p.0);
    path.dedup_by_key(|p| p.0);
    let result = path.iter().fold(0, |acc, (pos, dir)| {
        if pos == &initial_position {
            acc
        } else {
            let mut path_2: Vec<(i32, i32)> = vec![];
            let mut r = acc;
            position = *pos - dir;
            let _ = directions
                .iter()
                .cycle()
                .skip_while(|d| d != &dir)
                .take_while(|dir| {
                    let mut ret: Option<bool> = None;
                    while ret.is_none() {
                        match input
                            .iter()
                            .nth((position + **dir) as usize)
                            .unwrap_or(&'\n')
                        {
                            '\n' => ret = Some(false),
                            '#' => ret = Some(true),
                            _ => {
                                if position + **dir == *pos {
                                    ret = Some(true);
                                } else if path_2.contains(&(position, **dir)) {
                                    r += 1;
                                    ret = Some(false);
                                } else {
                                    path_2.append(&mut vec![(position, **dir)]);
                                    position += **dir;
                                }
                            }
                        }
                    }
                    ret.unwrap()
                })
                .collect::<Vec<&i32>>();
            r
        }
    });
    println!("{result}");
}
