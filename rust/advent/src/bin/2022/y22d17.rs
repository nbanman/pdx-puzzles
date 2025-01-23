use std::{cmp::max, collections::HashMap};

use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

const ROCKS: &[(i8, &[i8])] = &[
    (4, &[15]),
    (3, &[2, 7, 2]),
    (3, &[7, 4, 4]),
    (1, &[1, 1, 1, 1]),
    (2, &[3, 3]),
];

struct State {
    height: usize,
    rows: Vec<u8>,
}

impl State {
    fn contains(&self, x: i8, y: usize, rock: &[i8]) -> bool {
        rock.iter().enumerate().any(|(dy, &row)| {
            let Some(&other) = self.rows.get(y + dy) else { return false; };
            (row << x) as u8 & other != 0
        })
    }

    fn plus(&self, x: i8, y: usize, rock: &[i8]) -> Self {
        let row_size = max(self.rows.len(), y + rock.len());
        let mut rows = Vec::with_capacity(row_size);
        rows.extend_from_slice(&self.rows);
        for _ in self.rows.len()..row_size {
            rows.push(0);
        }

        let height = self.height + rows.len() - self.rows.len();
        for (dy, &row) in rock.iter().enumerate() {
            rows[y + dy] = rows[y + dy] | (row << x) as u8;
        }
        let mut visible = vec![0u8; row_size + 1];
        visible[row_size] = 1;
        let mut q = Vec::new();
        let start = row_size << 3;
        q.push(start);
        while let Some(pos) = q.pop() {
            if pos < start && rows[pos >> 3] & (1 << (pos & 7)) != 0 {
                continue;
            }
            for next in [pos - 8, pos - 1, pos + 1, pos + 8] {
                if next & 7 != 7 && (0..(visible.len() << 3)).contains(&next) &&
                    visible[next >> 3] & (1 << (next & 7)) == 0 
                {
                    visible[next >> 3] = visible[next >> 3] | (1 << (next & 7));
                    q.push(next);
                }
            }
        }
        for (i, row) in rows.iter_mut().enumerate() {
            *row = *row & visible[i];
        }
        let trim = rows.iter().enumerate()
            .find(|(_, &row)| row != 0)
            .map(|(i, _)| i)
            .unwrap_or_default();
        
        Self { height, rows: rows[trim as usize..].to_owned() }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 17).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(n: usize, jet: Input) -> Output {
    let jet = jet.trim();
    let mut jet_index = 0;
    let mut state = State { height: 0, rows: vec![127] };
    let mut seen: HashMap<(usize, usize, Vec<u8>), usize, rustc_hash::FxBuildHasher> = 
        FxHashMap::default();
    let mut heights: Vec<usize> = Vec::new();

    for i in 0..n {
        let rock_index = i % 5;
        if let Some(j) = seen.insert((rock_index, jet_index, state.rows.clone()), i) {
            let q = (n - j) / (i - j);
            let r = (n -j) % (i - j);
            return heights[j + r] + q * (state.height - heights[j]);
        } 
        let (width, rock) = ROCKS[rock_index];
        let mut x: i8 = 2;
        let mut y = state.rows.len() + 3;
        while !state.contains(x, y, rock) {
            let x2 = match jet.as_bytes()[jet_index] {
                b'<' => x - 1,
                b'>' => x + 1,
                c => panic!("{} is not a valid wind direction", c as char)
            };
            jet_index = if jet_index == jet.len() - 1 {
                0
            } else {
                jet_index + 1
            };
            if x2 >= 0 && x2 <= 7 - width && !state.contains(x2, y, rock) {
                x = x2;
            }
            y -= 1;
        }
        heights.push(state.height);
        state = state.plus(x, y + 1, rock);
    }
    state.height
}


fn part1(jet: Input) -> Output {
    solve(2022, jet)
}

fn part2(jet: Input) -> Output {
    solve(1_000_000_000_000, jet)
}

#[test]
fn default() {
    let input = get_input(22, 17).unwrap();
    assert_eq!(3055, part1(&input));
    assert_eq!(1507692307690, part2(&input));
}

// Input parsed (16μs)
// 1. 3055 (1ms)
// 2. 1507692307690 (1ms)
// Total: 3ms