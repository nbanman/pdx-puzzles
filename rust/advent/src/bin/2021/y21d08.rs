use std::collections::{HashMap, HashSet};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Int = usize;
type Input = Vec<Lcd>;
type Output = Int;

struct Lcd {
    display: Vec<String>,
    output_value: usize,
}

impl Lcd {
    pub fn new(s: &str) -> Self {
        let (wires, display) = s.split_once(" | ").unwrap();

        let wire_groups= wires.split(' ')
            .map(|wire_group| wire_group.chars().sorted().collect::<String>())
            .into_group_map_by(|wire_group| wire_group.len());
        let mut digit_map = FxHashMap::default();
        
        // 1, 4, 7, 8 all have unique numbers of wires  
        digit_map.insert(1, wire_groups.get(&2).unwrap().first().unwrap().clone()); 
        digit_map.insert(4, wire_groups.get(&4).unwrap().first().unwrap().clone()); 
        digit_map.insert(7, wire_groups.get(&3).unwrap().first().unwrap().clone()); 
        digit_map.insert(8, wire_groups.get(&7).unwrap().first().unwrap().clone()); 
        // the remaining digits can be derived from comparisons with numbers that have already been found
        Self::insert_derived(&mut digit_map, &wire_groups, 6, 6, 1, 1);
        Self::insert_derived(&mut digit_map, &wire_groups, 9, 6, 4, 4);
        Self::insert_derived(&mut digit_map, &wire_groups, 5, 5, 6, 5);
        Self::insert_derived(&mut digit_map, &wire_groups, 2, 5, 5, 3);
        Self::insert_derived(&mut digit_map, &wire_groups, 3, 5, 5, 4);
        Self::insert_derived(&mut digit_map, &wire_groups, 0, 6, 5, 4);
        let digit_map: HashMap<_, _> = digit_map.into_iter()
            .map(|(k, v)| (v, k))
            .collect();
        let display: Vec<String> = display
            .split(' ')
            .map(|str| str.chars().sorted().collect::<String>())
            .collect();
        let output_value = display
            .iter()
            .map(|wire_group| digit_map.get(wire_group).unwrap())
            .join("")
            .parse()
            .unwrap();
        Self { display, output_value }
    }

    fn insert_derived(
        digit_map: &mut FxHashMap<Int, String>,
        wire_groups: &HashMap<Int, Vec<String>>,
        digit_no: Int, 
        group_no: Int,
        intersection: Int,
        intersect_size: Int,
    ) {
        let to_insert = wire_groups.get(&group_no)
            .unwrap()
            .iter()
            .find(|it| {
                let it: HashSet<_> = it.chars().collect();
                let prior: HashSet<_> = digit_map.get(&intersection).unwrap().chars().collect();
                it
                    .intersection(&prior)
                    .collect::<Vec<_>>()
                    .len() == intersect_size
            })
            .unwrap();
        digit_map.insert(digit_no, to_insert.clone());
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| Lcd::new(line)).collect()
}

fn part1(displays: &Input) -> Output {
    displays.iter()
        .flat_map(|lcd| &lcd.display)
        .filter(|display| !(5..=6).contains(&display.len()))
        .count()
}

fn part2(displays: &Input) -> Output {
    displays.iter().map(|it| it.output_value).sum()
}

#[test]
fn default() {
    let input = get_input(21, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(397, part1(&input));
    assert_eq!(1027422, part2(&input));
}

// Input parsed (1ms)
// 1. 397 (6μs)
// 2. 1027422 (3μs)
// Total: 1ms