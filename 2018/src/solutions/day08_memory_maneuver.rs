#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

/// Recursively decodes a node (and any children)
///
/// Where a node consists of:
/// *A header, which is always exactly two numbers:
/// ** The quantity of child nodes.
/// ** The quantity of metadata entries.
/// *Zero or more child nodes (as specified in the header).
/// * One or more metadata entries (as specified in the header).
fn parse_node(iter: &mut std::str::Split<'_, &str>) -> Node {
    let mut node = Node::default();
    // Extract the header
    let child_count = iter.next().unwrap().parse::<usize>().unwrap();
    let metadata_count = iter.next().unwrap().parse::<usize>().unwrap();
    // Extract the specified number of children
    for _ in 0..child_count {
        node.children.push(parse_node(iter));
    }
    // Extract the specified number of metadata fields
    for _ in 0..metadata_count {
        node.metadata
            .push(iter.next().unwrap().parse::<usize>().unwrap());
    }
    node
}

/// Recursively sums all metadata fields contained in this and any child nodes
fn sum_metadata(node: &Node) -> usize {
    node.metadata.iter().sum::<usize>()
        + node.children.iter().map(|n| sum_metadata(n)).sum::<usize>()
}

/// Recursively calculates the value of a node
///
/// Where:
/// * If a node has no child nodes, its value is the sum of its metadata entries.
///   So, the value of node B is 10+11+12=33, and the value of node D is 99.
/// * However, if a node does have child nodes, the metadata entries become indexes which refer to those child nodes.
///   A metadata entry of 1 refers to the first child node, 2 to the second, 3 to the third, and so on.
///   The value of this node is the sum of the values of the child nodes referenced by the metadata entries.
///   If a referenced child node does not exist, that reference is skipped.
///   A child node can be referenced multiple time and counts each time it is referenced.
///   A metadata entry of 0 does not refer to any child node.
fn node_value(node: &Node) -> usize {
    if node.children.is_empty() {
        // No children, simply sum up metadata entries
        node.metadata.iter().sum::<usize>()
    } else {
        node.metadata
            .iter()
            .map(|&index| {
                // Get the value of the child node referenced by this metadata entry
                // if it exists, else just return 0
                node.children
                    .get(index - 1)
                    .map_or(0, |child| node_value(child))
            })
            .sum::<usize>()
    }
}

#[aoc_generator(day8)]
fn gen(input: &str) -> Node {
    parse_node(&mut input.split(" "))
}

#[aoc(day8, part1)]
fn part1(input: &Node) -> usize {
    sum_metadata(input)
}

#[aoc(day8, part2)]
fn part2(input: &Node) -> usize {
    node_value(input)
}
