use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a [Hand];
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone)]
struct Hand {
    cards: Vec<u8>, // convert chars into u8s ordered by card strength
    bid: usize,
}

impl Hand {
    fn new(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards.as_bytes().iter()
            .map(|&c| {
                match c {
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    num => num - 48,
                }
            })
            .collect();
        let bid = bid.parse().unwrap();
        Self { cards, bid }
    }

        // returns an array used for comparing hand strength. Each element in the array is compared, and
    // if tied, the next element is compared.
    fn hand_strength(&self, jacks_are_jokers: bool) -> [u8; 7] {
        // number of jokers gets added to the most numerous of the other cards to make the most powerful hand
        let jokers = if jacks_are_jokers {
            self.cards.iter()
                .filter(|&&c| c == 11)
                .count() as u8
        } else {
            0
        };

        // groups cards together, for use in determining the two most frequently occurring cards.
        let mut groups = [0u8; 15];
        for &card in self.cards.iter() {
            if !jacks_are_jokers || card != 11 {
                groups[card as usize] += 1;
            }
        }

        // get the frequency of the two most frequently occurring cards
        let (first, second) = groups.into_iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(2)
            .collect_tuple()
            .unwrap();

        let mut strength = [0u8; 7];

        strength[0] = first + jokers; // # of most frequently occurring card, plus any jokers
        strength[1] = second; // # of second most frequently occurring card

        // remainder of array is filled with the strength of each individual card in the hand
        for (index, &card) in self.cards.iter().enumerate() {
            strength[index + 2] = if jacks_are_jokers && card == 11 { 0 } else { card }
        }
        strength
    }
}


fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new).collect()
}

// takes the hands, sorts by the hand strength as defined by each puzzle part, assigns points using rank and
// bid amount, then returns sum of all points
fn solve(hands: &[Hand], jacks_are_jokers: bool) -> usize {
    hands.iter()
        .sorted_by_cached_key(|hand| hand.hand_strength(jacks_are_jokers))
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

fn part1(hands: Input) -> Output {
    solve(hands, false)
}

fn part2(hands: Input) -> Output {
    solve(hands, true)
}

#[test]
fn default() {
    let input = get_input(23, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(253866470, part1(&input));
    assert_eq!(254494947, part2(&input));
}
