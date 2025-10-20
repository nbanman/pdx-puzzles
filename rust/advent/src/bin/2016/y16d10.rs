use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use rustc_hash::FxHashMap;
use utilities::{minmax::minmax, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (usize, FxHashMap<Recipient, usize>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Recipient {
    Bot(usize),
    Bin(usize),
}

impl Recipient {
    fn inner(&self) -> usize {
        match self {
            Recipient::Bot(inner) => *inner,
            Recipient::Bin(inner) => *inner,
        }
    }
}

fn parse_input(input: &str) -> Input {
    let bot_rx = regex!(r"bot (?<botId>\d+) gives low to (?<lowType>bot|output) (?<lowId>\d+) and high to (?<highType>bot|output) (?<highId>\d+)");
    let bots: FxHashMap<usize, (Recipient, Recipient)> = bot_rx.captures_iter(input)
        .map(|caps| {
            let bot_id: usize = caps.name("botId").unwrap().as_str().parse().unwrap();
            let low_id: usize = caps.name("lowId").unwrap().as_str().parse().unwrap();
            let high_id: usize = caps.name("highId").unwrap().as_str().parse().unwrap();
            let low_type = caps.name("lowType").unwrap().as_str();
            let low = match low_type {
                "bot" => Recipient::Bot(low_id),
                "output" => Recipient::Bin(low_id),
                _ => { unreachable!(); },
            };
            let high_type = caps.name("highType").unwrap().as_str();
            let high = match high_type {
                "bot" => Recipient::Bot(high_id),
                "output" => Recipient::Bin(high_id),
                _ => { unreachable!(); },
            };
            (bot_id, (low, high))
        })
        .collect();

    let mut responsible: Option<usize> = None;

    let reg_rx = regex!(r"value (?<value>\d+) goes to bot (?<botId>\d+)");
    let mut registry = FxHashMap::default();
    for caps in reg_rx.captures_iter(input) {
        let bot_id: usize = caps.name("botId").unwrap().as_str().parse().unwrap();
        let value: usize = caps.name("value").unwrap().as_str().parse().unwrap();
        assign(Recipient::Bot(bot_id), value, &mut registry, &mut responsible, &bots);
    }
    (responsible.unwrap(), registry)
}

fn assign(
    recipient: Recipient,
    value: usize,
    registry: &mut FxHashMap<Recipient, usize>,
    responsible: &mut Option<usize>,
    bots: &FxHashMap<usize, (Recipient, Recipient)>
) {
    if let Some(&current) = registry.get(&recipient) {
        let (&low, &high) = minmax(&current, &value);
        if low == 17 && high == 61 {
            *responsible = Some(recipient.inner());
        }
        let (low_recipient, high_recipient) = bots.get(&recipient.inner()).unwrap();
        assign(*low_recipient, low, registry, responsible, bots);
        assign(*high_recipient, high, registry, responsible, bots);
        registry.remove(&recipient);
    } else {
        registry.insert(recipient, value);
    }
}

fn part1(input: &Input) -> Output {
    let (responsible, _) = input;
    *responsible
}

fn part2(input: &Input) -> Output {
    let (_, registry) = input;
    registry.into_iter()
        .filter(|(id, _)| id.inner() <= 2)
        .fold(1, |acc, (_, &value)| acc * value)
}

#[test]
fn default() {
    let input = get_input(16, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!(101, part1(&input));
    assert_eq!(37789, part2(&input));
}

// Input parsed (839μs)
// 1. 101 (6μs)
// 2. 37789 (2μs)
// Total: 851μs
