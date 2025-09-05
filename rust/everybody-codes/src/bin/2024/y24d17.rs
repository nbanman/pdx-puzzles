use std::collections::HashSet;
use std::mem;
use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::structs::{
    coord::Coord2U,
    grid::Grid2,
    stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 17);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input1, false), stopwatch.lap().report());
    println!("2. {} ({})", solve(&input2, false), stopwatch.lap().report());
    // println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, brilliant: bool) -> usize {
    let stars: Vec<Pos> = get_stars(input);
    let distances: Grid2<usize> = get_distances(&stars);
    let mut constellations: Vec<Vec<usize>> = (0..stars.len())
        .map(|idx| vec![idx])
        .collect();

    let mut distance_sum = 0;
    let mut brilliants: Vec<usize> = Vec::new();
    let star_indices: HashSet<usize> = (0..stars.len()).collect();

    let mut min_dist = usize::MAX;
    let mut leading_constellation: Option<usize> = None;

    loop {
        let (to_attach_idx, to_attach) = constellations.iter().enumerate()
            .min_by_key(|(_, constellation)| constellation.len())
            .expect("Already checked against empty.");

        min_dist = usize::MAX;
        leading_constellation = None;

        for test_idx in 0..constellations.len() {
            // don't check against yourself
            if to_attach_idx == test_idx { continue; }

            for &test in constellations[test_idx].iter() {
                for &candidate in to_attach.iter() {
                    let dist = distances[candidate * stars.len() + test];
                    if dist < min_dist && (!brilliant || dist < 6){
                        min_dist = dist;
                        leading_constellation = Some(test_idx);
                    }
                }
            }
        }

        if let Some(closest) = leading_constellation {
            distance_sum += min_dist;
            let tmp: Vec<_> = constellations[to_attach_idx].drain(..).collect();
            constellations[closest].extend(tmp);
            constellations.remove(to_attach_idx);
        } else {
            brilliants.push(distance_sum + stars.len());
            distance_sum = 0;
            constellations.remove(to_attach_idx);
            if constellations.is_empty() { break; }
        }
    }

    let total_distance = brilliants.into_iter().reduce(|acc, dist| acc * dist).unwrap();

    total_distance
}

fn part2(input: Input) -> usize {
    todo!()
}

fn part3(input: Input) -> usize {
    todo!()
}

fn get_stars(input: Input) -> Vec<Pos> {
    Grid2::try_from(input)
        .unwrap()
        .iter_with_coords()
        .filter_map(|(pos, &c)| if c == '*' { Some(pos) } else { None })
        .collect()
}

fn get_distances(stars: &[Pos]) -> Grid2<usize> {
    let length = stars.len();
    let mut distances = Vec::with_capacity(length * length);
    for a in stars.into_iter() {
        for b in stars.into_iter() {
            if a == b {
                distances.push(usize::MAX);
            } else {
                distances.push(a.manhattan_distance(b));
            }
        }
    }
    Grid2::new2d(distances, length).unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 17);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn examples() {
    let inputs = [r"*...*
..*..
.....
.....
*.*..", r".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
.......................................",];
    assert_eq!(16, solve(inputs[0], false));
    assert_eq!(15624, solve(inputs[1], true));
}
