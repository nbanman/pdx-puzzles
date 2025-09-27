use std::cmp::Reverse;
use std::collections::BinaryHeap;
use indexmap::IndexMap;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = (Edges, usize, usize);
type Output = usize;
type Edges = Vec<Vec<EdgeInfo>>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Side { Outer, Inner, }

#[derive(Debug, Clone)]
struct Portal {
    name: String,
    side: Side
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: usize,
    level: isize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level.cmp(&other.level).then(self.pos.cmp(&other.pos))
    }
}


#[derive(Debug, Copy, Clone)]
struct EdgeInfo {
    state: State,
    distance: usize,
}


fn parse_input(input: &str) -> Input {
    let maze: Grid2<char> = input.try_into().unwrap();
    let portal_rx = regex!(r"\.\w{2}|\w{2}\.");
    let mut portals: IndexMap<usize, Portal> = IndexMap::default();

    // hz
    for (row, chars) in maze.rows().enumerate() {
        let s = chars.iter().copied().join("");
        for mr in portal_rx.find_iter(&s) {
            let name = mr.as_str();
            let pos = row * maze.width() + if name.starts_with('.') {
                mr.start()
            } else {
                mr.end() - 1
            };
            let side = if mr.start() == 0 || mr.end() == s.len() {
                Side::Outer
            } else {
                Side::Inner
            };
            let portal = Portal {
                name: name.trim_matches('.').to_string(),
                side,
            };
            portals.insert(pos, portal);
        }
    }

    // vt
    for (col, chars) in maze.columns().enumerate() {
        let s = chars.iter().copied().join("");
        for mr in portal_rx.find_iter(&s) {
            let name = mr.as_str();
            let pos = col + maze.width() * if name.starts_with('.') {
                mr.start()
            } else {
                mr.end() - 1
            };
            let side = if mr.start() == 0 || mr.end() == s.len() {
                Side::Outer
            } else {
                Side::Inner
            };
            let portal = Portal {
                name: name.trim_matches('.').to_string(),
                side,
            };
            portals.insert(pos, portal);
        }
    }

    let vertex_map: FxHashMap<usize, usize> = portals.keys()
        .copied()
        .enumerate()
        .map(|(idx, port)| (port, idx))
        .collect();

    let get_portal_positions = |side: Side| {
        portals.iter()
                .filter(|(_, portal)| portal.side == side)
                .map(|(pos, portal)| (portal.name.clone(), *vertex_map.get(pos).unwrap()))
                .collect::<FxHashMap<String, usize>>()
    };

    let inner_portal_positions = get_portal_positions(Side::Inner);
    let outer_portal_positions = get_portal_positions(Side::Outer);

    let mut edges: Vec<Vec<EdgeInfo>> = Vec::new();
    for (&pos, portal) in portals.iter() {
        let mut next: Vec<(usize, usize)> = Vec::new();
        let mut visited: FxHashSet<usize> = FxHashSet::default();
        let mut q: Vec<(usize, usize)> = vec!((pos, 0));
        while let Some((current, dist)) = q.pop() {
            if current != pos && portals.contains_key(&current) {
                next.push((current, dist));
            } else {
                let neighbors = maze
                    .adjacent(current, false)
                    .unwrap();
                for neighbor in neighbors {
                    if *neighbor.value == '.' && !visited.contains(&neighbor.index) {
                        visited.insert(neighbor.index);
                        q.push((neighbor.index, dist + 1));
                    }
                }
            }
        }
        let mut neighbors: Vec<EdgeInfo> = next.into_iter()
            .map(|(neighbor, distance)| {
                let state = State {
                    pos: *vertex_map.get(&neighbor).unwrap(),
                    level: 0,
                };
                EdgeInfo { state, distance }
            })
            .collect();
        if portal.name != "AA" && portal.name != "ZZ" {
            let (warp_pos, warp_level) = match portal.side {
                Side::Outer => (*inner_portal_positions.get(&portal.name).unwrap(), -1),
                Side::Inner => (*outer_portal_positions.get(&portal.name).unwrap(), 1),
            };
            let state = State { pos: warp_pos, level: warp_level };
            let warp = EdgeInfo { state, distance: 1 };
            neighbors.push(warp);
        }
        edges.push(neighbors);
    }

    (
        edges,
        *outer_portal_positions.get("AA").unwrap(),
        *outer_portal_positions.get("ZZ").unwrap()
    )
}

fn find_exit(input: &Input, dimension_warp: bool) -> Output {
    let (edges, start, end) = input;
    let start = (0, State { pos: *start, level: 0 });
    let end = State { pos: *end, level: 0 };
    let mut q: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();
    q.push(Reverse(start));
    let mut vertices: FxHashMap<State, usize> = FxHashMap::default();

    loop {
        let Reverse((dist, state)) = q.pop().unwrap();
        if state == end {
            return dist;
        }
        for edge in edges[state.pos].iter() {
            if !dimension_warp || state.level != 0 || edge.state.level != -1 {
                let edge_level = state.level + if dimension_warp { edge.state.level } else { 0 };
                let edge_state = State { level: edge_level, ..edge.state };
                let alternate_dist = dist + edge.distance;
                let existing_dist = *vertices.get(&edge_state).unwrap_or(&usize::MAX);
                if alternate_dist < existing_dist {
                    vertices.insert(edge_state.clone(), alternate_dist);
                    q.push(Reverse((alternate_dist, edge_state)));
                }
            }
        }
    }
}

fn part1(input: &Input) -> Output {
    find_exit(input, false)
}

fn part2(input: &Input) -> Output {
    find_exit(input, true)
}

#[test]
fn default() {
    let input = get_input(19, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(528, part1(&input));
    assert_eq!(6214, part2(&input));
}

// Input parsed (2ms)
// 1. 528 (8μs)
// 2. 6214 (346μs)
// Total: 3ms