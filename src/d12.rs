/// AoC 2021 -- Day 12
/// https://adventofcode.com/2021/day/12
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use crate::util;

const DEBUG: bool = true;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node(String);

impl Node {
    pub fn new(name: &str) -> Self {
        Node(name.to_string())
    }

    pub fn is_small(&self) -> bool {
        self.0.chars().all(|c| c.is_lowercase())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Finite path through a graph
type Path = Vec<Node>;
type PathSlice<'a> = &'a [Node];

pub fn fmt_path(v: PathSlice) -> String {
    let ns: Vec<String> = v.iter().map(|n| format!("{}", n)).collect();
    format!("[{}]", ns.join(", "))
}

/// A directed graph, represented as a map from nodes to neighbors
pub struct Graph {
    neighbors: HashMap<Node, Vec<Node>>
}

const MAX_STEPS: usize = 100000;

impl Graph {
    /// Enumerate all paths in ``self`` from start to end that traverse each small node at most
    /// once.
    ///
    /// Implementation is an iterative depth-first search through the graph, pruning node
    /// small node visits whenever the small node is alread on the path visited so far.
    ///
    /// Search loop tracks:
    /// - current path (last node on current path is always the top current node on the call stack)
    /// - call stack that tracks each visited node, along with a stack of nodes to visit from there
    ///
    /// Example:
    ///
    /// ```text
    ///     start
    ///     /   \
    /// c--A-----b--d
    ///     \   /
    ///      end
    /// ```
    ///
    /// current path:      call stack:
    /// -------------      -----------
    /// start              <(start, [b, A])>
    /// --> visit A, append A to current path, push (A, A-nbs) onto call stack
    /// start-A            <(start, [b]), (A, [start, c, b, end])>
    /// --> visit end: output start-A-end
    /// start-A            <(start, [b]), (A, [start, c, b])>
    /// --> visit b, append b to current path, push (b, b-nbs) onto call stack
    /// start-A-b          <(start, [b]), (A, [start, c]), (b, [start, d, end, b])>
    /// ...
    /// start-A-b          <(start, [b]), (A, [start, c]), (b, [])>
    /// --> top of call stack nbs is empty: pop call stack, pop  current path
    /// start-A            <(start, [b]), (A, [start, c])>
    /// --> visit a, append c to current path, push (c, c-nbs) onto call stack
    /// start-A-c          <(start, [b]), (A, [start]), (c, [A])>
    /// ...
    ///
    /// Parameters:
    /// - start: starting node
    /// - end: ending node
    /// - max_ssn: maximum number of times that a single small node is allowed to appear in
    ///     each result path. Other small nodes are limited to 1 visit, as is the start node
    ///     and the end node.
    pub fn list_paths(&self, start: Node, end: Node, max_ssn: usize) -> Vec<Path> {
        let mut result_paths: HashSet<Vec<Node>> = HashSet::new();
        // initial call stack has the start and its neighbors
        let mut call_stack = vec![self.neighbors[&start].to_vec()];
        let mut current_path: Path = vec![start.clone()];
        let mut current_path_set: HashSet<Node> = current_path.iter().cloned().collect();

        let mut steps: usize = 0;
        while !call_stack.is_empty() && steps < MAX_STEPS {
            steps += 1;

            let current_stack: &mut Path = call_stack.last_mut().unwrap();
            if DEBUG {
                println!(
                    "current_path {}, stack {}",
                    fmt_path(&current_path),
                    fmt_path(current_stack),
                );
            }

            if current_stack.is_empty() {
                if DEBUG {
                    println!("current level exhusted, popping current path, popping call stack");
                }
                let current_path_end = current_path.pop().unwrap();
                current_path_set.remove(&current_path_end);
                call_stack.pop();
                continue;
            }

            let next_node = current_stack.pop().unwrap();

            // if next_node is the end, add the current path, plus 'end' to the result path set
            if next_node == end {
                let mut rpath = current_path.clone();
                rpath.push(next_node);
                result_paths.insert(rpath);
                continue;
            }

            // visit next_node: collect current node's neighbors and push them as a new level on
            // the call stack
            current_path.push(next_node.clone());
            current_path_set.insert(next_node);
            let current_node = current_path.last().unwrap();
            let nbs: Vec<Node> = self.neighbors[current_node].iter()
                // filtering here cuts number of iterations and vector copies in half in test cases
                .filter(|n| is_admissible(n, &current_path, &start, max_ssn))
                .cloned()
                .collect();
            if DEBUG {
                println!("visiting {}, pushing neighbors on the stack {}", current_node, fmt_path(&nbs));
            }
            call_stack.push(nbs);
        }
        if DEBUG {
            if steps >= MAX_STEPS {
                println!("*** Ran out of fuel at step {} ***", steps);
            } else {
                println!("*** Total steps {} ***", steps);
            }
        }
        result_paths.into_iter().collect()
    }
}

/// Predicate used to filter nodes to-be-visited
///
/// A test node is admissible to be visted iff:
///
/// 0. node is not 'start'
/// 1. node is big
/// 2. node is small, path is not yet maxed
/// 3. node is small, path is maxed, but test node is not on path
///
/// params:
/// - test_node: potential node to visit
/// - start: ref to the unique start node (which is never admissible to visit)
/// - current_path: the current path explored so far
/// - max_ssn: maximum number of times a (s)ingle (s)mall (n)ode may be visited on any
///     admissible path
fn is_admissible(test_node: &Node, current_path: PathSlice, start: &Node, max_ssn: usize) -> bool {
    if *test_node == *start {
        return false;
    } else if !test_node.is_small() {
        return true;
    }
    let mut counts: HashMap<Node, usize> = HashMap::new();
    for p in current_path.iter().cloned() {
            let e = counts.entry(p).or_insert(0);
            *e += 1;
    }
    let max_small_count = counts.iter()
        .filter(|(n, _c)| n.is_small())
        .map(|(_n, c)| *c)
        .max()
        .unwrap();
    let test_node_count = *counts.get(test_node).unwrap_or(&0);
    let path_has_the_max = max_small_count >= max_ssn;
    let path_has_the_node = test_node_count > 0;
    if DEBUG {
        println!(
            "     isa: path {}, test_node {}, path_has_the_max {}, path_has_the_node {}, max_small_count {}, max_ssn {}",
            fmt_path(current_path), test_node, path_has_the_max, path_has_the_node, max_small_count, max_ssn,
        );
    }
    !path_has_the_max || !path_has_the_node
}

// render graphs on the terminal, for fun and laughs
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (k, v) in self.neighbors.iter() {
            let prefix = if k.is_small() {"(small)"} else {"(big)  "};
            result += &format!("{} {} -> {}\n", prefix, k, fmt_path(v));
        }
        writeln!(f, "{}", result)
    }
}

pub fn parse_input(input_file: &str) -> Graph {
    let content = util::read_to_string(input_file).unwrap();
    parse_input_from_string(&content)
}

pub fn parse_input_from_string(content: &str) -> Graph {
    let edges: Vec<(Node, Node)> = content
        .trim()
        .split('\n')
        .map(|line| {
            let mut ps = line.trim().split('-');
            let s = Node(ps.next().expect("could not parse edge start node").to_string());
            let e = Node(ps.next().expect("could not parse edge end node").to_string());
            (s, e)
        })
        .collect();
    let mut neighbors: HashMap<Node, Vec<Node>> = HashMap::new();
    for (s, e) in edges {
        let s2 = s.clone();
        let e2 = e.clone();
        // add edge s -> e
        let snbds = neighbors.entry(s).or_insert_with(Vec::new);
        snbds.push(e);
        // add edge e -> s
        let enbds = neighbors.entry(e2).or_insert_with(Vec::new);
        enbds.push(s2);
    }
    Graph{neighbors}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph_parser() {
        let test_graph = parse_input("inputs/d12_test");
        println!("d12_test graph:\n{}", test_graph);

        let graph = parse_input("inputs/d12");
        println!("d12 graph:\n{}", graph);
    }

    #[test]
    fn test_list_paths_test_graph1() {
        let test_graph = parse_input("inputs/d12_test");
        let paths = test_graph.list_paths(Node::new("start"), Node::new("end"), 1);
        println!("d12 test_graph1 paths:");
        for (i, p) in paths.iter().enumerate() {
            println!("{}: {}", i, fmt_path(p));
        }
        assert_eq!(paths.len(), 10);
    }

    #[test]
    fn test_list_paths_test_graph2() {
        let test_graph = parse_input("inputs/d12_test2");
        let paths = test_graph.list_paths(Node::new("start"), Node::new("end"), 1);
        assert_eq!(paths.len(), 19);
    }

    #[test]
    fn test_list_paths_test_graph3() {
        let test_graph = parse_input("inputs/d12_test3");
        let paths = test_graph.list_paths(Node::new("start"), Node::new("end"), 1);
        assert_eq!(paths.len(), 226);
    }

    #[test]
    fn test_d12_part1() {
        let test_graph = parse_input("inputs/d12");
        let paths = test_graph.list_paths(Node::new("start"), Node::new("end"), 1);
        assert_eq!(paths.len(), 5178);
    }
}
