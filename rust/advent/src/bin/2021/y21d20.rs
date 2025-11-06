use std::cmp::max;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::{coord::Coord2U, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = (Algorithm, Image);
type Output = usize;
type Algorithm = Vec<bool>;
type Image = Grid2<bool>;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm.as_bytes().iter().map(|&b| b == b'#').collect();
    let image = Image::new2d_map_str(image, |c| c == '#').unwrap();
    (algorithm, image)
}

fn enhance((algorithm, image): &Input, steps: usize) -> Output {
    let offset = Pos::from((1, 1));
    let enhanced = (0..steps).fold(image.clone(), |acc, step| {
        let outside = step & 1 == 1;
        let next = Image::new2d_with_fn(acc.width() + 2, acc.height() + 2, |i| {
            let pos = Pos::from_index(i, acc.width() + 2).unwrap();
            let algo_index: usize = pos_to_nine(pos)
                .map(|it| {
                    if it.x() > 0 && it.x() <= acc.width() && it.y() > 0 && it.y() <= acc.height() {
                        let diff = it - offset;
                        let idx = diff.y() * acc.width() + diff.x();
                        acc.data[idx]
                    } else {
                        outside
                    }
                })
                .collect_vec()
                .into_iter()
                .rev()
                .enumerate()
                .map(|(index, bit)| if bit { 1 << index } else { 0 })
                .sum();
            algorithm[algo_index]
        });
        next
    });
    enhanced.iter().filter(|&&b| b).count()
}

fn pos_to_nine(pos: Pos) -> impl Iterator<Item = Pos> {
    let x = pos.x() as i64;
    let y = pos.y() as i64;
    (-1..=1).cartesian_product(-1..=1)
        .map(move |(yo, xo)| {
            let x = max(x + xo, 0) as usize;
            let y = max(y + yo, 0) as usize;
            Pos::from((x, y))
        })
}

fn part1(input: &Input) -> Output {
    enhance(input, 2)
}

fn part2(input: &Input) -> Output {
    enhance(input, 50)
}

#[test]
fn default() {
    let input = get_input(21, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(5786, part1(&input));
    assert_eq!(16757, part2(&input));
}

// Input parsed (65μs)
// 1. 5786 (1ms)
// 2. 16757 (61ms)
// Total: 62ms