use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

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

fn solve(manifold: &str) -> [u64; 2] {
    let manifold = manifold.as_bytes();
    let width = manifold.iter().position(|&b| b == b'\n').unwrap() + 1;
    let finished = manifold.len() - width * 2 + 1;

    let mut splits = 0;

    let mut todo = vec![0u64; width - 1];
    let mut next = vec![0u64; width - 1];
    todo[manifold.iter().position(|&b| b == b'S').unwrap()] = 1;

    for row in (0..manifold.len()).step_by(width) {
        for (pos, &timeline) in todo.iter().enumerate() {
            if timeline == 0 {
                continue;
            }
            if manifold[pos + row] == b'^' {
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
        next = vec![0; width - 1];
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

// Input parsed (56μs)
// 1. 1533 (7μs)
// 2. 10733529153890 (2μs)
// Total: 67μs
