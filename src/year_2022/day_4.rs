struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_left(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.start
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let left = Range::from_str(l);
            let right = Range::from_str(r);
            (left, right)
        })
        .collect()
}

#[aoc(day4, part1)]
fn containment(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(l, r)| l.contains(r) || r.contains(l))
        .count()
}

#[aoc(day4, part2)]
fn overlap(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(l, r)| l.overlaps_left(r) || r.overlaps_left(l))
        .count()
}
