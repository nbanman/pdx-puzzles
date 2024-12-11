use advent::utilities::get_input::get_input;
use utilities::{parsing::try_get::TryGet, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 10).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(topo_map: Input) -> Output { solve(topo_map, false) }
fn part2(topo_map: Input) -> Output { solve(topo_map, true) }

fn solve(topo_map: Input, distinct_paths: bool) -> Output {
    // working directly with string, so need width of each line to navigate in 2D properly
    let width = (topo_map.find('\n').unwrap() + 1) as isize;

    // finds all trailheads, then passes that on to the paths function, then sums the result
    topo_map.as_bytes().iter().enumerate()
        .filter(|&(_, height)| *height == b'0')
        .map(|(trailhead, _)| {
            paths(trailhead, topo_map.as_bytes(), width, distinct_paths)
        })
        .sum()
}

struct State { pos: usize, height: u8, }

// Standard DFS, with node tracking enabled or disabled depending on which part.
fn paths(trailhead: usize, topo_map: &[u8], width: isize, distinct_paths: bool) -> usize {
    let mut q = Vec::new();
    q.push(State { pos: trailhead, height: b'0' });
    let mut paths = 0;
    let mut visited = if distinct_paths {
        None
    } else {
        Some(vec![false; topo_map.len()])
    };
    while let Some(state) = q.pop() {
        if state.height == b'9' { paths += 1; }
        for neighbor in hike_path(state, topo_map, width) {
            if !distinct_paths {
                let visited = visited.as_mut()
                    .unwrap()
                    .get_mut(neighbor.pos)
                    .unwrap();
                if *visited { continue; } else { *visited = true; }
            }
            q.push(neighbor);
        }
    }
    paths
}

// finds adjacent spots along path, and returns the ones that are one height higher than current state
fn hike_path(
    state: State, 
    topo_map: &[u8], 
    width: isize
) -> impl Iterator<Item = State> + use<'_> {
    [-width, 1, width, -1].into_iter()
        .filter_map(move |offset| {
            let neighbor_pos = state.pos as isize + offset;
            let neighbor_height = topo_map.try_get(neighbor_pos)?;
            if *neighbor_height == state.height + 1 {
                Some(State { pos: neighbor_pos as usize, height: *neighbor_height })
            } else {
                None   
            }
        })
}

#[test]
fn default() {
    let input = get_input(24, 10).unwrap();
    assert_eq!(461, part1(&input));
    assert_eq!(875, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732", ];
    assert_eq!(36, part1(inputs[0]));
    // assert_eq!(Y, part2(&input));
}

// Input parsed (16μs)
// 1. 461 (76μs)
// 2. 875 (81μs)
// Total: 176μs

