#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

struct Supplies {
    stacks: Vec<Vec<u8>>,
    instructions: Vec<Instruction>, // n from to
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Supplies {
    let (init, instrs) = input.split_once("\n\n").unwrap();
    // from bottom up, ignoring the reference row
    let mut stacks = vec![];
    for line in init.lines().rev().skip(1) {
        let bytes = line.as_bytes();
        // "[a] [b] [c]"
        let count = (bytes.len() + 1) / 4;
        // this should be true first iteration, false all the rest
        if stacks.len() < count {
            for _ in 0..count {
                stacks.push(vec![]);
            }
        }
        for i in 0..count {
            if bytes[i * 4 + 1] != b' ' {
                stacks[i].push(bytes[i * 4 + 1]);
            }
        }
    }
    let mut instructions = vec![];
    for line in instrs.lines() {
        let [_, count, _, src, _, dest] = line.split_ascii_whitespace().collect::<Vec<_>>()[..] else {unreachable!("bad line")};
        instructions.push(Instruction {
            count: count.parse().unwrap(),
            from: src.parse::<usize>().unwrap() - 1,
            to: dest.parse::<usize>().unwrap() - 1,
        });
    }

    Supplies {
        stacks,
        instructions,
    }
}

#[aoc(day5, part1)]
fn iterated_stack_top(input: &Supplies) -> String {
    let mut stacks = input.stacks.to_owned();
    for instr in input.instructions.iter() {
        for _ in 0..instr.count {
            let top = stacks[instr.from].pop().unwrap();
            stacks[instr.to].push(top);
        }
    }
    let tops: Vec<_> = stacks.iter_mut().map(|s| s.pop().unwrap()).collect();
    String::from_utf8(tops).unwrap()
}
#[aoc(day5, part2)]
fn repeated_stack_top(input: &Supplies) -> String {
    let mut stacks = input.stacks.to_owned();
    for instr in input.instructions.iter() {
        let at = stacks[instr.from].len() - instr.count;
        // Kinda slow with all the intermediate allocations
        let top = stacks[instr.from].split_off(at);
        stacks[instr.to].extend(top);
    }
    let tops: Vec<_> = stacks.iter_mut().map(|s| s.pop().unwrap()).collect();
    String::from_utf8(tops).unwrap()
}
