#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .map(|line| line.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
fn entry_part_1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|chunk| chunk.iter().sum())
        .max()
        .unwrap_or(0)
}

#[aoc(day1, part2)]
fn entry_part_2(input: &[Vec<usize>]) -> usize {
    const K: usize = 3;
    let mut sums: Vec<usize> = input.iter().map(|chunk| chunk.iter().sum()).collect();
    sums.select_nth_unstable_by(K, |l, r| r.cmp(l));
    sums.iter().take(K).sum()
}
