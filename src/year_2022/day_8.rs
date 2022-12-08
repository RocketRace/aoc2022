struct Grid {
    elements: Vec<u8>,
    width: usize,
    height: usize,
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Grid {
    let mut elements = vec![];
    let mut height = 0;
    for line in input.lines() {
        height += 1;
        elements.extend(line.as_bytes().iter().map(|n| n - b'0'));
    }
    let width = elements.len() / height;
    Grid {
        elements,
        width,
        height,
    }
}

enum Axis {
    Horizontal,
    Vertical,
}

fn set_visible_axis(
    visible: &mut [u32],
    Grid {
        elements,
        width,
        height,
    }: &Grid,
    axis: Axis,
) {
    let width = *width;
    let height = *height;

    let (major, minor) = match axis {
        Axis::Vertical => (width, height),
        Axis::Horizontal => (height, width),
    };

    for i in 0..major {
        let mut top = 0u8;
        let mut top_j = 0;
        for j in 0..minor {
            let index = match axis {
                Axis::Horizontal => i * minor + j,
                Axis::Vertical => j * minor + i,
            };
            if elements[index] > top {
                top = elements[index];
                top_j = j;
                visible[index] = 1;
                if elements[index] == b'9' {
                    break; // no higher digits
                }
            }
        }
        let mut top_rev = 0;
        for j in 0..minor - 1 - top_j {
            let index = match axis {
                Axis::Horizontal => i * minor + minor - 1 - j,
                Axis::Vertical => (major - 1 - j) * minor + i,
            };
            if elements[index] > top_rev {
                top_rev = elements[index];
                visible[index] = 1;
                if elements[index] == top {
                    break; // no higher digits
                }
            }
        }
    }
}

#[allow(unused)]
fn debug_results(visible: &[u32], width: usize) {
    // Draws a pretty grid
    for row in visible.chunks(width) {
        for c in row {
            let s = match *c {
                0 => ' ',
                1 => '.',
                2..100 => 'o',
                100..1000 => 'O',
                1000..10000 => '@',
                10000..100000 => '#',
                _ => '!',
            };
            eprint!("{s}");
        }
        eprintln!()
    }
}

#[aoc(day8, part1)]
fn count_visible(grid: &Grid) -> u32 {
    let mut visible = vec![0_u32; grid.elements.len()];
    set_visible_axis(&mut visible, grid, Axis::Horizontal);
    set_visible_axis(&mut visible, grid, Axis::Vertical);
    visible.iter().sum()
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}
use Direction::*;

fn set_scores_direction(scores: &mut [u32], grid: &Grid, direction: Direction) {
    let (major, minor) = match direction {
        Right | Left => (grid.height, grid.width),
        Up | Down => (grid.width, grid.height),
    };

    for i in 1..major - 1 {
        let mut counts = [1; 10];
        for j in 1..minor - 1 {
            let index = match direction {
                Right => i * minor + j,
                Left => i * minor + minor - 1 - j,
                Down => j * minor + i,
                Up => (minor - 1 - j) * minor + i,
            };
            scores[index] *= counts[grid.elements[index] as usize];
            for k in 0..=grid.elements[index] {
                counts[k as usize] = 1;
            }
            for k in grid.elements[index] + 1..10 {
                counts[k as usize] += 1;
            }
        }
    }
}

#[aoc(day8, part2)]
fn best_spot(grid: &Grid) -> u32 {
    let mut scores = vec![1_u32; grid.elements.len()];
    set_scores_direction(&mut scores, grid, Right);
    set_scores_direction(&mut scores, grid, Left);
    set_scores_direction(&mut scores, grid, Down);
    set_scores_direction(&mut scores, grid, Up);

    scores.into_iter().max().unwrap_or(0)
}
