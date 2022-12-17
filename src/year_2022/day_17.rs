use fxhash::FxHashMap;

#[aoc(day17, part1)]
fn small(input: &[u8]) -> usize {
    falling_simulation(input, 2022)
}
#[aoc(day17, part2)]
fn big(input: &[u8]) -> usize {
    falling_simulation(input, 1_000_000_000_000)
}

// Not rigorous but seems to work enough for most input
const MAX_DEPTH_KEY: usize = 256;

const POLYOMINOES: [[u8; 4]; 5] = [
    [0b1111000, 0b0000000, 0b0000000, 0b0000000], // -
    [0b0100000, 0b1110000, 0b0100000, 0b0000000], // +
    [0b1110000, 0b0010000, 0b0010000, 0b0000000], // J
    [0b1000000, 0b1000000, 0b1000000, 0b1000000], // |
    [0b1100000, 0b1100000, 0b0000000, 0b0000000], // o
];

// width, height
const DIMENSIONS: [(u8, u8); 5] = [
    (4, 1),
    (3, 3),
    (3, 3),
    (1, 4),
    (2, 2)
];

fn overlapping_bits(a: [u8; 4], b: [u8; 4]) -> bool {
    let n = u32::from_ne_bytes(a);
    let m = u32::from_ne_bytes(b);
    n.count_ones() + m.count_ones() != (n | m).count_ones()
}

fn check_collision(board: &[u8], polyomino: usize, x: i8, y: usize) -> bool {
    // shifted too far
    if x < 0 {
        return true;
    }
    if x > 7 - DIMENSIONS[polyomino].0 as i8 {
        return true;
    }
    // fell too far
    if board.len() < y + 4 {
        return true;
    }
    // the compiler might be able to elide overflow checks here
    let shifted_polyomino = POLYOMINOES[polyomino].map(|row| row >> x);

    // the polyomino occupies a particular slice of the board
    let offset = board.len() - y ;
    let slice = &board[offset - 4..offset];
    let view = std::array::from_fn(|i| slice[i]);
    overlapping_bits(view, shifted_polyomino)
}

fn freeze_polyomino(board: &mut [u8], polyomino: usize, x: i8, y: usize) -> usize {
    let shifted_polyomino = POLYOMINOES[polyomino].map(|row| row >> x);
    // nothing past the height needs to be blitted
    let height = DIMENSIONS[polyomino].1 as usize;
    for i in 0..height {
        let offset = board.len() - y;
        board[offset - 4 + i] |= shifted_polyomino[i];
    }
    board.len() - y - 4 + height
}

#[allow(unused)]
fn debug(board: &[u8]) {
    println!("+-------+");
    for i in 0..board.len() {
        let j = board.len() - 1 - i;
        print!("|");
        for k in 0..7 {
            let l = 6 - k;
            if board[j] & (1 << l) != 0 {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        print!("|");
        println!();
    }
    println!("+-------+");
}

type Key = ([u8; MAX_DEPTH_KEY], usize, usize);

fn compute_key(board: &[u8], polyomino: usize, step: usize) -> Key {
    let arr = std::array::from_fn(|i| board[board.len().saturating_sub(i + 1)]);
    (arr, polyomino, step)
}

fn cycle_growth_length(fingerprints: &FxHashMap<Key, (Key, usize)>, start: &Key) -> (usize, usize) {
    let mut i = 0;
    let mut total_growth = 0;
    let mut key = start;
    while let Some((next, growth)) = fingerprints.get(key) {
        total_growth += growth;
        i += 1;
        if next == start {
            break;
        }
        key = next;
    }
    (total_growth, i)
}

fn falling_simulation(input: &[u8], blocks: usize) -> usize {
    // Earlier elements are lower in the board
    let mut board = vec![0; 7];

    let mut fingerprints = FxHashMap::default();

    let mut polyomino = 0;
    let mut step = 0;
    let mut height = 0;
    let mut bonus_height = 0;
    let mut block = 0;
    // no point looking for such short cycles
    let mut fingerprinting = blocks > 10000;
    while block < blocks {
        // spawn a new polyomino
        let mut x = 2;
        let mut y = 0;
        
        // check for cycles
        let initial_state = fingerprinting.then(|| {
            let state = compute_key(&board, polyomino, step);
            if fingerprints.contains_key(&state) {
                let (growth, length) = cycle_growth_length(&fingerprints, &state);
                let remaining_cycles = (blocks - block) / length;
                block += remaining_cycles * length;
                bonus_height = remaining_cycles * growth;
                fingerprinting = false;
            }
            state
        });

        loop {
            let motion = input[step];
            step += 1;
            step %= input.len();
            let delta = match motion {
                b'<' => {
                    -1
                }
                b'>' => {
                    1
                }
                _ => unreachable!("bad byte in input")
            };
            if !check_collision(&board, polyomino, x + delta, y) {
                x += delta;
            }
            if !check_collision(&board, polyomino, x, y + 1) {
                y += 1;
            }
            else {
                let highest_change = freeze_polyomino(&mut board, polyomino, x, y);
                let growth = if height <= highest_change {
                    highest_change - height
                }
                else {
                    0
                };
                height = height.max(highest_change);
                polyomino += 1;
                polyomino %= 5;
                board.resize(height + 3 + 4, 0);
                let new_state = compute_key(&board, polyomino, step);
                initial_state.map(|state| fingerprints.insert(state, (new_state, growth)));
                break;
            }
        }
        block += 1;
    }
    height + bonus_height
}
