use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utilities::structs::{stopwatch::{ReportDuration, Stopwatch}, str_grid::{AdjacentMetadata, StrGrid}};

type Input<'a> = &'a str;
type Clearing<'a> = StrGrid<'a>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 12);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input<'_>) -> (Clearing<'_>, Vec<bool>) {
    let clearing: StrGrid<'_> = input.into();
    let visited = vec![false; clearing.s.len()];
    (clearing, visited)
}

fn solve(clearing: &StrGrid, visited: &mut Vec<bool>, mut todo: Vec<usize>) -> usize {
    let mut next = Vec::new();
    for &pos in todo.iter() {
        visited[pos] = true;
    }

    while !todo.is_empty() {
        for pos in todo.drain( .. ) {
            let barrel = clearing.s[pos];
            for AdjacentMetadata { pos: adj_pos, dir: _, b: adj_barrel } in clearing.adjacent(pos) {
                if !visited[adj_pos] && barrel >= adj_barrel {
                    visited[adj_pos] = true;
                    next.push(adj_pos);
                }
            }
        }
        std::mem::swap(&mut todo, &mut next);
    }
    visited.iter().filter(|&&it| it).count()
}

fn part1(input: Input) -> usize {
    let (clearing, mut visited) = parse(input);
    let start = vec![0usize];
    solve(&clearing, &mut visited, start)
}

fn part2(input: Input) -> usize {
    let (clearing, mut visited) = parse(input);
    let start = vec![0, input.len() - 1];
    solve(&clearing, &mut visited, start)
}

fn part3(input: Input) -> usize {
    let (clearing, mut visited) = parse(input);
    let mut history = visited.clone();
    let mut winners = Vec::new();

    let peaks = clearing.s.iter().enumerate()
        .filter(|&(idx, &c)| {
            clearing.adjacent(idx).all(|adj| c > adj.b)
        })
        .map(|(idx, _)| idx)
        .collect_vec();

    for _ in 0..3 {
        let (_, exploded, winner) = peaks
            .par_iter()
            .filter(|&pos| !winners.contains(pos))
            .map(|&pos| {
                let mut exploded = history.clone();
                let barrels = solve(&clearing, &mut exploded, vec![pos]);
                (barrels, exploded, pos)
            })
            .max_by_key(|(barrels, _, _)| *barrels)
            .unwrap();
        winners.push(winner);
        history = exploded;
    }
    solve(&clearing, &mut visited, winners)
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 12);
    assert_eq!(240, part1(&input1));
    assert_eq!(5731, part2(&input2));
    assert_eq!(4135, part3(&input3));
}

// Input parsed (41μs)
// 1. 240 (30μs)
// 2. 5731 (92μs)
// 3. 4135 (8.774ms)
// Total: 8.943ms