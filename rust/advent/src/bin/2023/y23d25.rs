use std::collections::{HashMap, HashSet, VecDeque};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;
type Components<'a> = HashMap<&'a str, Vec<&'a str>>;
type BfsOutput<'a> = (&'a str, HashMap<&'a str, Option<&'a str>>);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn bfs<'a, F>(start: &'a str, neighbors: F) -> BfsOutput<'a>
where
    F: Fn(&'a str) -> Vec<&'a str>,
{
    let mut q: VecDeque<&str> = VecDeque::new();
    q.push_back(start);
    let mut visited: HashMap<&str, Option<&str>> = HashMap::new();
    visited.insert(start, None);
    let mut last: Option<&str> = None;

    while let Some(current) = q.pop_front() {
        last = Some(current);
        for neighbor in neighbors(current) {
            if !visited.contains_key(neighbor) {
                visited.insert(neighbor, Some(current));
                q.push_back(neighbor);
            }
        }
    }
    (last.unwrap(), visited)
}

fn path(bfs_output: BfsOutput<'_>) -> Vec<&'_ str> {
    let mut path = Vec::new();
    let (last, parents) = bfs_output;
    path.push(last);

    while let Some(&Some(parent)) = parents.get(path.last().unwrap()) {
        path.push(parent);
    }
    path.into_iter().rev().collect()
}

fn part1(input: Input) -> Output {
    let mut components: Components = Components::new();

    for line in input.trim_end().lines() {
        let (parent, children) = line.split_once(":").unwrap();
        let elements: Vec<&str> = children.split(' ').filter(|s| !s.is_empty()).collect();
        let entry = components.entry(parent).or_insert(Vec::new());
        for element in elements.iter() {
            entry.push(element);
        }
        for element in elements.iter() {
            components.entry(element).or_insert(Vec::new()).push(parent);
        }
    }

    // Get a node on the edge by taking a random node, running BFS and grabbing the farthest one.
    let (start, _blah) = bfs(components.keys().next().unwrap(), |pos: &str| {
        components[pos].clone()
    });

    // Run bfs from the start node three times, each time removing edges in the path taken. This will saturate
    // the 3 edges to be cut.
    let mut cut_edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for _ in 1..=3 {
        let path = path(bfs(start, |pos| {
            let edges = components[pos]
                .iter()
                .filter(|&&component| {
                    cut_edges
                        .get(pos)
                        .map(|set| !set.contains(component))
                        .unwrap_or(true)
                })
                .cloned()
                .collect();
            edges
        }));
        for (prev, next) in path.into_iter().tuple_windows() {
            cut_edges.entry(prev).or_insert(HashSet::new()).insert(next);
        }
    }

    // Run bfs one more time. Since all the bridge edges are removed, this will only find the nodes on one side.
    let (_, group_a) = bfs(start, |pos| {
        components[pos]
            .iter()
            .filter(|&&component| {
                cut_edges
                    .get(pos)
                    .map(|set| !set.contains(component))
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    });
    group_a.len() * (components.len() - group_a.len())
}

#[test]
fn default() {
    let input = get_input(23, 25).unwrap();
    assert_eq!(569904, part1(&input));
}

// Input parsed (17μs)
// 1. 569904 (1ms)
// Total: 1ms
