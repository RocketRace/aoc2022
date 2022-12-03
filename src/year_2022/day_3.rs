fn priority(c: &u8) -> u64 {
    let lower = c & 0b100000;
    let offset = c & 0b11111;
    if lower != 0 {
        offset as u64
    }
    else {
        offset as u64 + 26
    }
}

fn reduce(bits: &[u64]) -> u64 {
    bits.iter().fold(0, |a, b| a | b)
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<Vec<u64>> {
    input.lines().map(
        |line| line.as_bytes().iter().map(priority).map(|c| 1 << c).collect()
    )
    .collect()
}

#[aoc(day3, part1)]
fn max_within_line(input: &[Vec<u64>]) -> u32 {
    input.iter().map(
        |line| {
            let n = line.len();
            let (first, second) = line.split_at(n / 2);
            let hash_first = reduce(first);
            let hash_second = reduce(second);
            (hash_first & hash_second).trailing_zeros()
        }
    ).sum()
}

#[aoc(day3, part2)]
fn max_within_groups(input: &[Vec<u64>]) -> u32 {
    input.iter().array_chunks::<3>().map(
        |[first, second, third]| {
            let hash_first = reduce(first);
            let hash_second = reduce(second);
            let hash_third = reduce(third);
            (hash_first & hash_second & hash_third).trailing_zeros()
        }
    ).sum()
}

