#[aoc_generator(day6)]
fn generator(input: &[u8]) -> Vec<u64> {
    input.iter().map(|c| 1 << (*c as u64)).collect()
}

fn bithash(slice: &[u64]) -> u64 {
    slice.iter().fold(0, |acc, n| acc ^ n)
}

fn full_hash(input: &[u64], count: usize) -> usize {
    count
        + input
            .windows(count)
            .map(bithash)
            .take_while(|&n| n.count_ones() < count as u32)
            .count()
}

fn rolling_hash(input: &[u64], count: usize) -> usize {
    // Rolling hash using xor
    // Ensures that each bit is set an even number of times
    // during the roll
    let mut hash = bithash(&input[0..count]);
    if hash.count_ones() == count as u32 {
        return count;
    }
    for i in count..input.len() {
        hash ^= input[i - count];
        hash ^= input[i];
        if hash.count_ones() == count as u32 {
            return i + 1;
        }
    }
    unreachable!("No such position found")
}

#[aoc(day6, part1)]
fn with_regular_hash_4(input: &[u64]) -> usize {
    full_hash(input, 4)
}
#[aoc(day6, part2)]
fn with_regular_hash_14(input: &[u64]) -> usize {
    full_hash(input, 14)
}
#[aoc(day6, part1, rolling)]
fn with_rolling_hash_4(input: &[u64]) -> usize {
    rolling_hash(input, 4)
}
#[aoc(day6, part2, rolling)]
fn with_rolling_hash_14(input: &[u64]) -> usize {
    rolling_hash(input, 14)
}
