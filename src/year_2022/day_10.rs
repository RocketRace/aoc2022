
#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| if line == "noop" {0}
    else {
        line[5..].parse::<i32>().unwrap()
    }).collect()
}

#[aoc(day10, part1)]
fn selected_signals(input: &[i32]) -> i32 {
    let mut it = input.iter()
        .copied()
        .flat_map(|i| {
            if i == 0 {
                [i32::MAX, 0]
            }
            else {
                [i, 0]
            }
        })
        .filter(|&i| i != i32::MAX)
        .scan(1, |x, i| { let old = *x; *x += i; Some(old) })
        .enumerate()
        .map(|(cycle, x)| (cycle + 2, x))
        .map(|(cycle, x)| cycle as i32 * x);
    
    let initial = it.nth(18).unwrap();
    (0..5).fold(initial, |acc, _| acc + it.nth(39).unwrap())
}

#[aoc(day10, part2)]
fn crt_rasterize(input: &[i32]) -> &'static str {
    let mut grid = [b'.'; 40 * 6];
    input.iter()
        .copied()
        .flat_map(|i| {
            if i == 0 {
                [i32::MAX, 0]
            }
            else {
                [i, 0]
            }
        })
        .filter(|&i| i != i32::MAX)
        .scan(1, |x, i| { let old = *x; *x += i; Some(old) })
        .enumerate()
        .map(|(cycle, x)| (cycle + 1, x))
        .for_each(|(cycle, x)| {
            let offset = cycle as i32 % 40;
            if (offset - x) == (offset - x).signum() {
                grid[cycle] = b'#';
            };
        });
    
    grid.chunks(40).for_each(|chunk| println!("{}", std::str::from_utf8(chunk).unwrap()));
    
    ""
}

