use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Pattern>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

// Stores data in both rows and columns for easy operation.
#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

impl Pattern {
    fn new(s: &str) -> Self {
        let width = s.find('\n').unwrap();

        // intermediate representation
        let mirrors: Vec<bool> = s
            .as_bytes()
            .iter()
            .filter(|&&c| c != b'\n')
            .map(|&c| c == b'#')
            .collect();
        let height = mirrors.len() / width;
        let rows = (0..height)
            .map(|n| {
                let offset = n * width;
                mirrors[offset..offset + width].iter().copied().collect()
            })
            .collect();
        let cols = (0..width)
            .map(move |n| (0..height).map(|row| mirrors[row * width + n]).collect())
            .collect();
        Pattern { rows, cols }
    }

    // calls find_seam for columns first. If a seam is found, return the value.
    // Otherwise, calls find_seam for rows. If a seam is found, return the value * 100.
    // Otherwise, panic!
    fn seam_summary(&self, smudged: bool) -> usize {
        self.find_seam(true, smudged)
            .unwrap_or_else(|| self.find_seam(false, smudged).unwrap() * 100)
    }

    // Goes through each "line" (either a row or column), and compares it with the next one.
    // If there is a mirrored line, checks if the mirroring continues on either side. If it
    // continues until there are no more lines to be compared, a seam is found.
    //
    // In part 2, there is one line that has a difference of 1 (the smudge). So track
    // differences with "diff" and proceed accordingly.
    fn find_seam(&self, is_hz: bool, smudged: bool) -> Option<usize> {
        let smudge = if smudged { 1 } else { 0 };
        let lines = if is_hz { &self.cols } else { &self.rows };
        for i in 0..lines.len() - 1 {
            let mut diff = 0;
            for j in 0..=i {
                if i + j + 1 == lines.len() {
                    break;
                }
                diff += lines[i - j]
                    .iter()
                    .zip(lines[i + j + 1].iter())
                    .filter(|&(&aa, &bb)| aa != bb)
                    .count();
                if diff > smudge {
                    break;
                }
            }
            if smudge == diff {
                return Some(i + 1);
            };
        }
        None
    }
}

fn parse_input(input: &str) -> Input {
    input.split("\n\n").map(Pattern::new).collect()
}

fn part1(patterns: &Input) -> Output {
    patterns
        .iter()
        .map(|pattern| pattern.seam_summary(false))
        .sum()
}

fn part2(patterns: &Input) -> Output {
    patterns
        .iter()
        .map(|pattern| pattern.seam_summary(true))
        .sum()
}

#[test]
fn default() {
    let input = get_input(23, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(27505, part1(&input));
    assert_eq!(22906, part2(&input));
}

// Input parsed (158μs)
// 1. 27505 (18μs)
// 2. 22906 (17μs)
// Total: 195μs
