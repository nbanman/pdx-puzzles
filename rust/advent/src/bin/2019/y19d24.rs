use advent::utilities::get_input::get_input;
use rustc_hash::FxHashSet;
use utilities::structs::{grid::{Grid2, GridIterator}, stopwatch::{ReportDuration, Stopwatch}};

type Input = Grid2<bool>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    Grid2::new2d_map_str(input, |c| c == '#').unwrap()
}

fn advance(eris: &Input) -> Input {
    eris.iter().enumerate()
        .map(|(index, &b)| {
            let adjacent_bugs = eris.adjacent(index, false)
                .unwrap()
                .filter(|adj| *adj.value)
                .count();
            adjacent_bugs == 1 || (!b && adjacent_bugs == 2)
        })
        .try_collect_grid(eris.width())
        .unwrap()
}

fn biodiversity(eris: &Input) -> Output {
    eris.iter().fold(0, |acc, &b| (acc << 1) | if b { 1 } else { 0 } )
}

fn bit_on(d: usize, pos: usize) -> usize {
    (d >> pos) & 1
}

fn bugs_on_side(d: usize, side: usize) -> usize {
    (d & side).count_ones() as usize
}

fn bug_in_middle(d: usize, side: usize) -> usize {
    (d >> side) & 1
}

fn part1(eris: &Input) -> Output {
    let mut ratings = FxHashSet::default();
    let mut eris = eris.clone();
    ratings.insert(biodiversity(&eris));
    loop {
        eris = advance(&eris);
        let biodiversity = biodiversity(&eris);
        if !ratings.insert(biodiversity) {
            return biodiversity;
        }
    }
}

fn part2(eris: &Input) -> Output {
    let iterations = 200;
    let biodiversity = biodiversity(eris);
    let mut eris = vec![0; 301];
    eris[150] = biodiversity;
    let mut lower_bound = 149;
    let mut upper_bound = 151;

    let up = (0..5).map(|i| 1 << i).sum();
    let down = (20..25).map(|i| 1 << i).sum();
    let left = (0..5).map(|i| 1 << (i * 5)).sum();
    let right = (0..5).map(|i| 1 << (i * 5 + 4)).sum();

    for _ in 1..=iterations {
        eris = (0..eris.len())
            .map(|d| {
                if d < lower_bound || d > upper_bound {
                    0
                } else {
                    let mut new_dimension = 0;
                    for bit in 0..25 {
                        if bit == 12 { continue; }
                        let is_bug = bit_on(eris[d], bit) == 1;
                        let mut bugs = 0;

                        // look up
                        bugs += if bit == 17 {
                            bugs_on_side(eris[d - 1], down)
                        } else if bit < 5 {
                            bug_in_middle(eris[d + 1], 7)
                        } else {
                            bit_on(eris[d], bit - 5)
                        };

                        // look down
                        bugs += if bit == 7 {
                            bugs_on_side(eris[d - 1], up)
                        } else if bit >= 20 {
                            bug_in_middle(eris[d + 1], 17)
                        } else {
                            bit_on(eris[d], bit + 5)
                        };
                        
                        // look left
                        bugs += if bit == 13 {
                            bugs_on_side(eris[d - 1], right)
                        } else if bit % 5 == 0 {
                            bug_in_middle(eris[d + 1], 11)
                        } else {
                            bit_on(eris[d], bit - 1)
                        };

                        // look right
                        bugs += if bit == 11 {
                            bugs_on_side(eris[d - 1], left)
                        } else if bit % 5 == 4 {
                            bug_in_middle(eris[d + 1], 13)
                        } else {
                            bit_on(eris[d], bit + 1)
                        };
                        
                        let new_bug = bugs == 1 || (!is_bug && bugs == 2);
                        if new_bug {
                            new_dimension += 1 << bit;
                        }
                    }
                    new_dimension
                }
            })
            .collect();
        if 141_440 & eris[lower_bound] != 0 {
            lower_bound -= 1;
        }
        if 33_080_895 & eris[upper_bound] != 0 {
            upper_bound += 1;
        }
    }
    eris.into_iter()
        .map(|world| world.count_ones() as usize) 
        .sum()
}

#[test]
fn default() {
    let input = get_input(19, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(18852849, part1(&input));
    assert_eq!(1948, part2(&input));
}

// Input parsed (31μs)
// 1. 18852849 (90μs)
// 2. 1948 (1ms)
// Total: 1ms
