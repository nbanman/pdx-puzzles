use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (Sue<'a>, Vec<Sue<'a>>);
type Output = usize;
type Sue<'a> = FxHashMap<&'a str, usize>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn build_sue(s: &str) -> Sue<'_> {
    let rx = regex!(r"([a-z]+): (\d+)");
    rx.captures_iter(s)
        .map(|cap| {
            let item = cap.get(1).unwrap().as_str();
            let amt = cap.get(2).unwrap().as_str().parse().unwrap();
            (item, amt)
        })
        .collect()
}

fn parse_input(input: &str) -> Input<'_> {
    let ticker_tape = r"
        children: 3
        cats: 7
        samoyeds: 2
        pomeranians: 3
        akitas: 0
        vizslas: 0
        goldfish: 5
        trees: 3
        cars: 2
        perfumes: 1
    ";
    let aunt_sue = build_sue(ticker_tape);
    let sues = input.lines().map(build_sue).collect();
    (aunt_sue, sues)
}

fn modern_retroencabulator(sue: &Sue, aunt_sue: &Sue) -> bool {
    aunt_sue.iter().all(|(&item, amt)| {
        sue.get(item).map(|s_amt| amt == s_amt).unwrap_or(true)
    })
}

fn outdated_retroencabulator(sue: &Sue, aunt_sue: &Sue) -> bool {
    aunt_sue.iter().all(|(&item, amt)| {
        sue.get(item)
            .map(|s_amt| {
                match item {
                    "cats" | "trees" => s_amt > amt,
                    "pomeranians" | "goldfish" => s_amt < amt ,
                    _ => s_amt == amt,
                }
            })
            .unwrap_or(true)
    })
}

fn part1((aunt_sue, sues): &Input) -> Output {
    sues.iter()
        .position(|sue| modern_retroencabulator(sue, aunt_sue))
        .unwrap() + 1
}

fn part2((aunt_sue, sues): &Input) -> Output {
    sues.iter()
        .position(|sue| outdated_retroencabulator(sue, aunt_sue))
        .unwrap() + 1
}

#[test]
fn default() {
    let input = get_input(15, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(40, part1(&input));
    assert_eq!(241, part2(&input));
}

// Input parsed (657μs)
// 1. 40 (8μs)
// 2. 241 (15μs)
// Total: 682μs
