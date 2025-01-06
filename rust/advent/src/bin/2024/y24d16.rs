use std::{borrow::BorrowMut, cmp::Reverse, collections::{BinaryHeap, HashSet}};

use advent::utilities::get_input::get_input;
use utilities::{enums::cardinals::Cardinal, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 16).unwrap();
    let (paths, end) = get_paths(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&paths, end), stopwatch.lap().report());
    println!("2. {} ({})", part2(&paths, end), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State(usize);

impl State {
    pub fn new(pos: usize, dir: Cardinal) -> Self {
        Self((pos << 2) + dir.ordinal())
    }

    pub fn destruct(&self) -> (usize, Cardinal) {
        (self.0 >> 2, Cardinal::entries()[self.0 & 3])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Path {
    weight: usize,
    parent: Option<usize>,
    alternate_paths: Vec<usize>
}

fn get_paths<'a>(input: Input) -> (Vec<Option<Path>>, usize) {
    let maze = input.as_bytes();
    let width = input.find('\n').unwrap() + 1;
    let start = input.find('S').unwrap();
    let end = input.find('E').unwrap();
    
    let mut best_path = usize::MAX;
    let mut paths: Vec<Option<Path>> = vec![None; maze.len() * 4]; 

    let start_state = State::new(start, Cardinal::East);
    paths[start_state.0] = Some(Path {
        weight: 0,
        parent: None,
        alternate_paths: Vec::new(),
    });

    let mut q: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();
    q.push(Reverse((0, start_state)));

    while let Some(Reverse((weight, current))) = q.pop() {
        if maze[current.0 >> 2] == b'#' { println!("{:?}", current) };
        if weight > best_path { break; }
        let (pos, _) = current.destruct();
        if pos == end { best_path = weight; }
        
        for (neighbor_weight, neighbor) in get_edges(current, maze, width) {
            let alternate_weight = weight + neighbor_weight;
            let existing_weight = paths[neighbor.0]
                .as_mut()
                .map(|path| path.weight)
                .unwrap_or(usize::MAX);
            if alternate_weight < existing_weight && alternate_weight <= best_path {
                if paths[neighbor.0] == None {
                    paths[neighbor.0] = Some(Path {
                        weight: alternate_weight,
                        parent: Some(current.0),
                        alternate_paths: Vec::new(),
                    });
                } else {
                    let path = paths[neighbor.0].as_mut().unwrap().borrow_mut();
                    (*path).weight = alternate_weight;
                    (*path).parent = Some(current.0);
                }

                q.push(Reverse((alternate_weight, neighbor)));
            }
            if alternate_weight == existing_weight {
                paths[neighbor.0].as_mut().unwrap().alternate_paths.push(current.0);
            }
        }
    }
    (paths, end)
}

fn get_edges(state: State, maze: &[u8], width: usize) -> Vec<(usize, State)> {
    let (pos, dir) = state.destruct();
    let edges: Vec<(usize, State)> = [dir, dir.left(), dir.right()].into_iter()
        .filter_map(move|new_dir| {
            let new_pos = match new_dir {
                Cardinal::North => pos.checked_sub(width),
                Cardinal::East => Some(pos + 1),
                Cardinal::South => Some(pos + width),
                Cardinal::West => Some(pos - 1),
            }?;
            let block = maze.get(new_pos)?;
            if *block == b'#' {
                None
            } else {
                let weight = if dir == new_dir { 1 } else { 1001 };
                Some((weight, State::new(new_pos, new_dir)))
            }
        })
        .collect();
    edges
}

fn part1(paths: &Vec<Option<Path>>, end: usize) -> Output {
    Cardinal::entries().into_iter()
        .filter_map(|entry| {
            let index = entry.ordinal() + (end << 2);
            paths[index].as_ref()
        })
        .next()
        .unwrap()
        .weight
}

fn part2(paths: &Vec<Option<Path>>, end: usize) -> Output {
    let mut seats: HashSet<usize> = HashSet::new();
    seats.insert(end);
    for entry in Cardinal::entries() {
        let index = entry.ordinal() + (end << 2);
        if paths[index] != None {
            get_seats(index, paths, &mut seats); 
        }
    }
    seats.len()
}

fn get_seats(index: usize, paths: &Vec<Option<Path>>, seats: &mut HashSet<usize>) {
    seats.insert(index >> 2);
    let mut index = index;
    while let Some(parent) = paths[index].as_ref().unwrap().parent {
        seats.insert(parent >> 2);
        let alternate_paths: &Vec<usize> = paths[index].as_ref().unwrap().alternate_paths.as_ref();
        for &alternate_path in alternate_paths {
            get_seats(alternate_path, paths, seats);
        }    
        index = parent;
    }
}

#[test]
fn default() {
    let input = get_input(24, 16).unwrap();
    let (paths, end) = get_paths(&input);
    assert_eq!(105496, part1(&paths, end));
    assert_eq!(524, part2(&paths, end));
}

// Input parsed (1ms)
// 1. 105496 (5μs)
// 2. 524 (45μs)
// Total: 1ms