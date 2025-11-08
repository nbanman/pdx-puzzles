use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::{Itertools, MinMaxResult};
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Int = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 5);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, PartialEq, Eq)]
struct Segment {
    spine: Int,
    left: Option<Int>,
    right: Option<Int>,
}

impl Segment {
    fn place(&mut self, int: Int) -> bool {
        if int < self.spine && self.left.is_none() {
            self.left = Some(int);
            true
        } else if int > self.spine && self.right.is_none() {
            self.right = Some(int);
            true
        } else {
            false
        }
    }

    fn number(&self) -> Int {
        concat(
            concat(self.left.unwrap_or_default(), self.spine),
            self.right.unwrap_or_default(),
        )
    }

    fn is_closed(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sword {
    id: Int,
    segments: Vec<Segment>,
}

impl Sword {
    fn quality(&self) -> Int {
        self.segments.iter().fold(0, |acc, segment| concat(acc, segment.spine))
    }
}

impl From<&str> for Sword {
    fn from(value: &str) -> Self {
        let mut ints = value.get_numbers::<Int>();
        let id = ints.next().unwrap();
        let mut segments = vec![Segment {
            spine: ints.next().unwrap(),
            left: None,
            right: None,
        }];
        let mut open = 0;

        'outer: for int in ints {
            let mut found_open = false;
            for idx in open..segments.len() {
                let segment = segments.get_mut(idx).unwrap();
                if segment.is_closed() {
                    if !found_open {
                        open = idx + 1;
                    }
                    continue;
                } else {
                    found_open = true;
                }
                if segment.place(int) {
                    continue 'outer;
                }
            }
            segments.push(Segment {
                spine: int,
                left: None,
                right: None,
            })
        }
        Self { id, segments }
    }
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quality().cmp(&other.quality())
            .then_with(|| {
                self.segments.iter()
                    .zip(other.segments.iter())
                    .map(|(a, b)| a.number().cmp(&b.number()))
                    .find(|&ordering| ordering != std::cmp::Ordering::Equal)
                    .unwrap_or_else(|| self.id.cmp(&other.id))
            })
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn concat(a: Int, b: Int) -> Int {
    let mut pow = 1;
    while pow <= b {
        pow *= 10;
    }
    a * pow + b
}

fn part1(input: Input) -> Int {
    Sword::from(input).quality()
}

fn part2(input: Input) -> Int {
    let minmax = input.lines()
        .map(|line| Sword::from(line).quality())
        .minmax();
    if let MinMaxResult::MinMax(min, max) = minmax {
        return max - min;
    }
    unreachable!()
}

fn part3(input: Input) -> Int {
    input.lines()
        .map(|line| Sword::from(line))
        .sorted_unstable()
        .rev()
        .enumerate()
        .map(|(idx, sword)| (idx as Int + 1) * sword.id)
        .sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 5);
    assert_eq!(2782784532, part1(&input1));
    assert_eq!(8637361015798, part2(&input2));
    assert_eq!(31574813, part3(&input3));
}

// Input parsed (41μs)
// 1. 2782784532 (9μs)
// 2. 8637361015798 (89μs)
// 3. 31574813 (664μs)
// Total: 808μs