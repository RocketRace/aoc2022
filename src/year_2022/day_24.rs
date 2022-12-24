use std::collections::{VecDeque, HashSet};

use bit_vec::BitVec;
use num::integer::lcm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    step: usize,
    x: usize,
    y: usize,
}

impl State {
    fn modulo(self, cycle: usize) -> State {
        State { step: self.step % cycle, ..self }
    }
}

#[derive(Debug)]
struct Blizzards {
    width: usize,
    height: usize,
    right: Vec<BitVec>,
    left: Vec<BitVec>,
    up: Vec<BitVec>,
    down: Vec<BitVec>,
}

impl Blizzards {
    fn blizzed(&self, state: State) -> bool {
        state.y != 0 && state.y != self.height + 1 && (
            self.right[state.y - 1][(state.x as isize - state.step as isize).rem_euclid(self.width as isize) as usize]
            || self.left[state.y - 1][((state.x + state.step) % self.width)]
            || self.down[state.x][(state.y as isize - state.step as isize - 1).rem_euclid(self.height as isize) as usize]
            || self.up[state.x][(state.y + state.step - 1) % self.height]
        )
    }

    fn walled(&self, state: State) -> bool {
        (state.x != 0 && state.y == 0) || (state.x != self.width - 1 && state.y == self.height + 1)
    }   
    
    fn check_and_add(&self, queue: &mut VecDeque<State>, state: State) {
        if !self.blizzed(state) && !self.walled(state) {
            queue.push_back(state);
        }
    }
}

#[aoc_generator(day24)]
fn generator(input: &str) -> Blizzards {
    let height = input.lines().count() - 2;
    let width = input.len() / (height + 2) - 2;

    let mut right = vec![BitVec::from_elem(width, false); height];
    let mut left = vec![BitVec::from_elem(width, false); height];
    let mut up = vec![BitVec::from_elem(height, false); width];
    let mut down = vec![BitVec::from_elem(height, false); width];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            match c {
                b'>' => right[y - 1].set(x - 1, true), // x >= 1
                b'<' => left[y - 1].set(x - 1, true),
                b'v' => down[x - 1].set(y - 1, true),
                b'^' => up[x - 1].set(y - 1, true),
                _ => (),
            }
        }
    }

    Blizzards {
        width,
        height,
        left,
        right,
        down,
        up,
    }
}

fn pathfind(blizzards: &Blizzards, initial_state: State, goal: (usize, usize)) -> State {
    let cycle = lcm(blizzards.width, blizzards.height);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(initial_state);

    while let Some(state) = queue.pop_front() {

        if (state.x, state.y) == goal {
            return state;
        }

        if visited.contains(&state.modulo(cycle)) {
            continue;
        }
        visited.insert(state.modulo(cycle));
        
        if state.x < blizzards.width - 1 {
            let new = State {
                step: state.step + 1,
                x: state.x + 1,
                ..state
            };
            blizzards.check_and_add(&mut queue, new)
        }
        if state.x > 0 {
            let new = State {
                step: state.step + 1,
                x: state.x - 1,
                ..state
            };
            blizzards.check_and_add(&mut queue, new)
        }
        if state.y < blizzards.height + 1 {
            let new = State {
                step: state.step + 1,
                y: state.y + 1,
                ..state
            };
            blizzards.check_and_add(&mut queue, new)
        }
        if state.y > 0 {
            let new = State {
                step: state.step + 1,
                y: state.y - 1,
                ..state
            };
            blizzards.check_and_add(&mut queue, new)
        }
        let new = State {
            step: state.step + 1,
            ..state
        };
        if !blizzards.blizzed(new) {
            queue.push_back(new);
        }
    }
    unreachable!("cannot reach ,,,")
}

#[aoc(day24, part1)]
fn one_way(blizzards: &Blizzards) -> usize {
    pathfind(blizzards, State {step: 0, x: 0, y: 0}, (blizzards.width - 1, blizzards.height + 1)).step
}

#[aoc(day24, part2)]
fn three_way(blizzards: &Blizzards) -> usize {
    let first_step = pathfind(blizzards, State {step: 0, x: 0, y: 0}, (blizzards.width - 1, blizzards.height + 1));
    let second_step = pathfind(blizzards, first_step, (0, 0));
    pathfind(blizzards, second_step, (blizzards.width - 1, blizzards.height + 1)).step
}

