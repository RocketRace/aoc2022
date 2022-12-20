use std::{
    collections::{BinaryHeap, HashMap},
    ops::{Add, Div, Index, Mul, Sub},
    str::FromStr,
};

use num::{
    traits::{Bounded, SaturatingSub},
    Zero,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    id: u16,
    costs_per_material: MatVec<MatVec<u16>>,
    costs_per_robot: MatVec<MatVec<u16>>,
}

impl Blueprint {
    fn max_costs(&self) -> MatVec<u16> {
        self.costs_per_material
            .map(|v| v.into_iter().max().unwrap_or(u16::MAX))
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Here's a messed up long statement
        let (
            id,
            ore_cost,
            clay_cost,
            obsidian_cost_0,
            obsidian_cost_1,
            geode_cost_0,
            geode_cost_1
        ) = scan_fmt::scan_fmt!(s,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            u16, u16, u16, u16, u16, u16, u16
        ).unwrap();

        Ok(Blueprint {
            id,
            costs_per_material: MatVec::new(
                MatVec::new(ore_cost, clay_cost, obsidian_cost_0, geode_cost_0),
                MatVec::new(0, 0, obsidian_cost_1, 0),
                MatVec::new(0, 0, 0, geode_cost_1),
                MatVec::new(0, 0, 0, 0),
            ),
            // more convenient to implement than transposing on demand
            costs_per_robot: MatVec::new(
                MatVec::new(ore_cost, 0, 0, 0),
                MatVec::new(clay_cost, 0, 0, 0),
                MatVec::new(obsidian_cost_0, obsidian_cost_1, 0, 0),
                MatVec::new(geode_cost_0, 0, geode_cost_1, 0),
            ),
        })
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Vec<Blueprint> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    materials: MatVec<u16>,
    robots: MatVec<u16>,
    minutes: u16,
    // next_intended_build: Intention
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
use Material::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct MatVec<T: Copy>([T; 4]);

impl<T: Copy> MatVec<T> {
    fn new(ore: T, clay: T, obsidian: T, geode: T) -> MatVec<T> {
        MatVec([ore, clay, obsidian, geode])
    }
    fn map<U: Copy, F: Fn(T) -> U>(self, f: F) -> MatVec<U> {
        MatVec(self.0.map(f))
    }
    fn map2<U: Copy, V: Copy, F: Fn(T, U) -> V>(self, other: MatVec<U>, f: F) -> MatVec<V> {
        MatVec::new(
            f(self[Material::Ore], other[Material::Ore]),
            f(self[Material::Clay], other[Material::Clay]),
            f(self[Material::Obsidian], other[Material::Obsidian]),
            f(self[Material::Geode], other[Material::Geode]),
        )
    }
}

impl<T: Copy + Add<Output = T>> Add for MatVec<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let MatVec([a, b, c, d]) = self;
        let MatVec([e, f, g, h]) = rhs;
        MatVec::new(a + e, b + f, c + g, d + h)
    }
}

impl<T: Copy + Add<u16, Output = T>> Add<Material> for MatVec<T> {
    type Output = Self;

    fn add(self, rhs: Material) -> Self::Output {
        let mut inner = self.0;
        inner[rhs as usize] = inner[rhs as usize] + 1;
        MatVec(inner)
    }
}

impl<T: Copy + SaturatingSub<Output = T>> Sub for MatVec<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let MatVec([a, b, c, d]) = self;
        let MatVec([e, f, g, h]) = rhs;
        MatVec::new(
            a.saturating_sub(&e),
            b.saturating_sub(&f),
            c.saturating_sub(&g),
            d.saturating_sub(&h),
        )
    }
}

impl<T: Copy + Mul<u16, Output = T>> Mul<u16> for MatVec<T> {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        let MatVec([a, b, c, d]) = self;
        MatVec::new(a * rhs, b * rhs, c * rhs, d * rhs)
    }
}
impl<T: Copy + Div<Output = T> + Bounded + Zero + Add<Output = T> + Sub<u16, Output = T>> Div
    for MatVec<T>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let MatVec([a, b, c, d]) = self;
        let MatVec([e, f, g, h]) = rhs;
        let div = |x: T, y: T| {
            if y.is_zero() {
                if x.is_zero() {
                    T::zero()
                } else {
                    T::max_value()
                }
            } else {
                (x + y - 1) / y
            }
        };
        MatVec::new(div(a, e), div(b, f), div(c, g), div(d, h))
    }
}

impl<T: Copy> IntoIterator for MatVec<T> {
    type Item = T;

    type IntoIter = <[T; 4] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Copy> Index<Material> for MatVec<T> {
    type Output = T;

    fn index(&self, index: Material) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl State {
    fn process_robots(self, time: u16) -> State {
        State {
            materials: self.materials + self.robots * time,
            minutes: self.minutes - time,
            ..self
        }
    }
    fn build(self, material: Material, blueprint: &Blueprint) -> State {
        State {
            materials: self.materials - blueprint.costs_per_robot[material],
            robots: self.robots + material,
            minutes: self.minutes,
        }
    }
    fn leftover_yields(&self, blueprint: &Blueprint) -> MatVec<bool> {
        let gains = self.materials + self.robots * self.minutes;
        let max_usages = blueprint.max_costs() * self.minutes;
        gains.map2(max_usages, |a, b| a > b)
    }

    fn enough_robots(&self, blueprint: &Blueprint) -> MatVec<bool> {
        self.robots.map2(blueprint.max_costs(), |a, b| a >= b)
    }

    fn minutes_until_build(&self, blueprint: &Blueprint) -> MatVec<u16> {
        blueprint.costs_per_robot.map(|robot| {
            ((robot - self.materials) / self.robots)
                .into_iter()
                .max()
                .unwrap_or(u16::MAX)
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.minutes
            .cmp(&other.minutes)
            .then_with(|| self.materials[Geode].cmp(&other.materials[Geode]))
            .then_with(|| (self.robots, self.materials).cmp(&(other.robots, other.materials)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn sum_for(blueprints: &[Blueprint], minutes: u16) -> Vec<(u16, u16)> {
    let mut counts = vec![];
    let mut visited = 0;
    for blueprint in blueprints {
        let mut heap = BinaryHeap::new();

        let initial_state = State {
            materials: MatVec::new(0, 0, 0, 0),
            robots: MatVec::new(1, 0, 0, 0),
            minutes,
        };
        heap.push(initial_state);

        let mut scores = HashMap::new();
        let mut max_geodes = vec![0; minutes as usize + 1];

        while let Some(state) = heap.pop() {
            // no cycles exist, but it's likely for two branches to converge
            if scores.contains_key(&state) {
                continue;
            }
            scores.insert(state, state.materials[Geode]);
            max_geodes[state.minutes as usize] =
                max_geodes[state.minutes as usize].max(state.materials[Geode]);

            // no further options
            if state.minutes == 0 {
                continue;
            }
            if max_geodes[state.minutes as usize] >= state.materials[Geode] + 2 {
                continue;
            }
            visited += 1;

            // each state has choices
            // these are then pruned depending on time remaining, demand for robots,
            // and a heuristic upper bound for geodes produced
            let waits = state
                // would buying the robot be useless since the extra material can't be used?
                .leftover_yields(blueprint)
                // would buying the robot be useless since you can already buy one of every robot per minute?
                .map2(state.enough_robots(blueprint), |a, b| !a && !b)
                // ... but there's never enough geode machines
                .map2(MatVec::new(false, false, false, true), |a, b| a || b)
                .map2(state.minutes_until_build(blueprint), |a, b| a.then_some(b))
                // would buying the robot be useless since the time runs out before you can reach it?
                // this includes cases where you will never wait enough time (production = 0)
                .map(|min| min.filter(|&m| m < state.minutes));

            // choice: wait until you can build a certain robot, then build it
            [Ore, Clay, Obsidian, Geode].iter().for_each(|&mat| {
                if let Some(wait) = waits[mat] {
                    heap.push(state.process_robots(wait + 1).build(mat, blueprint));
                }
            });
            // or simply do nothing until the end, lol
            heap.push(state.process_robots(state.minutes));
        }
        counts.push((blueprint.id, max_geodes.into_iter().max().unwrap_or(0)));
    }

    println!("visited {visited} states");
    counts
}

#[aoc(day19, part1)]
fn quick_but_wide(blueprints: &[Blueprint]) -> u16 {
    sum_for(blueprints, 24).iter().map(|(a, b)| a * b).sum()
}

#[aoc(day19, part2)]
fn slow_but_narrow(blueprints: &[Blueprint]) -> u16 {
    sum_for(&blueprints[0..3], 32)
        .iter()
        .map(|(_, b)| b)
        .product()
}
