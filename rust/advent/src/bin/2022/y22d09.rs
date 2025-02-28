use advent::utilities::get_input::get_input;
use rustc_hash::FxHashSet;
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Pos = Coord2;
type Input = Vec<Pos>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 9).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        // flattens out a vec containing vectors repeated several times
        .flat_map(|line| {
            // for each line, separate the direction (UDLR) and the distance (#), then convert the
            // direction to a vector and the distance to usize
            let (dir, dist) = line.split_once(' ').unwrap();
            let dir = match dir {
                "U" => Cardinal::North,
                "D" => Cardinal::South,
                "L" => Cardinal::West,
                "R" => Cardinal::East,
                dir => panic!("unknown direction `{}`", dir),
            };
            let dist = dist.parse::<usize>().unwrap();
            vec![dir; dist]
        })
        // takes the flattened directions and uses it to plot the movement of the original knot
        .scan(Pos::origin(), |pos, dir| {
            *pos = pos.move_direction(dir, 1).unwrap();
            Some(*pos)
        })
        .collect()
}

fn solve(first_knot: Input, knots: usize) -> usize {
    (1..knots)
        .fold(first_knot, |prev_rope, _| {
            let mut pos = Pos::origin();
            let mut rope = vec![pos];
            for prev_pos in prev_rope {
                let diff = prev_pos - pos;
                if diff.x().abs() > 1 || diff.y().abs() > 1 {
                    pos += Pos::new2d(diff.x().signum(), 0);
                    pos += Pos::new2d(0, diff.y().signum());
                    rope.push(pos);
                }
            }
            rope
        })
        .into_iter()
        .collect::<FxHashSet<_>>()
        .len()
}

fn part1(first_knot: &Input) -> Output {
    solve(first_knot.clone(), 2)
}

fn part2(first_knot: &Input) -> Output {
    solve(first_knot.clone(), 10)
}

#[test]
fn default() {
    let input = get_input(22, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(6175, part1(&input));
    assert_eq!(2578, part2(&input));
}

// Input parsed (234μs)
// 1. 6175 (300μs)
// 2. 2578 (242μs)
// Total: 780μs
