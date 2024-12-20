use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (Vec<&'a str>, Vec<&'a str>);
type Output = usize;
type Cache = std::collections::HashMap<String, usize, rustc_hash::FxBuildHasher>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 19).unwrap();
    let input = parse_input(&input);
    let mut cache = Cache::default();
    cache.insert(String::new(), 1usize);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input, &mut cache), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input, &mut cache), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (towels, designs) = input.trim().split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.split("\n").collect();
    let mut cache: FxHashMap<String, usize> = FxHashMap::default();
    cache.insert(String::new(), 1);
    (towels, designs)
}

fn variations(design: &str, towels: &[&str], cache: &mut Cache) -> Output {
    if design.is_empty() { return 1; }
    if cache.contains_key(design) { return cache[design]; }
    let vars = towels.iter() 
        .map(|&towel| {
            // println!("towel: {towel}, design: {design}, starts with: {}", design.starts_with(towel));
            if design.starts_with(towel) {
                variations(&design[towel.len()..], towels, cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(design.to_string(), vars);
    vars
}

fn part1(input: &Input, cache: &mut Cache) -> Output {
    let (towels, designs) = input;
    designs.iter()
        .filter(|&&design| variations(design, towels, cache) > 0)
        .count()
}

fn part2(input: &Input, cache: &mut Cache) -> Output {
    let (towels, designs) = input;
    designs.iter()
        .map(|&design| variations(design, towels, cache))
        .sum()
}

#[test]
fn default() {
    let input = get_input(24, 19).unwrap();
    let input = parse_input(&input);
    let mut cache = Cache::default();
    assert_eq!(238, part1(&input, &mut cache));
    assert_eq!(635018909726691, part2(&input, &mut cache));
}

#[test]
fn example() {
    let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    let input = parse_input(input);
    let mut cache = Cache::default();
    assert_eq!(6, part1(&input, &mut cache));
    assert_eq!(16, part2(&input, &mut cache));
}

// Input parsed (47μs)
// 1. 238 (16ms)
// 2. 635018909726691 (13μs)
// Total: 16ms
