use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Vec<Particle>;
type Output = usize;
type Pos = Coord<i64, 3>;

#[derive(Debug, Copy, Clone)]
struct Particle {
    number: usize,
    p: Pos,
    v: Pos,
    a: Pos,
}

impl Particle {
    fn particle_at(&self, time: usize) -> Self {
        if time == 0 {
            *self
        } else {
            let v = self.v + self.a;
            let p = self.p + v;
            let new_particle = Particle { number: self.number, p, v, a: self.a };
            new_particle.particle_at(time - 1)
        }
    }
    fn stable_time(&self) -> usize {
        [
            self.stable_axis(Pos::x),
            self.stable_axis(Pos::y),
            self.stable_axis(Pos::z),
        ]
            .into_iter()
            .max()
            .unwrap()
    }
    
    fn stable_axis<F>(&self, axis: F) -> usize
    where F: Fn(&Pos) -> i64,
    {
        let v = axis(&self.v);
        let a = axis(&self.a);
        if v * a >= 0 {
            0
        } else {
            (v / a).abs() as usize
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().chunks(9).into_iter().enumerate()
        .map(|(number, chunk)| {
            let (px, py, pz, vx, vy, vz, ax, ay, az) = chunk.into_iter().collect_tuple().unwrap();
            let p = Pos::new3d(px, py, pz);
            let v = Pos::new3d(vx, vy, vz);
            let a = Pos::new3d(ax, ay, az);
            Particle { number, p, v, a }
        })
        .sorted_unstable_by_key(|particle| particle.a.manhattan_distance(&Pos::origin()))
        .collect()
}

fn part1(particles: &Input) -> Output {
    let closest = particles[0].a.manhattan_distance(&Pos::origin());
    let select_particles = particles.iter()
        .take_while(|particle| particle.a.manhattan_distance(&Pos::origin()) == closest)
        .collect_vec();
    let offset = select_particles.iter()
        .map(|particle| particle.stable_time())
        .max()
        .unwrap();
    select_particles.iter()
        .max_by_key(|particle| particle.particle_at(offset).p.manhattan_distance(&Pos::origin()))
        .map(|particle| particle.number)
        .unwrap()
}

fn part2(particles: &Input) -> Output {
    successors(Some(particles.clone()), |prev| {
        Some(
            prev.iter()
                .into_group_map_by(|particle| particle.p)
                .into_iter()
                .filter(|(_, particles)| particles.len() == 1)
                .map(|(_, particle)| particle[0].particle_at(1))
                .collect_vec()
        )
    })
        .take(1000)
        .last()
        .unwrap()
        .len()
}

#[test]
fn default() {
    let input = get_input(17, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(308, part1(&input));
    assert_eq!(504, part2(&input));
}

// Input parsed (295μs)
// 1. 308 (9μs)
// 2. 504 (48ms)
// Total: 48ms
