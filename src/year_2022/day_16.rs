// Today has been needlessly difficult. I unfortunately don't have the time for a better implementation

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

type Name = (char, char);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: Name,
    rate: u32,
    connections: Vec<Name>,
}

#[aoc_generator(day16)]
fn generator(input: &str) -> Vec<Valve> {
    input
        .lines()
        .map(|line| {
            let (first, conn) = line.split_once(';').unwrap();
            let (head, flow) = first.split_once(" has flow rate=").unwrap();
            let name = (head.as_bytes()[6] as char, head.as_bytes()[7] as char);
            let rate = flow.parse().unwrap();
            let connections = conn
                .split(", ")
                .map(|n| {
                    if n.len() == 2 {
                        n.as_bytes()
                    }
                    else {
                        let b = n.as_bytes();
                        &b[b.len() - 2..]
                    }
                })
                .filter_map(|n| (n.len() == 2).then_some((n[0] as char, n[1] as char)))
                .collect();
            Valve {
                name,
                rate,
                connections,
            }
        })
        .collect()
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Hole {
    name: Name,
    minutes: u8,
    loss: u32,
    vented: u64,
}

impl Hole {
    fn key(&self) -> (u8, u64, Name) {
        (self.minutes, self.vented, self.name)
    }
}

impl Ord for Hole {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .loss
            .cmp(&self.loss)
            .then_with(|| other.name.cmp(&self.name))
            .then_with(|| other.vented.cmp(&self.vented))
            .then_with(|| other.minutes.cmp(&self.minutes))
        // not consistent with partialeq
    }
}

impl PartialOrd for Hole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DoubleHole {
    me: Hole,
    your_name: Name,
}

impl DoubleHole {
    fn key(&self) -> ((u8, u64, Name), Name) {
        (self.me.key(), self.your_name)
    }
}

impl Ord for DoubleHole {
    fn cmp(&self, other: &Self) -> Ordering {
        self.me.cmp(&other.me).then_with(|| self.your_name.cmp(&other.your_name))
    }
}

impl PartialOrd for DoubleHole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_loss(vented: u64, losers: usize, sus: &[Valve]) -> u32 {
    (0..losers)
        .filter_map(|i| (vented & (1 << i) == 0).then_some(sus[i].rate))
        .sum()
}

#[aoc(day16, part1)]
fn solo_vent(input: &[Valve]) -> u32 {
    let minutes_left = 30;

    let suspicious: Vec<_> = input
        .iter()
        .filter(|valve| valve.rate > 0)
        .cloned()
        .collect();


    let total_possible_loss: u32 = suspicious.iter().map(|valve| valve.rate * minutes_left as u32).sum();
    let losers = suspicious.len();

    let mut neighbors = HashMap::new();
    for valve in input.iter() {
        neighbors.insert(valve.name, valve.connections.clone());
    }

    let start = ('A', 'A');
    // we want the shortest path that visits all relievers in the order that minimizes loss
    // loss = total flow not being released per tick
    let mut heap = BinaryHeap::new();
    let first_hole = Hole {
        name: start,
        minutes: minutes_left,
        loss: 0,
        vented: 0,
    };
    heap.push(first_hole);

    let mut min_loss_for = HashMap::new();
    min_loss_for.insert(first_hole.key(), 0u32);
    let mut min_loss = u32::MAX;


    while let Some(hole) = heap.pop() {
        if hole.minutes == 0 {
            min_loss = min_loss.min(hole.loss);
            break;
        }
        if hole.vented == (1 << losers) - 1 {
            min_loss = min_loss.min(hole.loss);
            break;
        }

        if hole.loss > min_loss_for.get(&hole.key()).copied().unwrap_or(u32::MAX) {
            continue;
        }

        if let Some(index) = (0..losers).find(|&i| suspicious[i].name == hole.name) {
            if hole.vented & (1 << index) == 0 {
                let vent_hole = Hole {
                    name: hole.name,
                    minutes: hole.minutes - 1,
                    loss: hole.loss + compute_loss(hole.vented, losers, &suspicious),
                    vented: hole.vented | (1 << index)
                };
    
                if vent_hole.loss < min_loss_for.get(&vent_hole.key()).copied().unwrap_or(u32::MAX) {
                    heap.push(vent_hole);
                    min_loss_for.insert(vent_hole.key(), vent_hole.loss);
                }
            }
        }

        for &neighbor in neighbors[&hole.name].iter() {
            let next_hole = Hole {
                name: neighbor,
                minutes: hole.minutes - 1,
                loss: hole.loss + compute_loss(hole.vented, losers,&suspicious),
                vented: hole.vented,
            };

            if next_hole.loss < min_loss_for.get(&next_hole.key()).copied().unwrap_or(u32::MAX) {
                heap.push(next_hole);
                min_loss_for.insert(next_hole.key(), next_hole.loss);
            }
        }
    }
    total_possible_loss - min_loss
}

#[aoc(day16, part2)]
fn pair_vent(input: &[Valve]) -> u32 {
    let minutes_left = 26;

    let suspicious: Vec<_> = input
        .iter()
        .filter(|valve| valve.rate > 0)
        .cloned()
        .collect();


    let total_possible_loss: u32 = suspicious.iter().map(|valve| valve.rate * minutes_left as u32).sum();
    let losers = suspicious.len();

    let mut neighbors = HashMap::new();
    for valve in input.iter() {
        neighbors.insert(valve.name, valve.connections.clone());
    }

    let (my_start, your_start) = (('A', 'A'), ('A', 'A'));
    // we want the shortest path that visits all relievers in the order that minimizes loss
    // loss = total flow not being released per tick
    let mut heap = BinaryHeap::new();
    let first_hole = DoubleHole {
        me: Hole {
            name: my_start,
            minutes: minutes_left,
            loss: 0,
            vented: 0,
        },
        your_name: your_start
    };
        
    heap.push(first_hole);

    let mut min_loss_for = HashMap::new();
    min_loss_for.insert(first_hole.key(), 0u32);
    let mut min_loss = u32::MAX;

    while let Some(hole) = heap.pop() {
        if hole.me.minutes == 0 {
            min_loss = min_loss.min(hole.me.loss);
            break;
        }
        if hole.me.vented == (1 << losers) - 1 {
            min_loss = min_loss.min(hole.me.loss);
            break;
        }

        if hole.me.loss > min_loss_for.get(&hole.key()).copied().unwrap_or(u32::MAX) {
            continue;
        }

        if let Some(index) = (0..losers).find(|&i| suspicious[i].name == hole.me.name) {
            if hole.me.vented & (1 << index) == 0 {
                if let Some(your_index) = (0..losers).find(|&i| suspicious[i].name == hole.your_name) {
                    if hole.me.vented & (1 << your_index) == 0 {
                        let vent_hole = DoubleHole {
                            me: Hole {
                                name: hole.me.name,
                                minutes: hole.me.minutes - 1,
                                loss: hole.me.loss + compute_loss(hole.me.vented, losers, &suspicious),
                                vented: hole.me.vented | (1 << index) | (1 << your_index)
                            },
                            your_name: hole.your_name
                        };

                        if vent_hole.me.loss < min_loss_for.get(&vent_hole.key()).copied().unwrap_or(u32::MAX) {
                            heap.push(vent_hole);
                            min_loss_for.insert(vent_hole.key(), vent_hole.me.loss);
                        }
                    }
                }
                for &your_neighbor in neighbors[&hole.your_name].iter() {
                    let vent_hole = DoubleHole {
                        me: Hole {
                            name: hole.me.name,
                            minutes: hole.me.minutes - 1,
                            loss: hole.me.loss + compute_loss(hole.me.vented, losers, &suspicious),
                            vented: hole.me.vented | (1 << index)
                        },
                        your_name: your_neighbor
                    };
        
                    if vent_hole.me.loss < min_loss_for.get(&vent_hole.key()).copied().unwrap_or(u32::MAX) {
                        heap.push(vent_hole);
                        min_loss_for.insert(vent_hole.key(), vent_hole.me.loss);
                    }
                }
            }
        }

        for &neighbor in neighbors[&hole.me.name].iter() {
            if let Some(your_index) = (0..losers).find(|&i| suspicious[i].name == hole.your_name) {
                if hole.me.vented & (1 << your_index) == 0 {
                    let vent_hole = DoubleHole {
                        me: Hole {
                            name: neighbor,
                            minutes: hole.me.minutes - 1,
                            loss: hole.me.loss + compute_loss(hole.me.vented, losers, &suspicious),
                            vented: hole.me.vented | (1 << your_index)
                        },
                        your_name: hole.your_name
                    };

                    if vent_hole.me.loss < min_loss_for.get(&vent_hole.key()).copied().unwrap_or(u32::MAX) {
                        heap.push(vent_hole);
                        min_loss_for.insert(vent_hole.key(), vent_hole.me.loss);
                    }
                }
            }
            for &your_neighbor in neighbors[&hole.your_name].iter() {
                let vent_hole = DoubleHole {
                    me: Hole {
                        name: neighbor,
                        minutes: hole.me.minutes - 1,
                        loss: hole.me.loss + compute_loss(hole.me.vented, losers, &suspicious),
                        vented: hole.me.vented
                    },
                    your_name: your_neighbor
                };
    
                if vent_hole.me.loss < min_loss_for.get(&vent_hole.key()).copied().unwrap_or(u32::MAX) {
                    heap.push(vent_hole);
                    min_loss_for.insert(vent_hole.key(), vent_hole.me.loss);
                }
            }
        }
    }
    total_possible_loss - min_loss
}
