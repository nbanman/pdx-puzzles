use std::{cmp::Reverse, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::{
    enums::cardinals::Cardinal, graphs::EdgeInfo, parsing::get_numbers::ContainsNumbers, structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch}, store::Store,
    }
};

type Input = Cavern;
type Output = usize;
type Pos = Coord2U;

struct Cavern {
    erosion_map: FxHashMap<Pos, usize>,
    depth: usize,
    target: Pos,
}

impl From<&str> for Cavern {
    fn from(value: &str) -> Self {
        let (depth, x, y) = value.get_numbers().collect_tuple().unwrap();
        let target = Pos::new2d(x, y);
        let mut erosion_map: FxHashMap<Pos, usize> = FxHashMap::default();
        erosion_map.insert(Pos::origin(), Self::erosion_level(depth, 0));
        erosion_map.insert(target, Self::erosion_level(depth, 0));
        Self { erosion_map, depth, target }
    }
}

impl Cavern {
    fn erosion_level(depth: usize, v: usize) -> usize {
        (v + depth) % 20183
    }

    fn get_erosion(&mut self, pos: Pos) -> usize {
        if let Some(erosion) = self.erosion_map.get(&pos) {
            *erosion
        } else {
            let erosion = if pos.y() == 0 {
                Self::erosion_level(self.depth, pos.x() * 16807)
            } else if pos.x() == 0 {
                Self::erosion_level(self.depth, pos.y() * 48271)
            } else {
                Self::erosion_level(
                    self.depth,
                    self.get_erosion(pos.move_direction(Cardinal::West, 1).unwrap())
                        * self.get_erosion(pos.move_direction(Cardinal::North, 1).unwrap()),
                )
            };
            self.erosion_map.insert(pos, erosion);
            erosion
        }
    }

    fn get_terrain(&mut self, pos: Pos) -> Terrain {
        Terrain::ENTRIES[self.get_erosion(pos) % 3]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

impl Terrain {
    const ENTRIES: [Self; 3] = [Self::Rocky, Self::Wet, Self::Narrow];

    fn ordinal(&self) -> usize {
        match self {
            Terrain::Rocky => 0,
            Terrain::Wet => 1,
            Terrain::Narrow => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Tool {
    Gear,
    Torch,
    Neither,
}

impl Tool {
    const ENTRIES: [Self; 3] = [Self::Gear, Self::Torch, Self::Neither];
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 22).unwrap();
    let mut input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&mut input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&mut input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.into()
}

fn part1(cavern: &mut Input) -> Output {
    let mut sum = 0;
    Pos::for_rectangle(Pos::origin(), cavern.target, |pos| {
        sum += cavern.get_terrain(pos).ordinal()
    });
    sum
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Pos,
    tool: Tool,
    f: usize,
    edge_info: EdgeInfo<usize>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.f.partial_cmp(&other.f) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.pos.partial_cmp(&other.pos) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.tool.partial_cmp(&other.tool) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.edge_info.partial_cmp(&other.edge_info)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part2(cavern: &mut Input) -> Output {
    let target = cavern.target;
    let heuristic = |pos: Pos| pos.manhattan_distance(target);

    let start = State {
        pos: Pos::origin(),
        tool: Tool::Torch,
        f: heuristic(Pos::origin()),
        edge_info: EdgeInfo { cost: 0, parent: None },
    };

    let mut open = BinaryHeap::new();
    open.push(Reverse(start));
    let mut closed: Store<(Pos, Tool), EdgeInfo<usize>> = Store::new();
    while let Some(Reverse(State { pos, tool, f, edge_info })) = open.pop() {
        let st = (pos, tool);
        if closed.contains(&st) { continue; }
        let id = closed.assign(st, edge_info);
        if pos == target {
            return f;
        }
        let neighbors = pos.adjacent(false).into_iter()
            .filter_map(|neighbor| {
                let neighbor_tool = change_tool(
                    cavern.get_terrain(neighbor),
                    cavern.get_terrain(pos),
                )
                    .unwrap_or(tool);
                if closed.contains(&(neighbor, neighbor_tool)) {
                    None
                } else {
                    let cost = edge_info.cost + if tool != neighbor_tool { 8 } else { 1 };
                    let end_mod = if neighbor == cavern.target && neighbor_tool != Tool::Torch { 7 } else { 0 };
                    let neighbor_edge_info = EdgeInfo { cost: cost + end_mod, parent: id };
                    let neighbor_f = neighbor_edge_info.cost + heuristic(neighbor);
                    Some(State {
                        pos: neighbor,
                        tool: neighbor_tool,
                        f: neighbor_f,
                        edge_info: neighbor_edge_info,
                    })
                }
            });
        for neighbor in neighbors {
            open.push(Reverse(neighbor));
        }
    }
    unreachable!()
}

fn change_tool(neighbor_terrain: Terrain, state_terrain: Terrain) -> Option<Tool> {
    if neighbor_terrain == state_terrain {
        None
    } else {
        Some(Tool::ENTRIES[state_terrain.ordinal() + neighbor_terrain.ordinal() - 1])
    }
}

#[test]
fn default() {
    let input = get_input(18, 22).unwrap();
    let mut input = parse_input(&input);
    assert_eq!(5637, part1(&mut input));
    assert_eq!(969, part2(&mut input));
}

// Input parsed (13μs)
// 1. 5637 (308μs)
// 2. 969 (77ms)
// Total: 77ms