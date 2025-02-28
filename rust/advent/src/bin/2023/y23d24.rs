use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord3,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Debug)]
struct Stone {
    pos: Point,
    vel: Point,
}

impl Stone {
    fn intersects(&self, other: &Stone) -> bool {
        let t = (other.pos.y + (self.pos.x - other.pos.x) * other.vel.y / other.vel.x - self.pos.y)
            / (self.vel.y - self.vel.x * other.vel.y / other.vel.x);
        if t < 0.0 {
            return false;
        }

        let s = (t * self.vel.x + self.pos.x - other.pos.x) / other.vel.x;
        if s < 0.0 {
            return false;
        }

        let x = t * self.vel.x + self.pos.x;
        let y = t * self.vel.y + self.pos.y;

        let lower = 200000000000000.0;
        let upper = 400000000000000.0;

        x >= lower && x <= upper && y >= lower && y <= upper
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 24).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    input
        .get_numbers::<i64>()
        .tuples()
        .map(|(x, y, _)| Point {
            x: x as f64,
            y: y as f64,
        })
        .tuples()
        .map(|(pos, vel)| Stone { pos, vel })
        .combinations(2)
        .filter(|combo| {
            let a = combo[0];
            let b = combo[1];
            a.intersects(&b)
        })
        .count()
}

fn part2(input: Input) -> i64 {
    let stones: Vec<(Coord3, Coord3)> = input
        .get_numbers::<i64>()
        .tuples::<(_, _, _)>()
        .map(|(x, y, z)| Coord3::new3d(x, y, z))
        .tuples::<(_, _)>()
        .collect();

    let mut times = Vec::new();

    'outer: for axis in 0..=2 {
        'mid: for &(pos, vel) in stones.iter() {
            for &(other_pos, other_vel) in stones.iter() {
                if vel.get(axis) == other_vel.get(axis) {
                    if pos.get(axis) == other_pos.get(axis) {
                        times.push(0);
                    } else {
                        times.clear();
                        continue 'mid;
                    }
                } else {
                    let diff_pos = (other_pos.get(axis).unwrap() - pos.get(axis).unwrap()).abs();
                    let diff_vel = (other_vel.get(axis).unwrap() - vel.get(axis).unwrap()).abs();
                    if diff_pos % diff_vel == 0 {
                        times.push(diff_pos / diff_vel)
                    } else {
                        times.clear();
                        continue 'mid;
                    }
                }
            }
            if !times.is_empty() {
                break 'outer;
            }
        }
    }

    let (i, j) = times
        .iter()
        .enumerate()
        .filter(|&(_, &time)| time > 0)
        .map(|(idx, _)| idx)
        .take(2)
        .next_tuple()
        .unwrap();

    fn rock_pos(stones: &Vec<(Coord3, Coord3)>, i: usize, axis: usize, time: i64) -> i64 {
        stones[i].0.get(axis).unwrap() + stones[i].1.get(axis).unwrap() * time
    }

    (0..=2)
        .map(|axis| {
            let i_pos = rock_pos(&stones, i, axis, times[i]);
            i_pos
                - (i_pos - rock_pos(&stones, j, axis, times[j])) / (times[i] - times[j]) * times[i]
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(23, 24).unwrap();
    assert_eq!(14046, part1(&input));
    assert_eq!(808107741406756, part2(&input));
}

// Input parsed (18μs)
// 1. 14046 (1ms)
// 2. 808107741406756 (51μs)
// Total: 1ms
