use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 16);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_spell(notes: Input) -> Vec<u64> {
    let mut wall: Vec<u64> = notes.get_numbers().collect();
    let mut spell = Vec::new();
    for n in 1..=wall.len() {
        if wall[n - 1] > 0 {
            spell.push(n as u64);
            for k in (n - 1..wall.len()).step_by(n) {
                wall[k] -= 1;
            }
        }
    }
    spell
}

fn sum_bricks(spell: &[u64], len: u64) -> u64 {
    spell.iter()
        .map(|&n| len / n)
        .sum()
}

fn part1(notes: Input) -> u64 {
    sum_bricks(&notes.get_numbers().collect_vec(), 90)
}

fn part2(notes: Input) -> u64 {
    get_spell(notes).into_iter().product()
}

fn part3(notes: Input) -> u64 {
    let spell = get_spell(notes);
    let blocks: u64 = 202_520_252_025_000;

    // calculate the low end by using f64s to give fractional columns based on the highest value
    // instruction and then pretending that the LCM is that highest value instruction. That gets
    // you very close but a few more columns will be built out (but not completed) by the number 
    // of bricks, since some of those fractional columns will have to be built. 
    let highest = spell.last().unwrap().to_owned() as f64;
    let portion_sum = spell.iter()
        .map(|&i| highest / i as f64)
        .sum::<f64>();
    let low = (blocks as f64 / (portion_sum / highest)).ceil() as u64;

    // I already "know the answer" but a robust algorithm will not assume to know the high. So this
    // will increase by modest amounts until the high number is above the number of blocks reached.
    let gallop = 10;
    let mut high = low + gallop;
    let mut high_sum = sum_bricks(&spell, high);
    while high_sum <= blocks {
        high += gallop;
        high_sum = sum_bricks(&spell, high);
    }
    let potentials = (low..high).collect_vec();

    // use binary search to find the answer. Ok would be an exact match, which won't happen. Err
    // will give the index of the value that's just a bit higher than the exact match, but any block
    // beyond an exact match would create another column. So we subtract one from the index.
    match potentials.binary_search_by_key(&blocks, |&cols| sum_bricks(&spell, cols)) {
        Ok(idx) => potentials[idx],
        Err(idx) => potentials[idx - 1],
    }
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 16);
    assert_eq!(232, part1(&input1));
    assert_eq!(148135882752, part2(&input2));
    assert_eq!(97929823831789, part3(&input3));
}

// Input parsed (33μs)
// 1. 232 (9μs)
// 2. 148135882752 (8μs)
// 3. 97929823831789 (17μs)
// Total: 71μs
