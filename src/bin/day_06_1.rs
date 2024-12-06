fn main() {
    let mut input: Vec<char> = include_str!("./day_06.input").chars().collect();
    let mut position: i32 = input.iter().position(|c| c == &'^').unwrap() as i32;
    let width = (input.iter().position(|c| c == &'\n').unwrap() + 1) as i32;
    let directions: [i32; 4] = [-width, 1, width, -1];
    let _ = directions
        .iter()
        .cycle()
        .take_while(|dir| {
            let mut ret: Option<bool> = None;
            while ret.is_none() {
                input[position as usize] = 'X';
                match input
                    .iter()
                    .nth((position + **dir) as usize)
                    .unwrap_or(&'\n')
                {
                    '\n' => ret = Some(false),
                    '#' => ret = Some(true),
                    _ => {
                        position += **dir;
                    }
                }
            }
            ret.unwrap()
        })
        .collect::<Vec<&i32>>();
    println!("{}", input.iter().filter(|c| **c == 'X').count());
}
