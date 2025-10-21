use std::cmp::Ordering;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = BridgeParts;
type Output = u64;
type Bridge = (usize, u64);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

struct BridgeParts {
    components: Vec<Component>,
    port_map: FxHashMap<u64, Vec<usize>>,
}

#[derive(Debug, Copy, Clone)]
struct Component {
    a: u64,
    b: u64,
}

impl Component {
    fn strength(&self) -> u64 {
        self.a + self.b
    }

    fn other_end(&self, n: u64) -> u64 {
        if self.a == n { self.b } else { self.a }
    }
}

fn parse_input(input: &str) -> Input {
    let components: Vec<Component> = input
        .get_numbers()
        .tuples()
        .map(|(a, b)| Component { a, b })
        .collect();

    let mut port_map = FxHashMap::default();

    for (idx, component) in components.iter().enumerate() {
        port_map.entry(component.a).or_insert(Vec::new()).push(idx);
        port_map.entry(component.b).or_insert(Vec::new()).push(idx);
    }

    BridgeParts {
        components,
        port_map,
    }
}

struct State {
    n: u64,
    strength: u64,
    visited: u64,
}

impl State {
    fn len(&self) -> usize {
        self.visited.count_ones() as usize
    }
}

fn build_bridge(
    bridge_parts: &BridgeParts,
    comparator: fn(Bridge, Bridge) -> std::cmp::Ordering,
) -> Output {
    let mut q = Vec::with_capacity(35);
    q.push(State { n: 0, strength: 0, visited: 0 });
    let mut max_cmp = (0usize, 0u64);
    while let Some(cur) = q.pop() {
        let mut candidate_count = 0;
        let candidates = bridge_parts
            .port_map
            .get(&cur.n)
            .unwrap()
            .iter()
            .filter(|&&index| (cur.visited >> index) & 1 == 0);

        for &index in candidates {
            candidate_count += 1;
            let component = bridge_parts.components[index];
            let strength = cur.strength + component.strength();
            let n = component.other_end(cur.n);
            let visited = cur.visited + (1 << index);
            q.push(State {
                n,
                strength,
                visited,
            });
        }

        if candidate_count == 0 {
            let cmp = (cur.len(), cur.strength);
            if comparator(cmp, max_cmp) == Ordering::Greater {
                max_cmp = cmp;
            }
        }
    }
    max_cmp.1
}

fn part1(bridge_parts: &Input) -> Output {
    build_bridge(bridge_parts, |(_, a_strength), (_, b_strength)| {
        a_strength.cmp(&b_strength)
    })
}

fn part2(bridge_parts: &Input) -> Output {
    build_bridge(bridge_parts, |(a_len, a_strength), (b_len, b_strength)| {
        a_len.cmp(&b_len).then(a_strength.cmp(&b_strength))
    })
}

#[test]
fn default() {
    let input = get_input(17, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(1868, part1(&input));
    assert_eq!(1841, part2(&input));
}

#[test]
fn example() {
    let input = r"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";
    let input = parse_input(input);
    assert_eq!(31, part1(&input));
    assert_eq!(19, part2(&input));
}

// Input parsed (24μs)
// 1. 1868 (29ms)
// 2. 1841 (27ms)
// Total: 56ms
