use std::array;

use fxhash::FxHashMap;
use num::{Rational64, Zero};

type Name = [u8; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Monkey {
    Const(i64),
    Add(Name, Name),
    Sub(Name, Name),
    Mul(Name, Name),
    Div(Name, Name),
    Human,
}

// returns the new rhs
fn inverse(original: &Monkey, args: Result<Rational64, Rational64>, rhs: Rational64) -> Rational64 {
    match original {
        // commutative
        Monkey::Add(_, _) => match args {
            Ok(x) | Err(x) => rhs - x,
        }
        Monkey::Mul(_, _) => match args {
            Ok(x) | Err(x) => rhs / x
        }
        // noncommutative
        Monkey::Sub(_, _) => match args {
            Ok(x) => x - rhs,
            Err(x) => rhs + x
        }
        Monkey::Div(_, _) => match args {
            Ok(x) => x / rhs,
            Err(x) => rhs * x
        }
        _ => rhs,
    }
}

impl Monkey {
    fn try_eval(&self, monkeys: &FxHashMap<Name, Monkey>) -> Option<i64> {
        match self {
            Monkey::Const(n) => Some(*n),
            Monkey::Add(x, y) => {
                Some(monkeys[x].try_eval(monkeys)? + monkeys[y].try_eval(monkeys)?)
            }
            Monkey::Sub(x, y) => {
                Some(monkeys[x].try_eval(monkeys)? - monkeys[y].try_eval(monkeys)?)
            }
            Monkey::Mul(x, y) => {
                Some(monkeys[x].try_eval(monkeys)? * monkeys[y].try_eval(monkeys)?)
            }
            Monkey::Div(x, y) => {
                Some(monkeys[x].try_eval(monkeys)? / monkeys[y].try_eval(monkeys)?)
            }
            _ => None,
        }
    }

    fn args(&self) -> Option<(Name, Name)> {
        match self {
            Monkey::Add(x, y)
            | Monkey::Sub(x, y)
            | Monkey::Mul(x, y)
            | Monkey::Div(x, y) => Some((*x, *y)),
            _ => None,
        }
    }
}

#[aoc_generator(day21)]
fn generator(input: &str) -> Vec<(Name, Monkey)> {
    input
        .lines()
        .map(|line| {
            let b = line.as_bytes();
            let name = array::from_fn(|i| b[i]);
            let monkey = if b.len() == "abcd: efgh + ijkl".len() {
                let x = array::from_fn(|i| b[i + 6]);
                let y = array::from_fn(|i| b[i + 13]);
                match b[11] {
                    b'+' => Monkey::Add(x, y),
                    b'-' => Monkey::Sub(x, y),
                    b'*' => Monkey::Mul(x, y),
                    b'/' => Monkey::Div(x, y),
                    _ => unreachable!("bad monkey"),
                }
            } else {
                let n = std::str::from_utf8(&b[6..]).unwrap().parse().unwrap();
                Monkey::Const(n)
            };

            (name, monkey)
        })
        .collect()
}

#[aoc(day21, part1)]
fn eval(input: &[(Name, Monkey)]) -> i64 {
    let monkeys: FxHashMap<Name, Monkey> = input.iter().copied().collect();
    let root = b"root";
    monkeys[root].try_eval(&monkeys).unwrap()
}

#[aoc(day21, part2)]
fn solve(input: &[(Name, Monkey)]) -> i64 {
    let mut monkeys: FxHashMap<Name, Monkey> = input.iter().copied().collect();

    // Assumption: "humn" appears exactly once
    let human = b"humn";
    monkeys.insert(*human, Monkey::Human);

    let (x, y) = monkeys[b"root"].args().unwrap();

    let mut lhs = Monkey::Sub(x, y);
    let mut rhs = Rational64::zero();
    while let Some((l, r)) = lhs.args() {
        let l_result = monkeys[&l].try_eval(&monkeys);
        let r_result = monkeys[&r].try_eval(&monkeys);
        let args = match (l_result, r_result) {
            (Some(x), None) => Ok(Rational64::from(x)),
            (None, Some(x)) => Err(Rational64::from(x)),
            _ => unreachable!("funny")
        };
        rhs = inverse(&lhs, args, rhs);
        lhs = if args.is_ok() {
            monkeys[&r]
        } else {
            monkeys[&l]
        };
    }
    rhs.to_integer()
}
