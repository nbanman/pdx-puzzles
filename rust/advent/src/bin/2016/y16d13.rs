use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}};

type Input = (usize, usize);
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let fav: usize = input.parse().unwrap();
    let start = Pos::new2d(1, 1);
    let end = Pos::new2d(31, 39);
    let mut visited: FxHashMap<Pos, Option<usize>> = FxHashMap::default();
    visited.insert(start, Some(0));
    let mut steps = 0;
    let mut todo = Vec::new();
    let mut next = Vec::new();
    todo.push(start);
    
    loop {
        for pos in todo.drain( .. ) {
            for neighbor in pos.adjacent(false) {
                if visited.contains_key(&neighbor) { continue; }
                if neighbor == end {
                    visited.insert(neighbor, Some(steps));
                    let spaces = visited.values()
                        .filter(|steps| steps.map(|steps| steps <= 50).unwrap_or(false))
                        .count();
                    return (steps + 1, spaces);
                }
                let (x, y) = (neighbor.x(), neighbor.y());
                let hash = x * x + 3 * x + 2 * x * y + y + y * y + fav;
                if hash.count_ones() & 1 == 0 {
                    visited.insert(neighbor, Some(steps + 1));
                    next.push(neighbor);
                } else {
                    visited.insert(neighbor, None);
                }
            }
        }
        steps += 1;
        std::mem::swap(&mut todo, &mut next); 
    }
}

fn part1(input: &Input) -> Output {
    let (steps, _) = input;
    *steps
}

fn part2(input: &Input) -> Output {
    let (_, open) = input;
    *open
}

#[test]
fn default() {
    let input = get_input(16, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(92, part1(&input));
    assert_eq!(124, part2(&input));
}

// Input parsed (82μs)
// 1. 92 (8μs)
// 2. 124 (1μs)
// Total: 95μs