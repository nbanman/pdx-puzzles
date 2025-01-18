use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = FxHashMap<String, Int>;
type Int = u32;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut path = Vec::new();
    let mut directories = FxHashMap::default();

    for line in input.lines() {
        if let Some((first, remainder)) = line.split_once(' ') {
            if first == "$" {
                if let Some((_, directory_name)) = remainder.split_once(' ') {
                    if directory_name == ".." {
                        path.pop();
                    } else {
                        path.push(directory_name);
                    }
                }
            } else if let Ok(file_size) = first.parse::<Int>() {
                let mut path_name = String::new();
                path.iter().for_each(|folder| {
                    path_name.push_str(folder);
                    let current_file_size: &mut Int = directories
                        .entry(String::from(path_name.clone()))
                        .or_insert(0);
                    *current_file_size += file_size;
                })
            }
        }
    }
    directories
}

fn part1(directories: &Input) -> Int {
    directories.values().filter(|file_size| file_size <= &&100_000).sum()
}

fn part2(directories: &Input) -> Int {
    let space_available = 70_000_000 - directories.get(&String::from("/")).unwrap();
    let min_dir_size = 30_000_000 - space_available;

    *directories.values().filter(|file_size| file_size >= &&min_dir_size).min().unwrap()
}

#[test]
fn default() {
    let input = get_input(22, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(1477771, part1(&input));
    assert_eq!(3579501, part2(&input));
}

// Input parsed (130μs)
// 1. 1477771 (4μs)
// 2. 3579501 (1μs)
// Total: 139μs