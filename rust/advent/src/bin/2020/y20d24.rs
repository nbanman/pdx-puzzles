use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{enums::intercardinals::Intercardinal, structs::{hexagon::Hexagon, stopwatch::{ReportDuration, Stopwatch}}};

type Input = FxHashSet<Hexagon>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let rx = regex!("(nw|ne|sw|se|w|e)");
    let rules = input.lines()
        .map(|line| {
            rx.find_iter(line).map(|dir| match dir.as_str() {
                "w" => Intercardinal::North,
                "nw" => Intercardinal::Northeast,
                "ne" => Intercardinal::Southeast,
                "e" => Intercardinal::South,
                "se" => Intercardinal::Southwest,
                "sw" => Intercardinal::Northwest,
                _ => unreachable!()
            })
        });
    let mut flipped_tiles = FxHashSet::default();

    for rule in rules {
        let tile = rule.fold(Hexagon::origin(), |acc, dir| {
            acc.hex_at(dir)
        });

        if flipped_tiles.contains(&tile) {
            flipped_tiles.remove(&tile);
        } else {
            flipped_tiles.insert(tile);
        }
    }
    flipped_tiles
}

fn part1(flipped_tiles: Input) -> Output {
    flipped_tiles.len()
}

fn part2(mut flipped_tiles: Input) -> Output {
    let mut candidates: FxHashMap<Hexagon, usize> = FxHashMap::default();
    for _day in 0..100 {
        for tile in flipped_tiles.iter() {
            for adj in tile.adjacent() {
                candidates.entry(adj)
                    .and_modify(|n| { *n += 1; })
                    .or_insert(1);
            }
        }
        flipped_tiles = candidates.drain()
            .filter(|(candidate, occurrences)| {
                *occurrences == 2 || (*occurrences == 1 && flipped_tiles.contains(candidate))
            })
            .map(|(confirmed, _)| confirmed)
            .collect();
    }
    flipped_tiles.len()
}

#[test]
fn default() {
    let input = get_input(20, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(244, part1(input.clone()));
    assert_eq!(3665, part2(input));
}

// Input parsed (598μs)
// 1. 244 (7μs)
// 2. 3665 (11ms)
// Total: 12ms
