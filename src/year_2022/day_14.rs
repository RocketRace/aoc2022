#[derive(Debug, Clone, Copy)]
enum Interval {
    Horizontal { x: u32, y: u32, width: u32 },
    Vertical { x: u32, y: u32, height: u32 },
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<Interval> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|xy| {
                    let (x, y) = xy.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<(u32, u32)>>()
                .array_windows::<2>()
                .map(|&[(x, y), (x2, y2)]| {
                    if x == x2 {
                        let (y, y2) = if y2 > y { (y, y2) } else { (y2, y) };
                        Interval::Vertical {
                            x,
                            y,
                            height: y2 - y + 1,
                        }
                    } else {
                        let (x, x2) = if x2 > x { (x, x2) } else { (x2, x) };
                        Interval::Horizontal {
                            x,
                            y,
                            width: x2 - x + 1,
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Falling,
    Left,
    Right,
}

fn blit(input: &[Interval], part2: bool) -> u32 {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 512;
    let mut lowest = 0;
    let mut grid = [[0u8; WIDTH]; HEIGHT];
    for interval in input.iter() {
        match *interval {
            Interval::Horizontal { x, y, width } => {
                for i in 0..width {
                    grid[y as usize][x as usize + i as usize] = 1;
                    lowest = lowest.max(y as usize);
                }
            }
            Interval::Vertical { x, y, height } => {
                for i in 0..height {
                    grid[y as usize + i as usize][x as usize] = 1;
                    lowest = lowest.max(y as usize + height as usize);
                }
            }
        }
    }
    
    if part2 {
        for i in 0..WIDTH {
            grid[lowest + 2][i] = 1;
        }
        lowest += 2;
    }

    let mut x = 500;
    let mut y = 0;
    let mut state = State::Falling;
    let mut count = 0;
    while y <= lowest {
        if grid[y][x] == 0 {
            y += 1;
            state = State::Falling;
        } else {
            if y == 0 {
                break;
            }
            match state {
                State::Falling => {
                    x -= 1;
                    state = State::Left;
                }
                State::Left => {
                    x += 2;
                    state = State::Right;
                }
                State::Right => {
                    x -= 1;
                    y -= 1;
                    grid[y][x] = 1;
                    count += 1;
                    x = 500;
                    y = 0;
                    state = State::Falling;
                }
            }
        }
    }
    count
}

#[aoc(day14, part1)]
fn void(input: &[Interval]) -> u32 {
    blit(input, false)
}

#[aoc(day14, part2)]
fn floor(input: &[Interval]) -> u32 {
    blit(input, true)
}
