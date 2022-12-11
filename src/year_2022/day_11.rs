#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Operation::Add(y) => x + y,
            Operation::Mul(y) => x * y,
            Operation::Square => x * x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Monkey {
    op: Operation,
    test: u64,
    success: usize,
    failure: usize,
}

impl Monkey {
    fn handle(&self, x: u64, relief: u64, factor: u64) -> Item {
        let post_op = self.op.apply(x) / relief % factor;
        if post_op % self.test == 0 {
            Item { value: post_op, index: self.success }
        }
        else {
            Item { value: post_op, index: self.failure }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Item {
    value: u64,
    index: usize,
}

#[aoc_generator(day11)]
fn generator(input: &str) -> (Vec<Monkey>, Vec<Item>) {
    let mut monkeys = vec![];
    let mut items = vec![];
    for chunk in input.split("\n\n") {
        let mut index = None;
        let mut op = None;
        let mut test = None;
        let mut success = None;
        let mut failure = None;
        for line in chunk.lines() {
            if let Some(string) = line.strip_prefix("Monkey ") {
                index = string
                    .strip_suffix(':')
                    .and_then(|s| s.parse::<usize>().ok());
            } else if let Some(string) = line.strip_prefix("  Starting items: ") {
                items.extend(
                    string
                        .split(", ")
                        .filter_map(|s| s.parse::<u64>().ok())
                        .filter_map(|value| index.map(|i| Item { value, index: i })),
                );
            } else if line == "  Operation: new = old * old" {
                op = Some(Operation::Square);
            } else if let Some(target) = line.strip_prefix("  Operation: new = old * ") {
                op = target.parse::<u64>().ok().map(Operation::Mul);
            } else if let Some(target) = line.strip_prefix("  Operation: new = old + ") {
                op = target.parse::<u64>().ok().map(Operation::Add);
            } else if let Some(val) = line.strip_prefix("  Test: divisible by ") {
                test = val.parse::<u64>().ok();
            } else if let Some(target) = line.strip_prefix("    If true: throw to monkey ") {
                success = target.parse::<usize>().ok();
            } else if let Some(target) = line.strip_prefix("    If false: throw to monkey ") {
                failure = target.parse::<usize>().ok();
            }
        }
        monkeys.push(Monkey {
            op: op.unwrap(),
            test: test.unwrap(),
            success: success.unwrap(),
            failure: failure.unwrap(),
        })
    }
    (monkeys, items)
}

fn juggle((monkeys, items): &(Vec<Monkey>, Vec<Item>), steps: usize, relief: u64) -> u64 {
    let mut counters = vec![0; monkeys.len()];
    let factor: u64 = monkeys.iter().map(|m| m.test).fold(1, num::integer::lcm);
    let mut items = (*items).clone();
    for _ in 0..steps {
        for item in items.iter_mut() {
            let mut initial = *item;
            let mut new = monkeys[initial.index].handle(initial.value, relief, factor);
            counters[initial.index] += 1;
            while new.index >= initial.index {
                initial = new;
                new = monkeys[initial.index].handle(initial.value, relief, factor);
                counters[initial.index] += 1;
            }
            *item = new;
        }
    }
    let mut first = 0;
    let mut second = 0;
    for count in counters {
        if count > first {
            second = first;
            first = count;
        }
        else if count > second {
            second = count;
        }
    }
    first * second
}

#[aoc(day11, part1)]
fn annoying(input: &(Vec<Monkey>, Vec<Item>)) -> u64 {
    juggle(input, 20, 3)
}

#[aoc(day11, part2)]
fn stressful(input: &(Vec<Monkey>, Vec<Item>)) -> u64 {
    juggle(input, 10000, 1)
}
