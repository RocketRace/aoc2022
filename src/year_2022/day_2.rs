#[aoc(day2, part1)]
fn action_score(input: &[u8]) -> u32 {
    input.array_chunks::<4>().map(
        |&[opponent, _, you, _]| {
            let opponent = opponent - b'A';
            let you = you - b'X';
            let choice_score = you + 1;
            let win_score = (you + 3 - opponent + 1) % 3 * 3;
            (choice_score + win_score) as u32
        }
    )
    .sum()
}
#[aoc(day2, part2)]
fn outcome_score(input: &[u8]) -> u32 {
    input.array_chunks::<4>().map(
        |&[opponent, _, outcome, _]| {
            let opponent = opponent - b'A';
            let outcome = outcome - b'X';
            let win_score = outcome * 3;
            let you = (opponent + outcome + 2) % 3;
            let choice_score = you + 1;
            (choice_score + win_score) as u32
        }
    )
    .sum()
}

