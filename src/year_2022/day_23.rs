use fxhash::{FxHashMap, FxHashSet};

#[aoc_generator(day23)]
fn generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter_map(move |(x, b)| (b == b'#').then_some((x as i32, y as i32)))
        })
        .collect()
}

fn simulate(input: &[(i32, i32)], limit: usize) -> (FxHashSet<(i32, i32)>, usize) {
    let mut elves = FxHashSet::from_iter(input.iter().copied());
    let mut swap_buffer = FxHashSet::default();
    let mut contested_moves = FxHashMap::default();
    let mut step = 0;
    while step < limit {
        let mut still = true;
        'elf_check: for &(x, y) in elves.iter() {
            let neighbors = [
                (x + 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x - 1, y + 1),
                (x - 1, y - 1),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1),
            ];
            if !neighbors.iter().any(|pos| elves.contains(pos)) {
                swap_buffer.insert((x, y));
                continue;
            }
            let zones = [
                [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)], // N
                [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)], // S
                [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)], // W
                [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)], // E
            ];
            let targets = [
                (x, y - 1), // N
                (x, y + 1), // S
                (x - 1, y), // W
                (x + 1, y), // E
            ];
            for i in 0..4 {
                let index = (i + step) % 4;
                let zone = zones[index];
                if zone.iter().any(|pos| elves.contains(pos)) {
                    continue;
                }
                let target = targets[index];
                // The index is unique for each target-source pair
                contested_moves.entry(target).or_insert([None; 4])[index] = Some((x, y));
                continue 'elf_check;
            }
            // If we reach this point, all 4 movement attempts failed
            swap_buffer.insert((x, y));
        }
        for (&target, &attempts) in contested_moves.iter() {
            match attempts {
                [Some(_), None, None, None]
                | [None, Some(_), None, None]
                | [None, None, Some(_), None]
                | [None, None, None, Some(_)] => {
                    swap_buffer.insert(target);
                    // successful move; therefore keep simulating
                    still = false;
                }
                _ => {
                    attempts.into_iter().for_each(|attempt| {
                        if let Some(pos) = attempt {
                            swap_buffer.insert(pos);
                        }
                    });
                }
            }
        }

        std::mem::swap(&mut elves, &mut swap_buffer);
        swap_buffer.clear();
        contested_moves.clear();
        // no more movement, we can halt
        if still {
            break;
        }
        step += 1;
    }
    (elves, step)
}

#[aoc(day23, part1)]
fn aabb(input: &[(i32, i32)]) -> i32 {
    let (elves, _) = simulate(input, 10);
    // largest bounding box containing all elves
    let x_min = elves.iter().map(|&(x, _)| x).min().unwrap_or(0);
    let x_max = elves.iter().map(|&(x, _)| x).max().unwrap_or(0);
    let y_min = elves.iter().map(|&(_, y)| y).min().unwrap_or(0);
    let y_max = elves.iter().map(|&(_, y)| y).max().unwrap_or(0);
    (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as i32
}

#[aoc(day23, part2)]
fn halt(input: &[(i32, i32)]) -> usize {
    let (_, steps) = simulate(input, usize::MAX);
    steps + 1
}
