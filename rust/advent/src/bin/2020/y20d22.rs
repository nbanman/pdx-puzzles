use std::{cmp::min, collections::VecDeque};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{minmax::minmax, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (VecDeque<usize>, VecDeque<usize>);
type Output = usize;

#[derive(Debug, Clone, Copy)]
enum Player { One, Two, }

#[derive(Debug, Clone, Copy)]
struct Game {
    winner: Player,
    score: usize,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.split("\n\n")
        .map(|stanza| stanza.lines().filter_map(|line| line.parse().ok()).collect())
        .collect_tuple()
        .unwrap()
}

fn score(player: &VecDeque<usize>) -> usize {
    player.iter().enumerate()
        .fold(0, |acc, (index, &i)| {
            acc + (player.len() - index) * i
        })
}

fn update_player_part1(player: &mut VecDeque<usize>, short: usize, player_wins: Vec<[usize; 2]>) {
    for _ in 0..short {
        player.pop_front();
    }
    for [a, b] in player_wins {
        let (&min, &max) = minmax(&a, &b);
        player.push_back(max);
        player.push_back(min);
    }
}

fn play(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> Game {
    let mut cache: FxHashSet<(VecDeque<usize>, VecDeque<usize>)> = FxHashSet::default();
    while cache.insert((p1.clone(), p2.clone())) {
        let p1_peek = p1[0];
        let p2_peek = p2[0];
        if p1.len() - 1 < p1_peek || p2.len() - 1 < p2_peek {
            if p1_peek > p2_peek {
                award(&mut p1, &mut p2, [p1_peek, p2_peek]);
            } else {
                award(&mut p2, &mut p1, [p2_peek, p1_peek]);
            }
        } else {
            let p1_mini: VecDeque<usize> = p1.iter().skip(1).take(p1_peek).copied().collect();
            let p2_mini: VecDeque<usize> = p2.iter().skip(1).take(p2_peek).copied().collect();
            match play(p1_mini, p2_mini) {
                Game { winner: Player::One, .. } => {
                    award(&mut p1, &mut p2, [p1_peek, p2_peek]);
                },
                Game { winner: Player::Two, .. } => {
                    award(&mut p2, &mut p1, [p2_peek, p1_peek]);
                }
            }
        }
        
        if p1.is_empty() {
            return Game {
                winner: Player::Two,
                score: score(&p2),
            }
        } else if p2.is_empty() {
            return Game {
                winner: Player::One,
                score: score(&p1),
            }
        }
    }
    Game {
        winner: Player::One,
        score: score(&p1),
    }
}

fn award(winner: &mut VecDeque<usize>, loser: &mut VecDeque<usize>, in_play: [usize; 2]) {
    winner.pop_front();
    winner.extend(in_play);
    loser.pop_front();
}

fn part1((mut p1, mut p2): Input) -> Output {
    while !p1.is_empty() && !p2.is_empty() {
        let short = min(p1.len(), p2.len());
        let (p1wins, p2wins) = p1.iter().copied().zip(p2.iter().copied())
            .map(|(a, b)| [a, b])
            .partition::<Vec<[usize; 2]>, _>(|&[a, b]| a > b);
        update_player_part1(&mut p1, short, p1wins);
        update_player_part1(&mut p2, short, p2wins);
    }
    score(&p1) + score(&p2)
}

fn part2((p1, p2): Input) -> Output {
    play (p1, p2).score
}

#[test]
fn default() {
    let input = get_input(20, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(32824, part1(input.clone()));
    assert_eq!(36515, part2(input));
}

// Input parsed (21μs)
// 1. 32824 (14μs)
// 2. 36515 (217ms)
// Total: 217ms
