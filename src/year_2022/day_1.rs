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
    // No quickselect used here due to fixed and small k
    let iter = input.iter().map(|chunk| chunk.iter().sum());

    let mut top = 0;
    let mut mid = 0;
    let mut bot = 0;

    for sum in iter {
        if sum > top {
            bot = mid;
            mid = top;
            top = sum;
        } else if sum > mid {
            bot = mid;
            mid = sum;
        } else if sum > bot {
            bot = sum;
        }
    }
    top + mid + bot
}
