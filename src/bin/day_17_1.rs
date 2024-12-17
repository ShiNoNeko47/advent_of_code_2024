fn main() {
    let input = include_str!("./day_17.input")
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(": ").unwrap().1);
    let mut registers: Vec<i32> = input
        .clone()
        .take(3)
        .map(|reg| reg.parse().unwrap())
        .collect();
    let program: Vec<u8> = input
        .last()
        .unwrap()
        .split(",")
        .map(|command| command.parse().unwrap_or(0))
        .collect();
    let mut ip = 0;
    while ip < program.len() - 1 {
        let operand = program[ip + 1];
        let combo_operand = if operand <= 3 {
            operand as i32
        } else {
            registers[operand as usize - 4]
        };
        let cmd = program[ip];
        ip += 2;
        match cmd {
            0 => {
                registers[0] = registers[0] / 2_i32.pow(combo_operand as u32);
            }
            6 => {
                registers[1] = registers[0] / 2_i32.pow(combo_operand as u32);
            }
            7 => {
                registers[2] = registers[0] / 2_i32.pow(combo_operand as u32);
            }
            1 => registers[1] = registers[1] ^ operand as i32,
            2 => registers[1] = combo_operand % 8,
            3 => {
                if registers[0] != 0 {
                    ip = operand as usize
                }
            }
            4 => registers[1] = registers[1] ^ registers[2],
            5 => {
                print!("{},", combo_operand % 8)
            }
            _ => {}
        }
    }
}
