use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::{enums::cardinals::Cardinal, graphs::{bfs, EdgeInfo, PathInfo}, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = PathInfo<Pos, usize>;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut map: FxHashMap<Pos, Vec<Pos>> = FxHashMap::default();
    let mut return_pos: Vec<Pos> = Vec::new();
    let mut pos = Pos::origin();
    for &b in input.as_bytes().iter() {
        let next = match b {
            b'N' => { pos.move_direction(Cardinal::North, 1).unwrap() },
            b'W' => { pos.move_direction(Cardinal::West, 1).unwrap() },
            b'E' => { pos.move_direction(Cardinal::East, 1).unwrap() },
            b'S' => { pos.move_direction(Cardinal::South, 1).unwrap() },
            b'(' => {
                return_pos.push(pos);
                continue;
            },
            b'|' => {
                pos = *return_pos.last().unwrap();
                continue;
            },
            b')' => {
                return_pos.pop();
                continue;
            }
            _ => { continue; }
        };
        let to = map.entry(pos).or_insert(Vec::new());
        if !to.contains(&next) {
            to.push(next);
        }

        let from = map.entry(next).or_insert(Vec::new());
        if !from.contains(&pos) {
            from.push(pos);
        }
        pos = next
    }

    bfs(
        Pos::origin(),
        |_: EdgeInfo<usize>, pos| map.get(pos).unwrap().clone(),
        |_, _| false,
    )
}

fn part1(path: &Input) -> Output {
    path.nodes.get_entry(path.nodes.len() - 1).unwrap().1.cost
}

fn part2(path: &Input) -> Output {
    path.nodes.values.iter()
        .filter(|&(_, edge)| edge.cost >= 1000)
        .count()
}

#[test]
fn default() {
    let input = get_input(18, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(3930, part1(&input));
    assert_eq!(8240, part2(&input));
}

// Input parsed (3ms)
// 1. 3930 (8μs)
// 2. 8240 (13μs)
// Total: 3ms
