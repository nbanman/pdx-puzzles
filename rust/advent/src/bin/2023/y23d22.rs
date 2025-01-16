use std::{collections::{HashMap, HashSet, VecDeque}, iter::successors};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord3, stopwatch::{ReportDuration, Stopwatch}}};

type Input = BrickHouse;
type Output = usize;
type Pos = Coord3;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Brick(Vec<Pos>);

impl Brick {
    fn new(from: Pos, to: Pos) -> Self {
        let (min_x, max_x) = [from.x(), to.x()].into_iter().minmax().into_option().unwrap();
        let (min_y, max_y) = [from.y(), to.y()].into_iter().minmax().into_option().unwrap();
        let (min_z, max_z) = [from.z(), to.z()].into_iter().minmax().into_option().unwrap();
        Self (
            (min_x..=max_x).cartesian_product(min_y..=max_y).cartesian_product(min_z..=max_z) 
                .map(|((x, y), z)| Pos::new3d(x, y, z))
                .collect()
        )
    }
}


struct BrickHouse {
    bricks: Vec<Brick>,    
    rested_on: HashMap<Brick, HashSet<Brick>>,
    resting_on: HashMap<Brick, HashSet<Brick>>,
}

impl BrickHouse {
    fn new(dimensions: Pos, bricks: &[Brick]) -> Self {
        let mut space: Vec<Option<Brick>> = 
            vec![None; ((dimensions.x() + 1) * (dimensions.y() + 1) * (dimensions.z() + 1)) as usize]; 
        let mut rested_on: HashMap<Brick, HashSet<Brick>> = HashMap::new();
        let mut resting_on: HashMap<Brick, HashSet<Brick>> = HashMap::new();
        let one_below = Pos::new3d(0, 0, 1);
        let width = dimensions.x() + 1;
        let length_width = width * (dimensions.y() + 1);
        let bricks: Vec<Brick> = bricks.iter()
            .filter_map(|brick| {
                let (placement, lower) = successors(Some(brick.0.clone()), |points| {
                    Some(points.iter().map(|&point| point - one_below).collect())
                })
                    .tuple_windows()
                    .take_while(|(upper, _)| {
                        upper.iter().all(|point| {
                            point.z() >= 1 && 
                                space[Self::get_index(point, width, length_width)].is_none()
                        })
                    })
                    .last()
                    .map(|(upper, lower)| (Brick(upper), lower))?;
                
                for below in lower.iter() {
                    if let Some(below_brick) = 
                        space.get(Self::get_index(below, width, length_width)) {
                        if let Some(below_brick) = below_brick {
                            rested_on.entry(below_brick.clone())
                                .or_insert(HashSet::new())
                                .insert(placement.clone());
                            resting_on.entry(placement.clone())
                                .or_insert(HashSet::new())
                                .insert(below_brick.clone());
                        }
                    }
                }

                for point in placement.0.iter() {
                    space[Self::get_index(point, width, length_width)] = Some(placement.clone());
                }
                Some(placement)
            })
            .collect();
        Self { bricks, rested_on, resting_on }
    }
    
    fn safe_disintegrations(&self) -> Vec<Brick> {
        self.bricks.iter()
            .filter(|&brick| {
                if let Some(rested_bricks) = self.rested_on.get(brick) {
                    rested_bricks.iter().all(|rested_brick| {
                        self.resting_on[rested_brick].len() > 1
                    })
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
    
    fn chain_reactions(&self) -> usize {
        self.bricks.iter() 
            .map(|disintegrated| {
                let mut fallen_bricks = HashSet::new();
                fallen_bricks.insert(disintegrated.clone());
                let mut q = VecDeque::new();
                q.push_back(disintegrated.clone());
                while let Some(brick) = q.pop_front() {
                    if let Some(rested_bricks) = self.rested_on.get(&brick) {
                        for rested_brick in rested_bricks {
                            let resting_bricks = &self.resting_on[rested_brick];
                            if resting_bricks.iter()
                                .all(|resting_brick| fallen_bricks.contains(resting_brick)) {
                            
                                fallen_bricks.insert(rested_brick.clone());
                                q.push_back(rested_brick.clone());
                            }
                        }
                    }
                }
                fallen_bricks.len() - 1
            })
            .sum()
    }
    
    fn get_index(pos: &Pos, width: i64, length_width: i64) -> usize {
        (pos.x() + pos.y() * width + pos.z() * length_width) as usize
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let [mut max_x, mut max_y, mut max_z] = [0; 3];
    
    let bricks: Vec<Brick> = input.get_numbers().tuples()
        .map(|(x, y, z)| {
            if x > max_x { max_x = x; }
            if y > max_y { max_y = y; }
            if z > max_z { max_z = z; }
            Pos::new3d(x, y, z)
        })
        .tuples()
        .map(|(from, to)| Brick::new(from, to))
        .sorted_by(|a, b| a.0[0].z().cmp(&b.0[0].z()))
        .collect();
    
    let dimensions = Pos::new3d(max_x, max_y, max_z);
    BrickHouse::new(dimensions, &bricks)
}

fn part1(brick_house: &Input) -> Output {
    brick_house.safe_disintegrations().len()
}

fn part2(brick_house: &Input) -> Output {
    brick_house.chain_reactions()
}

#[test]
fn default() {
    let input = get_input(23, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(446, part1(&input));
    assert_eq!(60287, part2(&input));
}

#[test]
fn example() {
    let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
    let input = parse_input(&input);
    assert_eq!(5, part1(&input));
    assert_eq!(7, part2(&input));
}

// Input parsed (6ms)
// 1. 446 (163μs)
// 2. 60287 (67ms)
// Total: 73ms