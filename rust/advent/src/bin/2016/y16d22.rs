use std::{cmp::Reverse, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::{Coord, Coord2U}, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Vec<Node>;
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Node {
    pos: Pos,
    size: usize,
    used: usize,
}

impl Node {
    fn available(&self) -> usize {
        self.size - self.used
    }
}

trait PosBounds {
    fn contains(&self, pos: Pos) -> bool;
}

impl PosBounds for (Pos, Pos) {
    fn contains(&self, pos: Pos) -> bool {
        let (tl, br) = self;
        (tl.x()..=br.x()).contains(&pos.x()) && (tl.y()..=br.y()).contains(&pos.y())
    }
}

trait Boundable {
    fn bounds(self) -> (Pos, Pos);
}

impl<T: Iterator<Item = Pos>> Boundable for T {
    fn bounds(self) -> (Pos, Pos) {
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for Coord([x, y]) in self {
            // let (x, y) = (*x, *y);
            if x < min_x {
                min_x = x;
            } else if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            } else if y > max_y {
                max_y = y;
            }
        }
        (Pos::new2d(min_x, min_y), Pos::new2d(max_x, max_y))
    }
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().tuples()
        .map(|(x, y, size, used, _, _)| Node { pos: Pos::new2d(x, y), size, used })
        .collect()
}

fn part1(nodes: &Input) -> Output {
    nodes.iter()
        .filter(|node| node.used != 0)
        .map(|node_a| {
            nodes.iter()
                .filter(|&node_b| node_b.available() >= node_a.used)
                .count()
        })
        .sum()
}

fn outer_edges(pos: Pos, empty: Pos, bounds: &(Pos, Pos), walls: &FxHashSet<Pos>) -> Vec<((Pos, Pos), usize)> {
    pos.adjacent(false).into_iter()
        .filter(|&neighbor| bounds.contains(neighbor) && !walls.contains(&neighbor))
        .map(|neighbor| {
            let weight = {
                let start_h = empty.manhattan_distance(neighbor);
                let start_vertex = (start_h, 0, empty);
                let mut open = BinaryHeap::new();
                open.push(Reverse(start_vertex));
                let mut closed = FxHashSet::default();
                let mut edge_weight = 0;
                while let Some(Reverse((_, weight, cur_pos))) = open.pop() {
                    if !closed.insert(cur_pos) { continue; }
                    if cur_pos == neighbor {
                        edge_weight = weight + 1;
                        break;
                    }
                    for inner_neighbor in cur_pos.adjacent(false).into_iter()
                        .filter(|&inner_neighbor| {
                            inner_neighbor != pos
                                && bounds.contains(inner_neighbor)
                                && !walls.contains(&inner_neighbor)
                        })
                    {
                        if !closed.contains(&inner_neighbor) {
                            let h = weight + 1 + inner_neighbor.manhattan_distance(neighbor);
                            open.push(Reverse((h, weight + 1, inner_neighbor)))
                        }
                    }
                }
                edge_weight
            };
            ((neighbor, pos), weight)
        })
        .collect()
}

// Double-nested A* implementation. The problem requires moving certain "goal data" from its original location
// to the top-left "origin." It can only move by swapping positions with an "empty spot." The empty spot can swap
// positions within the bounds of the nodes, but there are some spots with too much data for swapping. These are
// effectively "walls."
//
// The "outer" A* heuristic is the manhattan distance * 5 from the goal data to the origin, because the best-case
// scenario^ takes four moves to get the empty spot in front of the goal data, and one move to move into the empty
// spot. The state tracks both the goal data location and the empty spot.
//
// The "outer" edges are spots adjacent to the goal data within the bounds of the Node space, excluding "walls."
//
// The actual weight of each edge needs to be calculated. I used A* for this "inner" calculation. This time the
// heuristic is simply manhattan distance from the empty spot to the edge location. The "inner" edges are spots
// adjacent to the empty spot, excluding walls and the goal data itself.
//
// ^: Not truly a best-case scenario, if the empty spot is already adjacent to the goal data. This is an edge case I
// leave as an exercise to the next coder.
fn part2(nodes: &Input) -> Output {
    let bounds = nodes.iter().map(|node|node.pos).bounds();
    let walls: FxHashSet<Pos> = nodes.iter()
        .filter(|node| node.used > 400)
        .map(|node| node.pos)
        .collect();
    let goal_data = Pos::new2d(bounds.1.x(), 0);
    let empty = nodes.iter()
        .find(|node| node.used == 0)
        .map(|node| node.pos)
        .unwrap();

    let outer_heuristic = |pos: Pos| pos.manhattan_distance(Pos::origin()) * 5;
    let start_vertex = (outer_heuristic(goal_data), 0, (goal_data, empty));
    let mut open = BinaryHeap::new();
    open.push(Reverse(start_vertex));
    let mut closed = FxHashSet::default();
    while let Some(Reverse((_, weight, (pos, empty)))) = open.pop() {
        if !closed.insert(pos) { continue; }
        if pos == Pos::origin() {
            return weight;
        }
        let edges = outer_edges(pos, empty, &bounds, &walls);
        for ((neighbor, next_empty), neighbor_weight) in edges {
            if !closed.contains(&neighbor) {
                let h = weight + neighbor_weight + outer_heuristic(neighbor);
                open.push(Reverse((h, weight + neighbor_weight, (neighbor, next_empty))));
            }
        }
    }
    unreachable!()
}

#[test]
fn default() {
    let input = get_input(16, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(924, part1(&input));
    assert_eq!(213, part2(&input));
}

// Input parsed (122μs)
// 1. 924 (279μs)
// 2. 213 (274μs)
// Total: 678μs