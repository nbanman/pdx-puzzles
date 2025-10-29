use advent::utilities::get_input::get_input;
use fancy_regex::Regex;
use rustc_hash::FxHashMap;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = (Vec<Rule>, Vec<&'a str>);
type Output = usize;

#[derive(Debug, Clone)]
enum Rule {
    Value { name: usize, c: char },
    Seq { name: usize, sub_rules: Box<Vec<usize>> },
    Fork { name: usize, left: Box<Vec<usize>>, right: Box<Vec<usize>> },
}

impl Rule {
    fn name(&self) -> usize {
        match self {
            Rule::Value { name, .. } => *name,
            Rule::Seq { name, .. } => *name,
            Rule::Fork { name, .. } => *name,
        }
    }
    fn expand(&self, register: &FxHashMap<usize, &Rule>, looping: usize) -> String {
        let s = match self {
            Rule::Value { c, .. } => {
                c.to_string()
            },
            Rule::Seq { sub_rules, .. } => {
                sub_rules.iter()
                    .map(|name| register.get(name).unwrap().expand(register, looping))
                    .collect()
            },
            Rule::Fork { name, left, right } => {
                let left_str: String = left.iter()
                    .map(|i| register.get(i).unwrap().expand(register, looping))
                    .collect();

                if looping >= 5 {
                    left_str
                } else {
                    let mut expansion = String::new();
                    expansion.push('(');
                    expansion.push_str(&left_str);
                    expansion.push('|');
                    let looping = if *name == 8 || *name == 11 {
                        looping + 1
                    } else {
                        looping
                    };
                    let right_str: String = right.iter()
                        .map(|i| register.get(i).unwrap().expand(register, looping))
                        .collect();
                    expansion.push_str(&right_str);
                    expansion.push(')');
                    expansion
                }
            },
        };
        s
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let value = value.as_bytes();
        let ints: Vec<usize> = value.get_numbers().collect();
        match ints.len() {
            1 => Rule::Value { name: ints[0], c: value[value.len() - 2] as char },
            2 => Rule::Seq { name: ints[0], sub_rules: Box::new(vec![ints[1]]) },
            3 => {
                if value.contains(&b'|') {
                    Rule::Fork {
                        name: ints[0],
                        left: Box::new(vec![ints[1]]),
                        right: Box::new(vec![ints[2]]),
                    }
                } else {
                    Rule::Seq { name: ints[0], sub_rules: Box::new(vec![ints[1], ints[2]]) }
                }
            },
            _ => Rule::Fork {
                name: ints[0],
                left: Box::new(vec![ints[1], ints[2]]),
                right: Box::new(vec![ints[3], ints[4]]),
            }
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    let (rules, messages) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(Rule::from).collect();
    let messages = messages.lines().collect();
    (rules, messages)
}

fn solve(rules: &Vec<Rule>, messages: &Vec<&str>, part2: bool) -> Output {
    let eight =
        Rule::Fork { name: 8, left: Box::new(vec![42]), right: Box::new(vec![42, 8]) };
    let eleven = Rule::Fork {
        name: 11,
        left: Box::new(vec![42, 31]),
        right: Box::new(vec![42, 11, 31])
    };
    let register: FxHashMap<usize, &Rule> = rules.iter()
        .map(|rule| {
            if part2 {
                if rule.name() == 8 {
                    (rule.name(), &eight)
                } else if rule.name() == 11 {
                    (rule.name(), &eleven)
                } else {
                    (rule.name(), rule)
                }
            } else {
                (rule.name(), rule)
            }
        })
        .collect();
    let rx = register.get(&0).unwrap().expand(&register, 0);
    let rx = Regex::new(&format!("^{}$", &rx)).unwrap();
    messages.iter()
        .filter(|&&message| rx.is_match(message).unwrap())
        .count()
}

fn part1(input: &Input) -> Output {
    let (rules, messages) = input;
    solve(rules, messages, false)
}

fn part2(input: &Input) -> Output {
    let (rules, messages) = input;
    solve(rules, messages, true)
}

#[test]
fn default() {
    let input = get_input(20, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(151, part1(&input));
    assert_eq!(386, part2(&input));
}

// Input parsed (79μs)
// 1. 151 (4ms)
// 2. 386 (140ms)
// Total: 145ms
