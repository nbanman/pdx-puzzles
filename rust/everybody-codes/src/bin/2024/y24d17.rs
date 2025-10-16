use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
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
    println!("3. {} ({})", solve(&input3, true), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, brilliant: bool) -> usize {
    let stars: Vec<Pos> = get_stars(input);
    let distances: Grid2<usize> = get_distances(&stars);
    let mut constellations: Vec<(usize, Vec<usize>)> = (0..stars.len())
        .map(|idx| (0, vec![idx]))
        .collect();

    let mut brilliants: Vec<usize> = Vec::new();

    let mut min_dist: usize;
    let mut leading_constellation: Option<usize>;

    loop {
        min_dist = usize::MAX;
        leading_constellation = None;

        let (to_attach_idx, (distance_sum, to_attach)) = constellations.iter().enumerate()
            .min_by_key(|(_, (_, constellation))| constellation.len())
            .expect("Already checked against empty.");

        for test_idx in 0..constellations.len() {
            // don't check against yourself
            if to_attach_idx == test_idx { continue; }

            for &test in constellations[test_idx].1.iter() {
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
            let tmp_dist = constellations[to_attach_idx].0;
            constellations[closest].0 += min_dist + tmp_dist;
            let tmp_stars: Vec<_> = constellations[to_attach_idx].1.drain(..).collect();
            constellations[closest].1.extend(tmp_stars);
            constellations.remove(to_attach_idx);
        } else {
            brilliants.push(distance_sum + to_attach.len());
            constellations.remove(to_attach_idx);
            if constellations.is_empty() { break; }
        }
    }

    brilliants
        .into_iter()
        .sorted_unstable()
        .rev()
        .take(3)
        .reduce(|acc, dist| acc * dist).unwrap()
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
                distances.push(a.manhattan_distance(*b));
            }
        }
    }
    Grid2::new2d(distances, length).unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 17);
    assert_eq!(141, solve(&input1, false));
    assert_eq!(1270, solve(&input2, false));
    assert_eq!(99999, solve(&input3, true));
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
    // assert_eq!(16, solve(inputs[0], false));
    assert_eq!(15624, solve(inputs[1], true));
}

// Input parsed (41μs)
// 1. 141 (42μs)
// 2. 1270 (868μs)
// 3. 5097626928 (29ms)
// Total: 30ms