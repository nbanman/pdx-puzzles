use everybody_codes::utilities::inputs::get_event_inputs;
use indexmap::IndexMap;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;
use utilities::enums::cardinals::Cardinal;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use utilities::structs::str_grid::{AdjacentMetadata, StrGrid};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, _input2, _input3) = get_event_inputs(24, 20);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    // println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    let (grid, start_id) = parse_input(input);
    let heuristic = |e: &AdjacentMetadata<usize>| {
        grid.height - 1 - e.pos / grid.width
    };
    let get_edges = |e: &AdjacentMetadata<usize>| {
        glide_paths(&grid, e)
            .map(|neighbor| {
                let weight = match neighbor.b {
                    b'+' => 1,
                    b'.' | b'S' => 3,
                    b'-' => 4,
                    b => panic!("{} not valid character!", b as char),
                };
                (neighbor, weight)
            })
            .collect()
    };
    astar(start_id, heuristic, get_edges)
}

fn part2(input: Input) -> usize {
    todo!()
}

fn part3(input: Input) -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Hash)]
struct Vertex<E: PartialEq + PartialOrd + Clone> {
    id: E,
    weight: usize,
    h: usize,
    parent: Option<usize>,
}

impl <E: PartialEq + PartialOrd + Clone> PartialOrd for Vertex<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let af = self.weight + self.h;
        let bf = other.weight + other.h;
        af.partial_cmp(&bf)
    }
}

fn astar<E, H, G>(start_id: E, heuristic: H, get_edges: G) -> usize
where
    E: PartialEq + PartialOrd + Ord + Hash + Clone,
    H: Fn(&E) -> usize,
    G: Fn(&E) -> Vec<(E, usize)>,
{
    let h = heuristic(&start_id);
    let start_vertex = Vertex {
        id: start_id,
        weight: 0,
        h,
        parent: None,
    };
    let mut open: BinaryHeap<Reverse<Vertex<E>>> = BinaryHeap::new();
    open.push(Reverse(start_vertex));
    let mut closed: IndexMap<E, Vertex<E>> = IndexMap::new();

    while let Some(Reverse(v)) = open.pop() {
        if closed.contains_key(&v.id) { continue; }
        if v.h == 0 {
            let mut parent_tracker = v.parent;
            let mut steps = 1;
            while let Some(p_index) = parent_tracker {
                steps += 1;
                let (_, p_vertex) = closed.get_index(p_index).unwrap();
                parent_tracker = p_vertex.parent;
            }
            let altitude = 1000 - (v.weight - 2 * steps);
            return altitude + 100 - steps;
        }
        closed.insert(v.id.clone(), v.clone());
        for (edge, edge_weight) in get_edges(&v.id) {
            if !closed.contains_key(&edge) {
                let h = heuristic(&edge);
                open.push(Reverse(Vertex {
                    id: edge,
                    weight: v.weight + edge_weight,
                    h,
                    parent: Some(closed.len() - 1),
                }));
            }
        }
    }
    unreachable!();
}

fn parse_input(input: Input<'_>) -> (StrGrid<'_>, AdjacentMetadata<usize>) {
    let map = StrGrid::new(input).unwrap();
    let start = map.s.iter().enumerate()
        .find(|(_, s)| **s == b'S')
        .unwrap()
        .0;
    let start = AdjacentMetadata {
        pos: start,
        dir: Cardinal::South,
        b: b'S',
    };
    (map, start)
}

fn glide_paths(
    map: &StrGrid<'_>, state: &AdjacentMetadata<usize>
) -> impl Iterator<Item = AdjacentMetadata<usize>> {
    [state.dir.left(), state.dir, state.dir.right()].into_iter()
        .filter_map(|new_dir| map.move_direction(state.pos, new_dir))
        .filter(|adjacent| adjacent.b != b'#')
}

#[test]
fn default() {
    let (_input1, _input2, _input3) = get_event_inputs(24, 20);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn examples() {
    let inputs = [r"#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#", ];
    assert_eq!(1045, part1(inputs[0]));
    // assert_eq!(YY, part2(inputs[1]));
    // assert_eq!(YY, part3(inputs[2]));
}