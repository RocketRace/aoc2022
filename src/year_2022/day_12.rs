use std::collections::VecDeque;

#[derive(Debug)]
struct Graph {
    vertices: usize,
    start: usize,
    end: usize,
    neighbors: Vec<[usize; 4]>,
    reverse_neighbors: Vec<[usize; 4]>,
    grid: Vec<u8>
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Graph {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / (width + 1); // newline included
    let vertices = width * height;
    
    let mut neighbors = vec![[usize::MAX; 4]; vertices];
    let mut reverse_neighbors = vec![[usize::MAX; 4]; vertices];
    
    let mut grid: Vec<_> = input.bytes().filter(|&b| b != b'\n').collect();

    let start = (0..input.len()).find(|&i| grid[i] == b'S').unwrap();
    let end = (0..input.len()).find(|&i| grid[i] == b'E').unwrap();

    grid[start] = b'a';
    grid[end] = b'z';

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let elevation = grid[index];
            if x > 0 && grid[index - 1] <= elevation + 1 {
                neighbors[index][0] = index - 1;
                reverse_neighbors[index - 1][0] = index
            }
            if x < width - 1 && grid[index + 1] <= elevation + 1 {
                neighbors[index][1] = index + 1;
                reverse_neighbors[index + 1][1] = index
            }
            if y > 0 && grid[index - width] <= elevation + 1 {
                neighbors[index][2] = index - width;
                reverse_neighbors[index - width][2] = index
            }
            if y < height - 1 && grid[index + width] <= elevation + 1 {
                neighbors[index][3] = index + width;
                reverse_neighbors[index + width][3] = index
            }
        }
    }

    Graph { vertices, start, end, neighbors, reverse_neighbors, grid }
}

#[derive(PartialEq, Eq)]
struct Node {
    index: usize,
    distance: u32
}

#[derive(Debug, Clone, Copy)]
enum Part {
    One,
    Two
}

// dijkstra's algorithm is over twice as slow with the given input
fn bfs(graph: &Graph, part: Part) -> Option<u32> {
    let mut distances = vec![u32::MAX; graph.vertices];
    let mut heap = VecDeque::new();
    
    let first = match part {
        Part::One => graph.start,
        Part::Two => graph.end,
    };

    heap.push_back(Node {index: first, distance: 0});
    distances[first] = 0;

    while let Some( Node { index, distance } ) = heap.pop_front() {
        // found
        match part {
            Part::One => if index == graph.end {
                return Some(distance);
            }
            Part::Two => if graph.grid[index] == b'a' {
                return Some(distance);
            }
        }
        // suboptimal
        if distance > distances[index] {
            continue;
        }
        let neighbors = match part {
            Part::One => &graph.neighbors,
            Part::Two => &graph.reverse_neighbors
        };
        for neighbor in neighbors[index] {
            // ooh improvement
            if neighbor != usize::MAX && distance + 1 < distances[neighbor] {
                heap.push_back(Node { distance: distance + 1, index: neighbor });
                distances[neighbor] = distance + 1;
            }
        }
    }
    None // :(
}

#[aoc(day12, part1)]
fn climb_up(grid: &Graph) -> u32 {
    bfs(grid, Part::One).unwrap()
}

#[aoc(day12, part2)]
fn climb_down(grid: &Graph) -> u32 {
    bfs(grid, Part::Two).unwrap()
}

