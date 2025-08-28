use std::collections::VecDeque;

use everybody_codes::utilities::inputs::get_event_input;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashMap;
use utilities::{
    math::formulae::{gcd, lcm},
    parsing::get_numbers::ContainsNumbers,
};

type Machine<'a> = Vec<Wheel<'a>>;

#[derive(Debug)]
struct Wheel<'a> {
    steps: usize,
    grins: Vec<&'a [u8]>,
}

fn main() {
    // let (input1, input2, input3) = get_inputs(24, 1);
    let input_string_1 = get_event_input(24, 16, 1);
    let input1 = parse_input(&input_string_1);
    let input_string_2 = get_event_input(24, 16, 2);
    let input2 = parse_input(&input_string_2);
    println!("1. {}", part1(&input1, 100));
    println!("2. {}", part2(&input2, 202420242024));
    // println!("3. {}", solve(&input3, 3));
}

fn parse_input(input: &str) -> Machine<'_> {
    let (pos_str, grin_str) = input.split_once("\n\n").unwrap();
    let positions: Vec<usize> = pos_str.get_numbers().collect();
    let mut grins = vec![Vec::new(); positions.len()];
    for line in grin_str.lines() {
        let line = line.as_bytes();
        for (strip, index) in (0..line.len()).step_by(4).enumerate() {
            let emoji = &line[index..index + 3];
            if emoji != &[32, 32, 32] {
                grins[strip].push(&line[index..index + 3])
            }
        }
    }
    positions
        .into_iter()
        .zip(grins.into_iter())
        .map(|(position, grins)| Wheel {
            steps: position,
            grins,
        })
        .collect()
}

fn part1(machine: &Machine, pulls: usize) -> String {
    let pull = machine
        .iter()
        .map(|wheel| wheel.grins[(pulls * wheel.steps) % wheel.grins.len()])
        .collect::<Vec<_>>();
    let mut joined = Vec::new();
    let mut iter = pull.iter().peekable();
    while let Some(&grin) = iter.next() {
        joined.extend_from_slice(grin);
        if iter.peek().is_some() {
            joined.push(b' ');
        }
    }
    String::from_utf8(joined).expect("Invalid UTF-8")
}

fn part2(machine: &Machine, pulls: usize) -> usize {
    let steps = |wheel: &Wheel| wheel.grins.len() / gcd(wheel.steps, wheel.grins.len());
    let lcm = machine
        .iter()
        .dropping(1)
        .fold(steps(&machine[0]), |acc, wheel| lcm(acc, steps(wheel)));
    let loops = pulls / lcm;
    let remainder = pulls % lcm;
    let remainder_coins = coins(machine, 0, remainder);
    let loops_coins = remainder_coins + coins(machine, remainder, lcm);
    println!(
        "pulls: {pulls}, lcm: {lcm}, loops: {loops}, remainder: {remainder}, remainder_coins: {remainder_coins}, loop_coins: {loops_coins}"
    );
    loops_coins * loops + remainder_coins
}

fn coins(machine: &Machine, start: usize, end: usize) -> usize {
    (start + 1..=end)
        .par_bridge()
        .map(|pull| {
            let mut distribution = [0u8; 128];
            for wheel in machine.iter() {
                let &[l, _, r] = wheel.grins[(pull * wheel.steps) % wheel.grins.len()] else {
                    panic!("grin not correct size!");
                };
                distribution[l as usize] += 1;
                distribution[r as usize] += 1;
            }
            distribution
                .into_iter()
                .map(|count| if count > 2 { (count - 2) as usize } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[derive(Debug)]
struct State {
    pull: [usize; 3],
    coins: usize,
}

fn part3(machine: &Machine, pulls: usize) -> usize {
    let mut cache = FxHashMap::default();
    let start = State {
        pull: [0; 3],
        coins: 0,
    };
    cache.insert(start.pull, start.coins);
    let mut q = VecDeque::new();
    q.push_back(start);

    let mut max_coins = 0;
    for _turn in 0..pulls {
        for _ in 0..q.len() {
            let State { pull, coins } = q.pop_front().unwrap();
            if coins > max_coins {
                max_coins = coins;
            }
            for offset in -1..=1 {
                let next_pull: [usize; 3] = machine
                    .iter()
                    .zip(pull.into_iter())
                    .map(|(wheel, pos)| {
                        (pos + (wheel.steps as isize + offset) as usize) % wheel.grins.len()
                    })
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap();
                let mut distribution = [0u8; 128];
                for (pos, wheel) in next_pull.into_iter().zip(machine.iter()) {
                    let &[l, _, r] = wheel.grins[pos] else {
                        panic!("grin not correct size!");
                    };
                    distribution[l as usize] += 1;
                    distribution[r as usize] += 1;
                }

                let next_coins = coins
                    + distribution
                        .into_iter()
                        .map(|count| if count > 2 { (count - 2) as usize } else { 0 })
                        .sum::<usize>();

                match cache.entry(next_pull) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        if *entry.get() < next_coins {
                            q.push_back(State {
                                pull: next_pull,
                                coins: next_coins,
                            });
                            entry.insert(next_coins);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        q.push_back(State {
                            pull: next_pull,
                            coins: next_coins,
                        });
                        entry.insert(next_coins);
                    }
                }
            }
        }
    }
    unreachable!()
}

// #[test]
// fn default() {
//     let (input1, input2, input3) = get_inputs(24, 1);
//     assert_eq!(1354, solve(&input1, 1));
//     assert_eq!(5639, solve(&input2, 2));
//     assert_eq!(28180, solve(&input3, 3));
// }

#[test]
fn examples() {
    let input_1_str = r"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>";
    let input1_2 = parse_input(&input_1_str);
    assert_eq!(">.- -.- ^,-".to_string(), part1(&input1_2, 100));
    assert_eq!(280014668134, part2(&input1_2, 202420242024));
    // assert_eq!(28180, solve(&input3, 3));
}
