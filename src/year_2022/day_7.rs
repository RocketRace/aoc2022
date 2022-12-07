use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Node {
    File(u64),
    // size is populated after construction
    Directory {children: HashMap<String, Node>, parent: *mut Node, size: u64}
}


#[aoc_generator(day7)]
fn generator(input: &str) -> Node {
    let mut tree = Node::Directory { children: HashMap::new(), parent: std::ptr::null_mut(), size: 0 };
    let mut cwd = &mut tree;
    for cmd in input.split("$ ").skip(1) {
        let mut lines = cmd.lines();
        let input = lines.next().expect("No command");
        match input.split_once(' ') {
            Some((_, dir)) => {
                // cd
                match dir {
                    "/" => {
                        cwd = &mut tree;
                    }
                    ".." => {
                        let Node::Directory { parent, .. } = *cwd else {
                            unreachable!("cwd is not directory")
                        };
                        assert!(!parent.is_null(), "root directory has no parent");
                        // we've checked parent to be non-null
                        // everything else it could point to exists in `tree`
                        // also cwd is never aliased
                        cwd = unsafe {
                            &mut *parent
                        };
                    }
                    folder => {
                        let Node::Directory { children, .. } = cwd else {
                            unreachable!("cwd is not directory")
                        };
                        let child = children.get_mut(folder).expect("directory not found");
                        cwd = child;
                    }
                }
            }
            None => {
                // ls
                let ptr = cwd as *mut Node;
                let Node::Directory { children, .. } = cwd else {
                    unreachable!("cwd is not directory")
                };
                for output in lines {
                    let (prefix, filename) = output.split_once(' ').expect("Bad ls output");
                    let child = match prefix.parse::<u64>() {
                        Ok(size) => {
                            Node::File(size)
                        }
                        // "dir"
                        Err(_) => {
                            Node::Directory { children: HashMap::new(), parent: ptr, size: 0 }
                        }
                    };
                    children.insert(filename.to_string(), child);
                }
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
            let total = children.values_mut().fold(0, |tot, child| tot + count_sizes(child));
            *size = total;
            total
        },
    }
}

fn sum_directories_under(tree: &Node, limit: u64) -> u64 {
    match tree {
        Node::File(_) => 0,
        Node::Directory { children, size, .. } => {
            let init = if *size < limit {
                *size
            }
            else {
                0
            };
            children.values().fold(init, |tot, child| tot + sum_directories_under(child, limit))
        },
    }
}

#[aoc(day7, part1)]
fn removable_dirs(tree: &Node) -> u64 {
    sum_directories_under(tree, 100_000)
}

fn find_smallest_satisfying(tree: &Node, free_space_needed: u64) -> u64 {
    match tree {
        // should use option, but this is more ergonomic
        Node::File(_) => u64::MAX,
        Node::Directory { children, size, .. } => {
            let this_size = *size;
            // dbg!(total_used + total_free, free_space_needed);
            if this_size < free_space_needed {
                u64::MAX
            }
            else {
                // if we remove this directory, then we definitely have enough space
                // but maybe it also works with the children!
                children.values().fold(this_size, |lowest, child| 
                    lowest.min(find_smallest_satisfying(child, free_space_needed))
                )
            }
        },
    }
}

#[aoc(day7, part2)]
fn best_removable_dir(tree: &Node) -> u64 {
    let total_used = match tree {
        Node::File(_) => unreachable!("root is not a directory"),
        Node::Directory { size, .. } => *size,
    };
    let extra_free_needed = total_used + 30_000_000 - 70_000_000;
    find_smallest_satisfying(tree, extra_free_needed)
}

