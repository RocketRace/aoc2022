#[aoc_generator(day20)]
fn generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn mix(ints: &[i64], mul: i64, times: usize) -> i64 {
    let length = ints.len();
    let mixes: Vec<_> = ints.iter().map(|mix| mix * mul).collect();
    let mut keys: Vec<_> = (0..length).collect();
    for _ in 0..times {
        for (i, &mix) in mixes.iter().enumerate() {
            let old_pos = keys.iter().position(|&j| j == i).unwrap();
            keys.remove(old_pos);
            let new_pos = (old_pos as i64 + mix).rem_euclid(length as i64 - 1) as usize;
            keys.insert(new_pos, i);
        }
    }
    let zero = mixes.iter().position(|&i| i == 0).unwrap();
    let zero_key = keys.iter().position(|&i| i == zero).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| mixes[keys[(zero_key + i) % length]])
        .sum()
}

#[aoc(day20, part1)]
fn smol_decryption(input: &[i64]) -> i64 {
    mix(input, 1, 1)
}

#[aoc(day20, part2)]
fn big_decryption(input: &[i64]) -> i64 {
    mix(input, 811_589_153, 10)
}
