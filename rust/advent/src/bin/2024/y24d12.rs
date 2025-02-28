use std::collections::HashSet;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch},
        str_grid::StrGrid,
    },
};

type Input<'a> = &'a str;
type Output = usize;
type Pos = Coord2U;

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
    surveyed: FxHashSet<Pos>,
}

impl Region {
    pub fn new(arrangement: &StrGrid, pos: Pos, plant: u8) -> Self {
        // setting up DFS
        let mut q: Vec<Pos> = Vec::new();
        q.push(pos);
        let mut surveyed = FxHashSet::default();
        surveyed.insert(pos);
        // fence factors are counted block by block in the DFS loop, so use mutable variables
        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = 0;

        let corners = vec![
            Cardinal::North,
            Cardinal::East,
            Cardinal::South,
            Cardinal::West,
            Cardinal::North,
        ];

        // DFS loop
        while let Some(current) = q.pop() {
            let adjacent: FxHashMap<Cardinal, (Pos, bool)> = arrangement
                .adjacent(current)
                .map(|adjacent| {
                    let (a_pos, a_dir, a_plant) = adjacent.destruct();
                    (a_dir, (a_pos, a_plant == plant))
                })
                .collect();

            // update cost factors
            area += 1;
            let in_region: Vec<Pos> = adjacent
                .values()
                .filter_map(
                    |(n_pos, in_region)| {
                        if *in_region { Some(*n_pos) } else { None }
                    },
                )
                .collect();
            perimeter += 4 - in_region.len();
            let corners = get_corners(&corners, &adjacent, current, arrangement, plant);
            sides += corners;

            // add those plots that haven't already been examined to the queue
            in_region
                .into_iter()
                .filter(|adjacent| surveyed.insert(*adjacent))
                .for_each(|adjacent| q.push(adjacent));
        }
        Region {
            area,
            perimeter,
            sides,
            surveyed,
        }
    }
}

fn get_corners(
    corners: &[Cardinal],
    adjacent: &FxHashMap<Cardinal, (Pos, bool)>,
    current: Pos,
    arrangement: &StrGrid<'_>,
    plant: u8,
) -> usize {
    let corners = corners
        .iter()
        .tuple_windows()
        .filter(|&(a, b)| {
            let a_in_region = adjacent
                .get(a)
                .map_or_else(|| false, |(_, in_region)| *in_region);
            let b_in_region = adjacent
                .get(b)
                .map_or_else(|| false, |(_, in_region)| *in_region);
            if !a_in_region && !b_in_region {
                true
            } else {
                let out_region = if let Some(diag_start) = current.move_direction(*a, 1) {
                    if let Some(diag) = diag_start.move_direction(*b, 1) {
                        if let Some(diag_plant) = arrangement.get(diag) {
                            diag_plant != plant
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } else {
                    true
                };
                a_in_region && b_in_region && out_region
            }
        })
        .count();
    corners
}

fn get_regions(input: &str) -> Vec<Region> {
    let arrangement = StrGrid::new(input).unwrap();
    let mut surveyed: HashSet<Pos, rustc_hash::FxBuildHasher> = FxHashSet::default();
    let mut regions = Vec::new();
    for idx in 0..input.len() {
        if let Some(plant) = arrangement.get(idx) {
            let pos = arrangement.idx_to_coord(&idx);
            if surveyed.contains(&pos) {
                continue;
            }
            let region = Region::new(&arrangement, pos, plant);
            for plot in region.surveyed.iter() {
                surveyed.insert(*plot);
            }
            regions.push(region);
        } else {
            continue;
        }
    }
    regions
}

fn part1(input: Input) -> Output {
    get_regions(input)
        .into_iter()
        .map(|region| region.area * region.perimeter)
        .sum()
}

fn part2(input: Input) -> Output {
    get_regions(input)
        .into_iter()
        .map(|region| region.area * region.sides)
        .sum()
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 12).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[test]
fn default() {
    let input = get_input(24, 12).unwrap();
    assert_eq!(1424472, part1(&input));
    assert_eq!(870202, part2(&input));
}

#[test]
fn examples() {
    let inputs = [
        r"AAAA
BBCD
BBCC
EEEC",
        r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
    ];
    assert_eq!(140, part1(inputs[0]));
    assert_eq!(80, part2(inputs[0]));
}
