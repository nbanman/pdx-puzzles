use std::vec;
use rayon::iter::ParallelIterator;
use rayon::iter::ParallelBridge;
use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use utilities::structs::str_grid::StrGrid;

type Input<'a> = &'a str;
fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 18);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", parts1_and_2(&input1), stopwatch.lap().report());
    println!("2. {} ({})", parts1_and_2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parts1_and_2(input: Input) -> usize {
    let farm = get_farm(input);
    let start: Vec<usize> = farm.s.iter().enumerate()
        .filter_map(|(pos, &terrain)| {
            let x = pos % farm.width;
            let y = pos / farm.width;
            if y == 0 || y == farm.height - 1 || x == 0 || x == farm.width - 2 {
                if terrain == b'.' {
                    Some(pos)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    bfs(&farm, &start).0
}

fn part3(input: Input) -> usize {
    let farm = get_farm(input);

    let optimal = farm.s.iter().enumerate()
        .filter(|(_, t)| **t == b'P')
        .par_bridge()
        .map(|(pos, _)| rev_bfs(&farm, pos))
        .collect::<Vec<_>>()
        .into_iter()
        .fold(vec![0usize; farm.s.len()], |acc, dist_map| {
            acc.iter().zip(dist_map.iter()).map(|(a, b)| a + b).collect()
        })
        .iter()
        .enumerate()
        .filter(|(pos, _)| farm.s[*pos] == b'.')
        .min_by_key(|(_, n)| *n)
        .unwrap()
        .0;

    bfs(&farm, &vec![optimal]).1.iter().sum()
}

fn rev_bfs(farm: &StrGrid, end: usize) -> Vec<usize> {
    let mut turn_map = vec![0usize; farm.s.len()];
    turn_map[end] += 1;
    let mut turns = 0;
    let locations = farm.s.iter().filter(|&t| *t == b'P' || *t == b'.').count();
    let mut locations_found = 1;
    let mut todo: Vec<usize> =
        Vec::with_capacity(farm.s.len() - farm.s.iter().filter(|&it| *it != b'#').count(), );
    let mut next = todo.clone();
    todo.push(end);

    while !todo.is_empty() {
        if locations_found == locations { break; }
        for pos in todo.drain(..) {
            let neighbors: Vec<_> = farm
                .adjacent(pos)
                .filter(|n| turn_map[n.pos] == 0)
                .collect();

            for neighbor in neighbors {
                if neighbor.b == b'P' || neighbor.b == b'.' {
                    locations_found += 1;
                    turn_map[neighbor.pos] = turns + 1;
                    next.push(neighbor.pos);
                }
            }
        }
        turns += 1;
        (todo, next) = (next, todo);
    }

    turn_map[end] = 0;
    turn_map
}

fn bfs(farm: &StrGrid, start: &[usize]) -> (usize, Vec<usize>) {
    let palms = farm.s.iter().filter(|&t| *t == b'P').count();
    let mut turns = 0;
    let mut palms_found = Vec::with_capacity(palms);
    let mut todo: Vec<usize> =
        Vec::with_capacity(farm.s.len() - farm.s.iter().filter(|&it| *it != b'#').count(), );
    let mut next = todo.clone();
    todo.extend(start);
    let mut seen = vec![false; farm.s.len()];
    for pos in start {
        seen[*pos] = true;
    }

    while !todo.is_empty() {
        if palms_found.len() == palms { break; }
        for pos in todo.drain(..) {
            let neighbors: Vec<_> = farm
                .adjacent(pos)
                .filter(|n| !seen[n.pos])
                .collect();

            for neighbor in neighbors {
                match neighbor.b {
                    b'P' => palms_found.push(turns + 1),
                    b'#' => { continue; },
                    _ => {},
                }
                seen[neighbor.pos] = true;
                next.push(neighbor.pos);
            }
        }
        turns += 1;
        (todo, next) = (next, todo);
    }
    (turns, palms_found)
}

fn get_farm(input: Input) -> StrGrid {
    StrGrid::new(input).unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(24, 18);
    assert_eq!(103, parts1_and_2(&input1));
    assert_eq!(1507, parts1_and_2(&input2));
    assert_eq!(244810, part3(&input3));
}

#[test]
fn examples() {
    let inputs = [r"##########
..#......#
#.P.####P#
#.#...P#.#
##########", r"#######################
...P..P...#P....#.....#
#.#######.#.#.#.#####.#
#.....#...#P#.#..P....#
#.#####.#####.#########
#...P....P.P.P.....P#.#
#.#######.#####.#.#.#.#
#...#.....#P...P#.#....
#######################", r"##########
#.#......#
#.P.####P#
#.#...P#.#
##########", r"##########
#.#......#
#.P..###P#
#.#...P#.#
##########"];
    assert_eq!(11, parts1_and_2(inputs[0]));
    assert_eq!(21, parts1_and_2(inputs[1]));
    assert_eq!(12, part3(inputs[2]));
}

// Input parsed (45μs)
// 1. 103 (30μs)
// 2. 1507 (274μs)
// 3. 244810 (38ms)
// Total: 38ms