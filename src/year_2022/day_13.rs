use core::slice;
use std::cmp::Ordering;

use nom::{
    branch::alt, character::complete::char, combinator::map, multi::separated_list0,
    number::complete::double, sequence::delimited,
};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::List(xs), Packet::List(ys)) => xs.cmp(ys),
            (x, Packet::List(ys)) => slice::from_ref(x).cmp(ys),
            (Packet::List(xs), y) => xs[..].cmp(slice::from_ref(y)),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn wrap(self) -> Self {
        Self::List(vec![self])
    }
}

fn parse_packet(input: &str) -> nom::IResult<&str, Packet> {
    alt((
        map(double, |n| Packet::Int(n as u32)),
        map(
            delimited(
                char('['),
                separated_list0(char(','), parse_packet),
                char(']'),
            ),
            Packet::List,
        ),
    ))(input)
}

#[aoc_generator(day13)]
fn generator(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|line| {
            let (first, second) = line.split_once('\n').unwrap();
            (
                parse_packet(first).unwrap().1,
                parse_packet(second).unwrap().1,
            )
        })
        .collect()
}

#[aoc(day13, part1)]
fn ordered(input: &[(Packet, Packet)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| l.cmp(r).is_lt().then_some(i + 1))
        .sum()
}

#[aoc(day13, part2)]
fn sorted(input: &[(Packet, Packet)]) -> usize {
    let mut flat: Vec<Packet> = input
        .iter()
        .flat_map(|(l, r)| [l.clone(), r.clone()])
        .collect();
    let divider_2 = Packet::Int(2).wrap().wrap();
    let divider_6 = Packet::Int(6).wrap().wrap();
    flat.push(divider_2.clone());
    flat.push(divider_6.clone());
    flat.sort();
    let first = flat.binary_search(&divider_2).unwrap() + 1;
    let second = flat.binary_search(&divider_6).unwrap() + 1;
    first * second
}
