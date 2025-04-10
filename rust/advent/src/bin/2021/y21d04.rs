use advent::utilities::get_input::get_input;
use rustc_hash::FxHashSet;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{grid::{Grid2, GridError}, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (Vec<Int>, Vec<BingoCard>);
type Int = usize;
type Output = Int;

struct BingoCard {
    card: Grid2<Int>,
    win_conditions: Vec<FxHashSet<Int>>,
}

impl BingoCard {
    fn new(card_str: &str) -> Result<Self, GridError> {
        let numbers: Vec<Int> = card_str.get_numbers().collect();
        let card = Grid2::new2d(numbers, 5)?;
        let win_conditions = card.rows()
            .chain(card.columns())
            .map(|it| it.into_iter().cloned().collect())
            .collect();
        Ok(BingoCard { card, win_conditions })
    }
    
    fn bingo(&self, called_numbers: &FxHashSet<Int>) -> bool {
        self.win_conditions.iter().any(|it| {
            it.intersection(called_numbers).count() == it.len()
        })
    }
    
    fn score(&self, called_numbers: &FxHashSet<Int>) -> Int {
        self.card.iter().filter(|it| !called_numbers.contains(it)).sum()
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (draw_pile, bingo_cards) = input.split_once("\n\n").unwrap();
    let draw_pile = draw_pile.get_numbers().collect();
    let bingo_cards = bingo_cards
        .split("\n\n")
        .map(|card| BingoCard::new(card).unwrap())
        .collect();
    (draw_pile, bingo_cards)
}

fn part1(input: &Input) -> Output {
    let (draw_pile, bingo_cards) = input;
    let mut draw_pile = draw_pile.iter();
    let mut called_numbers: FxHashSet<Int> = draw_pile.by_ref().take(4).cloned().collect();
    let mut last = 0;
    let winner = draw_pile
        .filter_map(|&draw| {
            last = draw;
            called_numbers.insert(last);
            bingo_cards.iter().find(|card| card.bingo(&called_numbers))
        })
        .next()
        .unwrap();
    winner.score(&called_numbers) * last
}

fn part2(input: &Input) -> Output {
    let (draw_pile, bingo_cards) = input;
    let mut called_numbers: FxHashSet<Int> = draw_pile.iter().cloned().collect();
    let (&last_draw, winner) = draw_pile.iter()
        .rev()
        .filter_map(|draw| {
            called_numbers.remove(draw);
            bingo_cards
                .iter()
                .find(|card| !card.bingo(&called_numbers))
                .map(|card| (draw, card))
        })
        .next()
        .unwrap();
    called_numbers.insert(last_draw);
    winner.score(&called_numbers) * last_draw
}

#[test]
fn default() {
    let input = get_input(21, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(39902, part1(&input));
    assert_eq!(26936, part2(&input));
}

// Input parsed (189μs)
// 1. 39902 (732μs)
// 2. 26936 (54μs)
// Total: 979μs