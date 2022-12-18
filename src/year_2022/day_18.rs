use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

const DELTAS: [(i8, i8, i8); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

// may be adjusted if bigger inputs are given
const BIT_WIDTH: usize = 5;
const MAX_INDEX: u8 = 24;

impl Cube {
    fn index(&self) -> usize {
        self.x as usize | (self.y as usize) << (BIT_WIDTH) | (self.z as usize) << (BIT_WIDTH * 2)
    }

    fn from_index(idx: usize) -> Cube {
        let mask = (1 << BIT_WIDTH) - 1;
        Cube {
            x: (idx & mask) as u8,
            y: (idx >> BIT_WIDTH & mask) as u8,
            z: (idx >> (2 * BIT_WIDTH) & mask) as u8,
        }
    }

    fn add(&self, (dx, dy, dz): (i8, i8, i8)) -> Cube {
        Cube {
            x: self.x.saturating_add_signed(dx).min(MAX_INDEX),
            y: self.y.saturating_add_signed(dy).min(MAX_INDEX),
            z: self.z.saturating_add_signed(dz).min(MAX_INDEX),
        }
    }
}

#[aoc_generator(day18)]
fn generator(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|n| n.parse().unwrap());
            Cube {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
                z: coords.next().unwrap(),
            }
            .add((1, 1, 1)) // shift by 1 so any border checks around zeros don't underflow
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Point {
    Visitable,
    Blocking,
    Visited,
}

impl Point {
    const fn air(part2: bool) -> Point {
        if part2 {
            Point::Visitable
        }
        else {
            Point::Blocking
        }
    }
    const fn lava(part2: bool) -> Point {
        if part2 {
            Point::Blocking
        }
        else {
            Point::Visitable
        }
    }
}

// impl Debug for Point {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Point::Empty => f.write_char(' '),
//             Point::Lava => f.write_char('!'),
//             Point::VisitedLava => f.write_char('~'),
//         }
//     }
// }

#[aoc(day18, part1)]
fn total_surface_area(cubes: &[Cube]) -> u32 {
    flood_fill(cubes, false)
}
#[aoc(day18, part2)]
fn external_surface_area(cubes: &[Cube]) -> u32 {
    flood_fill(cubes, true)
}

fn flood_fill(cubes: &[Cube], part2: bool) -> u32 {
    let mut space = vec![Point::air(part2); 1 << (BIT_WIDTH * 3)];
    for cube in cubes {
        space[cube.index()] = Point::lava(part2);
    }

    let mut surface_area = 0;
    let mut stack = vec![];
    if part2{
        stack.push(0);
        flood_fill_loop(&mut space, &mut stack, &mut surface_area)
    } else {
        while let Some(idx) = (0..1 << (BIT_WIDTH * 3)).find(|&i| space[i] == Point::Visitable) {
            stack.push(idx);
            flood_fill_loop(&mut space, &mut stack, &mut surface_area);
        }
    }
    surface_area
}

fn flood_fill_loop(space: &mut [Point], stack: &mut Vec<usize>, surface_area: &mut u32) {
    while let Some(idx) = stack.pop() {
        if space[idx] == Point::Visited {
            continue;
        }
        space[idx] = Point::Visited;
        for delta in DELTAS {
            let new = Cube::from_index(idx).add(delta).index();
            match space[new] {
                Point::Blocking => *surface_area += 1,
                Point::Visitable => stack.push(new),
                Point::Visited => (),
            }
        }
    }
}