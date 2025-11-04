use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{minmax::minmax, parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Area;
type Pos = Coord2;

#[derive(Debug)]
struct Area {
    tl: Pos,
    br: Pos,
}

impl Area {
    fn max_y(&self) -> i64 {
        (1..self.br.y().abs()).sum()
    }

    fn y_steps(&self, v: Vector) -> Vec<i64> {
        let time_offset = if v.vel.y() >= 0 {
            v.vel.y() * 2 + 1
        } else {
            0
        };

        let velocity = if v.vel.y() >= 0 {
            -(v.vel.y() + 1)
        } else {
            v.vel.y()
        };

        let mut cromulent = Vec::new();
        let mut shot = Vector { pos: Pos::origin(), vel: Pos::new2d(0, velocity) };
        let mut time = 0;
        while shot.pos.y() >= self.br.y() {
            time += 1;
            shot = shot.step();
            if (self.br.y()..=self.tl.y()).contains(&shot.pos.y()) {
                cromulent.push(time + time_offset);
            }
        }
        cromulent
    }

    fn x_values(&self, steps: i64) -> Vec<i64> {
        let x_range = self.tl.x()..=self.br.x();
        let stall_speed = (5..=self.br.x())
            .find(|&it| {
                let sum = (1..=it).sum::<i64>();
                x_range.contains(&sum)
            })
            .unwrap();
        (stall_speed..=self.br.x())
            .filter(move |&x_vel| {
                let x = (1..=steps)
                    .fold(Vector { pos: Pos::origin(), vel: Pos::new2d(x_vel, 0) }, |acc, _| {
                        acc.step()
                    });
                x_range.contains(&x.pos.x())
            })
            .collect_vec()
    } 
}

#[derive(Debug)]
struct Vector {
    pos: Pos,
    vel: Pos,
}

impl Vector {
    fn step(&self) -> Self {
        let new_xv = match self.vel.x() {
            x if x > 0 => x - 1,
            x if x < 0 => x + 1,
            x => x,
        };
        Self { pos: self.pos + self.vel, vel: Pos::new2d(new_xv, self.vel.y() - 1) }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (x1, x2, y1, y2) = input.get_numbers::<i64>().collect_tuple().unwrap();
    let (&xa, &xb) = minmax(&x1, &x2);
    let (&ya, &yb) = minmax(&y1, &y2);
    Area { tl: Pos::new2d(xa, yb), br: Pos::new2d(xb, ya) }
}

fn part1(area: &Input) -> i64 {
    area.max_y()
}

fn part2(area: &Input) -> usize {
    (area.br.y() ..= area.max_y())
        .map(|y| (y, area.y_steps(Vector { pos: Pos::origin(), vel: Pos::new2d(0, y) })))
        .filter(|(_, y_steps)| !y_steps.is_empty())
        .flat_map(|(y, y_steps)| {
            y_steps.iter()
                .flat_map(|&y_step| {
                    area.x_values(y_step).iter().map(|&x| Pos::new2d(x, y)).collect_vec()
                })
                .collect_vec()
        })
        .unique()
        .count()
}

#[test]
fn default() {
    let input = get_input(21, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(17766, part1(&input));
    assert_eq!(1733, part2(&input));
}

// Input parsed (18μs)
// 1. 17766 (6μs)
// 2. 1733 (1ms)
// Total: 1ms
