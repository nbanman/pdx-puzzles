use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (usize, usize);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 9).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct Marble {
    id: usize,
    value: usize,
    left: usize,
    right: usize,
}
impl Marble {
    fn new(value: usize, marbles: &mut Vec<Marble>) -> usize {
        let id = marbles.len();
        let marble = Marble { id, value, left: id, right: id };
        marbles.push(marble);
        id
    }

    fn go_left(&self, n: usize, marbles: &[Marble]) -> usize {
        if n == 0 {
            self.id
        } else {
            marbles[self.left].go_left(n-1, marbles)
        }
    }
}

fn add_right(id: usize, right_value: usize, marbles: &mut Vec<Marble>) -> usize {
    // make a dummy marble and swap it with the current marble
    let mut current = Marble { id: 0, value: 0, left: 0, right: 0 };
    std::mem::swap(&mut current, &mut marbles[id]);
    let right_id = Marble::new(right_value, marbles);
    let right = marbles.get_mut(right_id).unwrap();

    right.left = id;
    right.right = current.right;

    // Normally you want to edit the marble to the right of the current marble, but on the first
    // pass that is itself! Which means that on the first pass only, we want to edit the
    // current marble that is swapped out of the marbles vec, rather than a marble in the vec.
    if marbles.len() == 2 {
        current.left = right_id;
    } else {
        marbles[current.right].left = right_id;
    }
    current.right = right_id;

    // swap back
    std::mem::swap(&mut current, &mut marbles[id]);
    
    right_id
}

fn remove(id: usize, marbles: &mut Vec<Marble>) -> usize {
    // make a dummy marble and swap it with the current marble
    let mut current = Marble { id: 0, value: 0, left: 0, right: 0 };
    std::mem::swap(&mut current, &mut marbles[id]);

    marbles[current.left].right = current.right;
    marbles[current.right].left = current.left;

    current.right
}

fn parse_input(input: &str) -> Input {
    let mut input = input.get_numbers();
    (input.next().unwrap(), input.next().unwrap())
}

fn solve(&(players, highest_value): &(usize, usize), multiplier: usize) -> Output {
    let last_marble = highest_value * multiplier;
    let mut marbles: Vec<Marble> = Vec::new();
    let mut scores = vec![0usize; players];
    let mut current = Marble:: new(0, &mut marbles);

    for x in 1..=last_marble {
        if x % 23 == 0 {
            current = marbles[current].go_left(7, &marbles);
            scores[(x - 1) % players] += x + marbles[current].value;
            current = remove(current, &mut marbles);
        } else {
            let right = marbles[current].right;
            current = add_right(right, x, &mut marbles);
        }
    }
    scores.into_iter().max().unwrap()
}

fn part1(input: &Input) -> Output {
    solve(input, 1)
}

fn part2(input: &Input) -> Output {
    solve(input, 100)
}

#[test]
fn default() {
    let input = get_input(18, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(422980, part1(&input));
    assert_eq!(3552041936, part2(&input));
}

// Input parsed (11μs)
// 1. 422980 (1ms)
// 2. 3552041936 (54ms)
// Total: 55ms