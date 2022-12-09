use fxhash::FxHashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
struct Vector2(i32, i32);

impl Vector2 {
    fn zero() -> Self {
        Self(0, 0)
    }
    fn right(scale: i32) -> Self {
        Self(scale, 0)
    }
    fn left(scale: i32) -> Self {
        Self(-scale, 0)
    }
    fn down(scale: i32) -> Self {
        Self(0, scale)
    }
    fn up(scale: i32) -> Self {
        Self(0, -scale)
    }
    fn normalize(self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }
    fn norm(self) -> i32 {
        self.0.abs().max(self.1.abs())
    }
}

impl std::ops::Mul<i32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<Vector2> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let direction = bytes[0];
            let scale = bytes[2..]
                .iter()
                .fold(0, |sum, digit| sum * 10 + digit - b'0') as i32;
            match direction {
                b'R' => Vector2::right(scale),
                b'L' => Vector2::left(scale),
                b'U' => Vector2::up(scale),
                b'D' => Vector2::down(scale),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn regular_tabby_cat(input: &[Vector2]) -> usize {
    multi_tail::<2>(input)
}
#[aoc(day9, part2)]
fn cat_o_nine_tails(input: &[Vector2]) -> usize {
    multi_tail::<10>(input)
}

// I'm using a const parameter because arrays are nice
fn multi_tail<const KNOTS: usize>(input: &[Vector2]) -> usize {
    let last = KNOTS - 1;
    let mut visited = FxHashSet::default();
    visited.insert(Vector2::zero());
    let mut knots = [Vector2::zero(); KNOTS];
    let mut previous_tail = Vector2::zero();
    for delta in input.iter().copied() {
        // Skip the overhead of hashset insertion if the tail hasn't moved
        if previous_tail != knots[last] {
            visited.insert(knots[last]);
            previous_tail = knots[last];
        }

        let mut simulation_step = |resolution| {
            knots[0] += resolution;
            for joint in 0..last {
                let head = knots[joint];
                let tail = &mut knots[joint + 1];
                if (head - *tail).norm() > 1 {
                    let jerk = (head - *tail).normalize();
                    *tail += jerk;
                    // The two knots are now aligned
                    let trail = head - *tail;
                    let step = trail.normalize();
                    
                    if joint == last - 1 {
                        for i in 0..trail.norm() {
                            visited.insert(*tail + step * i);
                        }
                    }
                    *tail += trail - step;
                }
                else {
                    // Any further knots won't move if this one's still
                    break;
                }
            }
        };

        // Finer motion resolution is required to simulate 
        // the universe with multiple knots
        if KNOTS > 2 {
            let step = delta.normalize();
            for _ in 0..delta.norm() {
                simulation_step(step);
            }
        }
        else {
            simulation_step(delta);
        }
    }
    visited.len()
}
