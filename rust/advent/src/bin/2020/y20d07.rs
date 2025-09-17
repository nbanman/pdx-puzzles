use std::collections::{HashMap, HashSet, VecDeque};
use lazy_regex::regex;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (Vec<Rule<'a>>, BagMap<'a>);
type Output = usize;
type BagMap<'a> = HashMap<&'a str, Rule<'a>>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    color: &'a str,
    held_bags: Vec<HeldBag<'a>>,
}

impl<'a> Rule<'a> {
    fn bags_inside(&self, bag_map: &BagMap) -> Output {
        let held_bag = HeldBag { color: self.color, count: 1 };
        held_bag.bags_inside(bag_map) - 1
    }

    fn contains(&self, other: &str, bag_map: &BagMap) -> bool {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut q: VecDeque<&HeldBag> = self.held_bags.iter().collect();
        while let Some(bag) = q.pop_front() {
            let current = bag.color;
            if current == other { return true; }
            visited.insert(current);
            let next = bag_map[current].held_bags.iter()
                .filter(|hb| !visited.contains(&hb.color));
            q.extend(next);
        }
        false
    }
}

#[derive(Debug, Clone)]
struct HeldBag<'a> {
    color: &'a str,
    count: Output,
}

impl<'a> HeldBag<'a> {
    fn bags_inside(&self, bag_map: &BagMap) -> Output {
        let inside_bags: Output = bag_map[&self.color].held_bags.iter()
            .map(|bag| bag.bags_inside(bag_map))
            .sum();
        self.count + self.count * inside_bags
    }
}

fn parse_input(input: &'_ str) -> Input<'_> {
    let rule_rx = regex!(r"(\w+ \w+) bags contain ([^.]+)\.");
    let bag_rx = regex!(r"(\d+) (\w+ \w+) bag");
    let rules: Vec<Rule> = rule_rx.captures_iter(input)
        .map(|caps| {
            let container = caps.get(1).unwrap().as_str();
            let contained = caps.get(2).unwrap().as_str();
            let held_bags = bag_rx.captures_iter(contained)
                .map(|held_caps| {
                    let count = held_caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let color = held_caps.get(2).unwrap().as_str();
                    HeldBag { color, count }
                })
                .collect();
            Rule { color: container, held_bags }
        })
        .collect();
    let mut bag_map = BagMap::new();
    for rule in rules.iter() {
        bag_map.insert(rule.color, rule.clone());
    }
    (rules, bag_map)
}

fn part1((rules, bag_map): &Input) -> Output {
    rules.iter()
        .filter(|rule| rule.contains("shiny gold", bag_map))
        .count()
}

fn part2((_, bag_map): &Input) -> Output {
    bag_map["shiny gold"].bags_inside(bag_map)
}

#[test]
fn default() {
    let input = get_input(20, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(252, part1(&input));
    assert_eq!(35487, part2(&input));
}

// Input parsed (3ms)
// 1. 252 (2ms)
// 2. 35487 (5μs)
// Total: 6ms