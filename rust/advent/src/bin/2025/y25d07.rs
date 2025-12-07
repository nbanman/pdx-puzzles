use advent::utilities::get_input::get_input;
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 7).unwrap();
    let [part1, part2] = solve(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1, stopwatch.lap().report());
    println!("2. {} ({})", part2, stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: &str) -> [u64; 2] {
    let manifold = Grid2::try_from(input).unwrap();
    let finished = manifold.len() - manifold.width() * 2;

    let mut splits = 0;

    let mut todo = vec![0u64; manifold.width()];
    let mut next = vec![0u64; manifold.width()];
    todo[manifold.iter().position(|&c| c == 'S').unwrap()] = 1;

    for row in (0..manifold.len()).step_by(manifold.width()) {
        for (pos, &timeline) in todo.iter().enumerate() {
            if timeline == 0 {
                continue;
            }
            if manifold[pos + row] == '^' {
                splits += 1;
                next[pos - 1] += timeline;
                next[pos + 1] += timeline;
            } else {
                next[pos] += timeline;
            }
        }
        if row == finished {
            break;
        }
        todo = next;
        next = vec![0; manifold.width()];
    }
    let total_timelines = next.into_iter().sum();
    [splits, total_timelines]
}

#[test]
fn default() {
    let input = get_input(25, 7).unwrap();
    let [part1, part2] = solve(&input);
    assert_eq!(1533, part1);
    assert_eq!(10733529153890, part2);
}

// Input parsed (150μs)
// 1. 1533 (4μs)
// 2. 10733529153890 (1μs)
// Total: 159μs
