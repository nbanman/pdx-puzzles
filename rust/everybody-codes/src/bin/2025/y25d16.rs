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

fn parse(input: Input) -> Vec<u64> {
    input.get_numbers().collect()
}

fn get_spell(elements: &mut Vec<u64>, cur: &mut Vec<u64>, wall: &[u64]) -> bool {
    if cur.as_slice() == wall {
        return true;
    }
    let n = *elements.last().unwrap();
    let len = wall.len() as u64;
    for n in n + 1..len {
        let mask: Vec<u64> = (0..len).map(|i| if (i + 1) % n == 0 { 1 } else { 0 }).collect();
        for (cur_col, brick) in cur.iter_mut().zip(&mask) {
            *cur_col += brick;
        }
        if cur.iter().zip(wall).all(|(cur_col, wall)| cur_col <= wall) {
            elements.push(n);
            if get_spell(elements, cur, wall) {
                return true;
            } else {
                elements.pop();
            }
        }
        for (cur_col, brick) in cur.iter_mut().zip(&mask) {
            *cur_col -= brick;
        }
    }
    false
}

fn sum_bricks(spell: &[u64], len: u64) -> u64 {
    spell.iter()
        .map(|&n| len / n)
        .sum()
}

fn part1(input: Input) -> u64 {
    sum_bricks(&parse(input), 90)
}

fn part2(input: Input) -> u64 {
    let wall = parse(input);
    let mut elements = vec![1];
    let mut cur = vec![1; wall.len()];
    get_spell(&mut elements, &mut cur, &wall);
    elements.into_iter().product()
}

fn part3(input: Input) -> u64 {
    let wall = parse(input);
    let mut elements = vec![1];
    let mut cur = vec![1; wall.len()];
    get_spell(&mut elements, &mut cur, &wall);
    let blocks: u64 = 202_520_252_025_000;

    // calculate the low end by using f64s to give fractional columns based on the highest value
    // instruction and then pretending that the LCM is that highest value instruction. That gets
    // you very close but a few more columns will be built out (but not completed) by the number 
    // of bricks, since some of those fractional columns will have to be built. 
    let highest = elements.last().unwrap().to_owned() as f64;
    let portion_sum = elements.iter()
        .map(|&i| highest / i as f64)
        .sum::<f64>();
    let low = (blocks as f64 / (portion_sum / highest)).ceil() as u64;

    // I already "know the answer" but a robust algorithm will not assume to know the high. So this
    // will increase by modest amounts until the high number is above the number of blocks reached.
    let gallop = 10;
    let mut high = low + gallop;
    let mut high_sum = sum_bricks(&elements, high);
    while high_sum <= blocks {
        high += gallop;
        high_sum = sum_bricks(&elements, high);
    }
    let potentials = (low..high).collect_vec();

    // use binary search to find the answer. Ok would be an exact match, which won't happen. Err
    // will give the index of the value that's just a bit higher than the exact match, but any block
    // beyond an exact match would create another column. So we subtract one from the index.
    match potentials.binary_search_by_key(&blocks, |&cols| sum_bricks(&elements, cols)) {
        Ok(idx) => potentials[idx],
        Err(idx) => potentials[idx - 1],
    }
}

#[test]
fn default() {
    // let (input1, input2, input3) = get_event_inputs(25, 16);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

// Input parsed (26μs)
// 1. 232 (8μs)
// 2. 148135882752 (27μs)
// 3. 97929823831789 (1.724ms)
// Total: 1.789ms