use std::cmp::Ordering;

use nom::{
    branch::alt,
    combinator::map,
    multi::separated_list0,
    number::complete::double,
    sequence::{preceded, terminated},
};

use nom::character::complete::char;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

fn zip_cmp(xs: &[Packet], ys: &[Packet]) -> Ordering {
    // tiebreaker
    xs.iter()
        .zip(ys.iter())
        .fold(Ordering::Equal, |ord, (x, y)| ord.then_with(|| x.cmp(y)))
        .then_with(|| xs.len().cmp(&ys.len()))
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::Int(x), Packet::List(ys)) => zip_cmp(&[Packet::Int(*x)], ys),
            (Packet::List(xs), Packet::Int(y)) => zip_cmp(xs, &[Packet::Int(*y)]),
            (Packet::List(xs), Packet::List(ys)) => zip_cmp(xs, ys),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> nom::IResult<&str, Packet> {
    alt((
        map(double, |n| Packet::Int(n as u32)),
        map(
            preceded(
                char('['),
                terminated(separated_list0(char(','), parse_packet), char(']')),
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
    let divider = |n| Packet::List(vec![Packet::List(vec![Packet::Int(n)])]);
    flat.push(divider(2));
    flat.push(divider(6));
    flat.sort();
    let first = flat.binary_search(&divider(2)).unwrap() + 1;
    let second = flat.binary_search(&divider(6)).unwrap() + 1;
    first * second
}
