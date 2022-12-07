use std::collections::HashMap;

enum Node {
    File(u64),
    // size is populated after construction
    Directory {
        children: HashMap<String, Node>,
        parent: *mut Node,
        size: u64,
    },
}

#[aoc_generator(day7)]
fn generator(input: &str) -> Node {
    let mut tree = Node::Directory {
        children: HashMap::new(),
        parent: std::ptr::null_mut(),
        size: 0,
    };
    let mut cwd = &mut tree;
    for cmd in input.split("$ ").skip(1) {
        let mut lines = cmd.lines();
        let input = lines.next().expect("No command");
        match input.split_once(' ') {
            Some((_, dir)) => match dir {
                "/" => cwd = &mut tree,
                ".." => {
                    if let Node::Directory { parent, .. } = *cwd {
                        if !parent.is_null() {
                            cwd = unsafe { &mut *parent };
                        }
                    }
                }
                folder => {
                    if let Node::Directory { children, .. } = cwd {
                        cwd = children.get_mut(folder).expect("directory not found");
                    }
                }
            },
            None => {
                let parent = cwd as *mut Node;
                if let Node::Directory { children, .. } = cwd {
                    children.extend(lines.map(|output| {
                        let (prefix, filename) = output.split_once(' ').expect("Bad ls output");
                        let child = match prefix.parse::<u64>() {
                            Ok(size) => Node::File(size),
                            Err(_) => Node::Directory {
                                children: HashMap::new(),
                                parent,
                                size: 0,
                            },
                        };
                        (filename.to_string(), child)
                    }));
                };
            }
        }
    }
    count_sizes(&mut tree);
    tree
}

fn count_sizes(input: &mut Node) -> u64 {
    match input {
        Node::File(size) => *size,
        Node::Directory { children, size, .. } => {
            *size = children.values_mut().map(count_sizes).sum();
            *size
        }
    }
}

fn sum_directories_under(tree: &Node, limit: u64) -> u64 {
    match tree {
        Node::File(_) => 0,
        Node::Directory { children, size, .. } => {
            let init = if *size < limit { *size } else { 0 };
            init + children
                .values()
                .map(|child| sum_directories_under(child, limit))
                .sum::<u64>()
        }
    }
}

#[aoc(day7, part1)]
fn removable_dirs(tree: &Node) -> u64 {
    sum_directories_under(tree, 100_000)
}

fn find_smallest_satisfying(tree: &Node, free_space_needed: u64) -> Option<u64> {
    match tree {
        Node::File(_) => None,
        Node::Directory { children, size, .. } => {
            (*size >= free_space_needed).then(|| {
                // if we remove this directory, then we definitely have enough space
                children
                    .values()
                    .filter_map(|child| find_smallest_satisfying(child, free_space_needed))
                    .fold(*size, |lowest, child| lowest.min(child))
            })
        }
    }
}

#[aoc(day7, part2)]
fn best_removable_dir(tree: &Node) -> u64 {
    match tree {
        Node::File(_) => unreachable!("root is not a directory"),
        Node::Directory { size, .. } => {
            find_smallest_satisfying(tree, *size + 30_000_000 - 70_000_000).expect("none found")
        }
    }
}
