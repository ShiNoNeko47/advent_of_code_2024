use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum LogicGate {
    And,
    Or,
    Xor,
}

struct Gate {
    value: Option<u8>,
    inputs: Option<(Box<Gate>, Box<Gate>)>,
    gate: LogicGate,
}

impl Gate {
    fn new(
        name: &str,
        connections: &HashMap<&str, ((&str, &str), LogicGate)>,
        values: &HashMap<&str, u8>,
    ) -> Self {
        if values.contains_key(name) {
            Self {
                value: Some(*values.get(name).unwrap()),
                inputs: None,
                gate: LogicGate::And,
            }
        } else {
            let conn = connections.get(&name).unwrap();
            Self {
                value: None,
                inputs: Some((
                    Box::new(Gate::new(conn.0 .0, connections, values)),
                    Box::new(Gate::new(conn.0 .1, connections, values)),
                )),
                gate: conn.1,
            }
        }
    }
    fn get_value(&mut self) -> u8 {
        if let Some(value) = self.value {
            value
        } else {
            let value = match self.gate {
                LogicGate::And => {
                    self.inputs.as_mut().unwrap().0.get_value()
                        & self.inputs.as_mut().unwrap().1.get_value()
                }
                LogicGate::Or => {
                    self.inputs.as_mut().unwrap().0.get_value()
                        | self.inputs.as_mut().unwrap().1.get_value()
                }
                LogicGate::Xor => {
                    self.inputs.as_mut().unwrap().0.get_value()
                        ^ self.inputs.as_mut().unwrap().1.get_value()
                }
            };
            self.value = Some(value);
            value
        }
    }
}

fn main() {
    let (values, connections) = include_str!("./day_24.input").split_once("\n\n").unwrap();
    let values: HashMap<&str, u8> = values.lines().fold(HashMap::new(), |mut acc, line| {
        let (key, value) = line.split_once(": ").unwrap();
        acc.insert(key, value.parse().unwrap());
        acc
    });
    let connections: HashMap<&str, ((&str, &str), LogicGate)> =
        connections.lines().fold(HashMap::new(), |mut acc, line| {
            let line: Vec<&str> = line
                .split_terminator([' ', '-', '>'])
                .filter(|s| !s.is_empty())
                .collect();
            acc.insert(
                line[3],
                (
                    (line[0], line[2]),
                    match line[1] {
                        "AND" => LogicGate::And,
                        "OR" => LogicGate::Or,
                        "XOR" => LogicGate::Xor,
                        _ => panic!(),
                    },
                ),
            );
            acc
        });
    let mut outputs: Vec<&&str> = connections
        .keys()
        .filter(|key| key.starts_with("z"))
        .collect();
    outputs.sort();
    outputs.reverse();
    println!(
        "{:?}",
        outputs.iter().fold(0, |acc, name| {
            (acc << 1) + Gate::new(name, &connections, &values).get_value() as u64
        })
    );
}
