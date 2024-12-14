use std::{
    fmt::{self, Debug},
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

struct RationalNumber {
    n: i128,
    d: i128,
}
impl RationalNumber {
    fn new(n: i128) -> Self {
        Self { n, d: 1 }
    }
    fn is_valid(&self) -> bool {
        self.n % self.d == 0 && self.n / self.d <= 100
    }
    fn to_int(&self) -> i128 {
        self.n / self.d
    }
}

impl Clone for RationalNumber {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            d: self.d,
        }
    }
}
impl Copy for RationalNumber {}

impl Add for RationalNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            n: self.n * other.d + self.d * other.n,
            d: self.d * other.d,
        }
    }
}
impl Sub for RationalNumber {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            n: self.n * other.d - self.d * other.n,
            d: self.d * other.d,
        }
    }
}
impl Div for RationalNumber {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        if other.n == 0 {
            panic!("Division by zero");
        }
        Self {
            n: self.n * other.d,
            d: self.d * other.n,
        }
    }
}
impl Mul for RationalNumber {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            n: self.n * other.n,
            d: self.d * other.d,
        }
    }
}

impl FromStr for RationalNumber {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            n: s.parse().unwrap(),
            d: 1,
        })
    }
}
impl Debug for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.d == 1 {
            write!(f, "{}", self.n)
        } else if self.n % self.d == 0 {
            write!(f, "{}", self.n / self.d)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}

fn main() {
    println!(
        "{:?}",
        include_str!("./day_13.input")
            .split("\n\n")
            .map(|machine| {
                machine
                    .split_terminator(&[',', ' ', '\n'])
                    .filter_map(|line| {
                        line.split_terminator(&['+', '='])
                            .skip(1)
                            .next()
                            .map(|n| RationalNumber::from_str(n).unwrap())
                    })
                    .collect::<Vec<RationalNumber>>()
            })
            .fold(0, |acc, mut machine| {
                machine[2] = machine[2] / machine[0];
                machine[4] = machine[4] / machine[0];
                machine[0] = RationalNumber::new(1);
                machine[3] = machine[3] - machine[2] * machine[1];
                machine[5] = machine[5] - machine[4] * machine[1];
                machine[1] = RationalNumber::new(0);
                machine[5] = machine[5] / machine[3];
                machine[3] = RationalNumber::new(1);
                machine[4] = machine[4] - machine[5] * machine[2];
                if machine[5].is_valid() && machine[4].is_valid() {
                    acc + machine[4].to_int() * 3 + machine[5].to_int()
                } else {
                    acc
                }
            })
    );
}
