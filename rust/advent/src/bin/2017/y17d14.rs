use advent::utilities::{get_input::get_input, hashes::dense_hash};
use itertools::Itertools;
use utilities::structs::{grid::Grid, stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<String>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    (0..128)
        .map(|i| {
            let mut prep = input.to_string();
            prep.push('-');
            prep.push_str(&i.to_string());
            let prep = prep.as_bytes().iter().copied()
                .chain([17, 31, 73, 47, 23].into_iter())
                .map(|b| b as usize)
                .collect_vec();
            dense_hash(&prep).chars()
                .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
                .collect()
        })
        .collect()
}

fn part1(rows: &Input) -> Output {
    rows.iter()
        .map(|row| row.as_bytes().iter().filter(|&&b| b == b'1').count())
        .sum()
}

fn part2(input: &Input) -> Output {
    let grid = Grid::new2d_map_str(
        input.iter().join("\n").as_str(),
        |c| c == '1'
    )
        .unwrap();

    let mut regions = 0;
    let mut visited = vec![false; grid.len()];
    let mut queue = Vec::new();

    // basic flood fill starting from each '1' position on the grid, exiting if already visited in earlier pass
    for (pos, _) in grid.iter().enumerate().filter(|&(_, &used)| used) {
        if visited[pos] { continue; }
        regions += 1;
        queue.push(pos);
        while let Some(current) = queue.pop() {
            for n_pos in grid.adjacent(current, false)
                .unwrap()
                .filter(|adj| *adj.value)
                .map(|adj| adj.index)
            {
                if visited[n_pos] {
                    continue;
                } else {
                    visited[n_pos] = true;
                }
                queue.push(n_pos);
            }
        }
    }
    
    regions
}

#[test]
fn default() {
    let input = get_input(17, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(8222, part1(&input));
    assert_eq!(1086, part2(&input));
}

// Input parsed (18ms)
// 1. 8222 (9μs)
// 2. 1086 (1ms)
// Total: 19ms
