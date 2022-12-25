const fn geom(i: u32, base: i64) -> i64 {
    base.pow(i) + (if i != 0 { geom(i - 1, base) } else { 0 })
}

/// maximum safe base-5 power in an i64
const MAX_EXP: u32 = 26;
const BALANCER: i64 = 2 * geom(MAX_EXP, 5);

#[aoc(day25, part1)]
fn entry(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            line.bytes()
                .scan(0, |a, b| {
                    *a *= 5;
                    *a += match b {
                        b'2' => 2,
                        b'1' => 1,
                        b'0' => 0,
                        b'-' => -1,
                        b'=' => -2,
                        _ => unreachable!(),
                    };
                    Some(*a)
                })
                .last()
                .unwrap()
        })
        .sum::<i64>()
        + BALANCER;

    String::from_utf8(
        (0..=MAX_EXP)
            .rev()
            .scan(sum, |sum, exp| {
                let power = 5i64.pow(exp);
                if (*sum).abs() > power / 2 {
                    let digit = *sum / power;
                    *sum -= digit * power;
                    Some(digit - 2)
                } else {
                    Some(-2)
                }
            })
            .map(|n| match n {
                -2 => b'=',
                -1 => b'-',
                0 => b'0',
                1 => b'1',
                2 => b'2',
                _ => unreachable!(),
            })
            .collect(),
    )
    .unwrap()
    .trim_start_matches('0')
    .to_string()
}
