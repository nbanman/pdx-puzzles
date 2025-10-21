use advent::utilities::{get_input::get_input, intcode::IntCode};
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = Vec<Nic>;
type Output = i64;
type Nic = IntCode;
type Nat = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let code: Vec<i64> = input.get_numbers().collect();
    let mut nics = vec![Nic::new(&code); 50];
    for (i, nic) in nics.iter_mut().enumerate() {
        nic.input(i as i64);
        nic.run_while_able();
    }
    nics
}

fn add_to_nat(nics: &mut Input, nat: &mut Vec<Nat>, end_on_first_add: bool) {
    for i in 0..nics.len() {
        let nic = &mut nics[i];
        if nic.input.is_empty() {
            nic.input(-1);
        }
        let (_, output) = nic.run_while_able();
        for (recipient, x, y) in output.into_iter().tuples() {
            if recipient == 255 {
                nat.push(Nat::new2d(x, y));
                if end_on_first_add { break; }
            } else {
                let recipient_nic = &mut nics[recipient as usize];
                recipient_nic.input(x);
                recipient_nic.input(y);
            }
        }
    }
}

fn part1(mut nics: Input) -> Output {
    let mut nat: Vec<Nat> = Vec::new();
    loop {
        add_to_nat(&mut nics, &mut nat, true);
        if !nat.is_empty() {
            return nat[0].y();
        }
    }
}

fn part2(mut nics: Input) -> Output {
    let mut nat: Vec<Nat> = Vec::new();
    let mut last: i64 = -1;
    loop {
        add_to_nat(&mut nics, &mut nat, false);
        if nics.iter().all(|nic| nic.input.is_empty()) {
            let y = nat.last().unwrap().y();
            if y == last { return y; }
            nics[0].input(nat.last().unwrap().x());
            nics[0].input(y);
            last = y;
        }
    }
}

#[test]
fn default() {
    let input = get_input(19, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(23701, part1(input.clone()));
    assert_eq!(17225, part2(input));
}

// Input parsed (721μs)
// 1. 23701 (832μs)
// 2. 17225 (983μs)
// Total: 2ms