use std::{cmp::Ordering, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    sensor: (i64, i64),
    closest: (i64, i64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn len(&self) -> i64 {
        (self.start - self.end).abs()
    }

    fn unit(x: i64) -> Interval {
        Interval { start: x, end: x }
    }
}

// Addition expands the bounds
impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Interval {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        // all overlapping intervals are equally ordered
        if self.end < other.start {
            Ordering::Less
        } else if other.end < self.start {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// For more efficient algorithmic complexity, this should use some form of
// interval tree for faster insertion.
//
// However, the number of intervals in the input is very small (less than a hundred),
// so a vector ends up being faster.
struct IntervalList {
    /// Must stay strictly increasing
    intervals: Vec<Interval>,
}

impl IntervalList {
    fn new() -> Self {
        Self { intervals: vec![] }
    }

    fn add_interval(&mut self, interval: Interval) {
        match self.intervals.binary_search(&interval) {
            Ok(_) => {
                // An overlapping interval exists; take their union
                // We maintain strict order to ensure there are no overlaps after add()
                let eq_start = self.intervals.partition_point(|&x| x < interval);
                // -1 because the partition_point interval is exclusive
                let eq_end = self.intervals.partition_point(|&x| x <= interval) - 1;
                let merged = self.intervals[eq_start] + self.intervals[eq_end] + interval;
                // Replace the range with this one element
                self.intervals.drain(eq_start + 1..=eq_end);
                self.intervals[eq_start] = merged
            }
            Err(i) => {
                self.intervals.insert(i, interval);
            }
        }
    }

    fn total_span(&self) -> i64 {
        self.intervals.iter().map(Interval::len).sum()
    }

    fn first_gap(&self, interval: Interval) -> Option<i64> {
        let start = self
            .intervals
            .binary_search(&Interval::unit(interval.start));
        let end = self.intervals.binary_search(&Interval::unit(interval.end));
        match (start, end) {
            (Ok(i), Ok(j)) if i == j => None,
            (Ok(i), Ok(j)) if i != j => Some(self.intervals[i].end + 1),
            (_, Err(_)) => Some(interval.end),
            (Err(_), _) => Some(interval.start),
            _ => todo!(),
        }
    }

    fn clear(&mut self) {
        self.intervals.clear();
    }
}

#[aoc_generator(day15)]
fn generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = scan_fmt::scan_fmt!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i64,
                i64,
                i64,
                i64
            )
            .unwrap();
            Entry {
                sensor: (sx, sy),
                closest: (bx, by),
            }
        })
        .collect()
}

#[aoc(day15, part1)]
fn one_row(input: &[Entry]) -> i64 {
    const CHECKED_SLICE: i64 = 2_000_000;
    let mut intervals = IntervalList::new();
    input
        .iter()
        .copied()
        .filter_map(
            |Entry {
                 sensor: (sx, sy),
                 closest: (bx, by),
             }| {
                let radius_manhattan = (sx - bx).abs() + (sy - by).abs();
                let y_offset = (CHECKED_SLICE - sy).abs();
                if y_offset <= radius_manhattan {
                    let width = radius_manhattan - y_offset;
                    Some(Interval {
                        start: sx - width,
                        end: sx + width,
                    })
                } else {
                    None
                }
            },
        )
        .for_each(|interval| {
            intervals.add_interval(interval);
        });

    intervals.total_span()
}

#[aoc(day15, part2)]
fn full_grid(input: &[Entry]) -> i64 {
    const CHECKED_SIZE: i64 = 4_000_000;
    // The approach from part 1 doesn't work due to the lack of an "efficient" ordering over the grid.
    let full_range = Interval {
        start: 0,
        end: CHECKED_SIZE,
    };
    let mut intervals = IntervalList::new();
    let mut i = 0;
    while i < CHECKED_SIZE {
        let it = input.iter().copied().filter_map(
            |Entry {
                 sensor: (sx, sy),
                 closest: (bx, by),
             }| {
                let radius_manhattan = (sx - bx).abs() + (sy - by).abs();
                let y_offset = (i - sy).abs();
                if y_offset <= radius_manhattan {
                    let width = radius_manhattan - y_offset;
                    Some(Interval {
                        start: sx - width,
                        end: sx + width,
                    })
                } else {
                    None
                }
            },
        );
        for interval in it {
            intervals.add_interval(interval);
        }
        if let Some(gap) = intervals.first_gap(full_range) {
            return gap * CHECKED_SIZE + i;
        }
        i += 1;
        intervals.clear();
        continue;
    }
    unreachable!()
}
