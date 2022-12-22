const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_value_or(flat_map: &[Vec<u8>], v: usize, w: usize) -> u8 {
    *flat_map.get(v).and_then(|row| row.get(w)).unwrap_or(&b' ')
}

fn flat_wrap(flat_map: &[Vec<u8>], (v, w, direction): State) -> State {
    let mut v = v;
    let mut w = w;
    let (offset_v, offset_w) = DIRECTIONS[direction];
    while get_value_or(
        flat_map,
        v.wrapping_add_signed(-offset_v),
        w.wrapping_add_signed(-offset_w),
    ) != b' '
    {
        v = v.wrapping_add_signed(-offset_v);
        w = w.wrapping_add_signed(-offset_w);
    }
    (v, w, direction)
}

const CUBE_SIZE: usize = 50;

type State = (usize, usize, usize);

fn cube_wrap(_: &[Vec<u8>], (v, w, direction): State) -> State {
    let (macro_v, q_w, new_dir) = match (v / CUBE_SIZE, w / CUBE_SIZE, direction) {
        // hard-coded transitions, woooo
        (0, 1, 0) => (3, 0, 1),
        (0, 1, 3) => (2, 0, 1),
        (0, 2, 0) => (3, 0, 0),
        (0, 2, 1) => (2, 1, 3),
        (0, 2, 2) => (1, 1, 3),
        (1, 1, 1) => (0, 2, 0),
        (1, 1, 3) => (2, 0, 2),
        (2, 0, 0) => (1, 1, 1),
        (2, 0, 3) => (0, 1, 1),
        (2, 1, 1) => (0, 2, 3),
        (2, 1, 2) => (3, 0, 3),
        (3, 0, 1) => (2, 1, 0),
        (3, 0, 2) => (0, 2, 2),
        (3, 0, 3) => (0, 1, 2),
        _ => unreachable!(),
    };
    let (offset_v, offset_w) = (v % CUBE_SIZE, w % CUBE_SIZE);
    let i = match direction {
        0 => offset_v,
        1 => offset_w,
        2 => CUBE_SIZE - 1 - offset_v,
        3 => CUBE_SIZE - 1 - offset_w,
        _ => unreachable!(),
    };
    let (macro_w, new_w) = [
        (CUBE_SIZE - 1, i),
        (i, 0),
        (0, CUBE_SIZE - 1 - i),
        (CUBE_SIZE - 1 - i, CUBE_SIZE - 1),
    ][new_dir];
    (
        macro_v * CUBE_SIZE + macro_w,
        q_w * CUBE_SIZE + new_w,
        new_dir,
    )
}

fn walk(flat_map: &[Vec<u8>], moves: &[Move], wrapper: fn(&[Vec<u8>], State) -> State) -> usize {
    let (mut y, mut x, mut direction) = (0, 0, 1);
    while flat_map[0][x] != b'.' {
        x += 1;
    }
    for movement in moves {
        match movement {
            Move::Left => direction = (direction + 3) % 4,
            Move::Right => direction = (direction + 1) % 4,
            &Move::Move(amount) => {
                for _ in 0..amount {
                    let (dr, dc) = DIRECTIONS[direction];
                    match flat_map
                        .get(y + dr as usize)
                        .and_then(|row| row.get(x + dc as usize))
                        .unwrap_or(&b' ')
                    {
                        b'.' => (y, x) = (y + dr as usize, x + dc as usize),
                        b'#' => break,
                        b' ' => {
                            let (nr, nc, d) = wrapper(flat_map, (y, x, direction));
                            if flat_map[nr][nc] == b'#' {
                                break;
                            }
                            (y, x, direction) = (nr, nc, d);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    1000 * (y + 1) + 4 * (x + 1) + [3, 0, 1, 2][direction]
}

enum Move {
    Left,
    Right,
    Move(u32),
}

#[aoc_generator(day22)]
fn generator(input: &str) -> (Vec<Vec<u8>>, Vec<Move>) {
    let (sprawled, instructions) = input.split_once("\n\n").unwrap();

    // only unfolded cubes allowed here
    let flat_map: Vec<Vec<u8>> = sprawled
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    // hardcoded transitions

    let moves = instructions
        .split_inclusive(|c| c == 'L' || c == 'R')
        .flat_map(|chunk| {
            let mut moves = vec![];
            if chunk.as_bytes()[chunk.len() - 1] == b'L' {
                moves.push(Move::Move(chunk[..chunk.len() - 1].parse().unwrap()));
                moves.push(Move::Left);
            } else if chunk.as_bytes()[chunk.len() - 1] == b'R' {
                moves.push(Move::Move(chunk[..chunk.len() - 1].parse().unwrap()));
                moves.push(Move::Right);
            } else {
                moves.push(Move::Move(chunk.parse().unwrap()));
            }
            moves
        })
        .collect();

    (flat_map, moves)
}

#[aoc(day22, part1)]
fn toroidal(input: &(Vec<Vec<u8>>, Vec<Move>)) -> usize {
    walk(&input.0, &input.1, flat_wrap)
}

#[aoc(day22, part2)]
fn cube_moment(input: &(Vec<Vec<u8>>, Vec<Move>)) -> usize {
    walk(&input.0, &input.1, cube_wrap)
}
