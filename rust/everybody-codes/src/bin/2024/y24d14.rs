use everybody_codes::utilities::inputs::get_event_inputs;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{HashSet, VecDeque},
    sync::LazyLock,
};
use utilities::structs::{
    coord::Coord3,
    stopwatch::{ReportDuration, Stopwatch},
};

static MOVES: LazyLock<[Coord3; 6]> = LazyLock::new(|| {
    [
        Coord3::new3d(1, 0, 0),
        Coord3::new3d(-1, 0, 0),
        Coord3::new3d(0, 1, 0),
        Coord3::new3d(0, -1, 0),
        Coord3::new3d(0, 0, 1),
        Coord3::new3d(0, 0, -1),
    ]
});
fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 14);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: &str) -> usize {
    grow_branch(input)
        .max_by(|a, b| a.y().cmp(&b.y()))
        .unwrap()
        .y() as usize
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| grow_branch(line))
        .collect::<HashSet<Coord3>>()
        .len()
}

fn part3(input: &str) -> usize {
    let branches: Vec<Vec<Coord3>> = input
        .lines()
        .map(|line| grow_branch(line).collect())
        .collect();
    let leaves: Vec<Coord3> = branches
        .iter()
        .map(|branch| *branch.last().unwrap())
        .collect();
    let tree: HashSet<Coord3> = branches
        .into_iter()
        .flat_map(|branch| branch.into_iter())
        .collect();
    let height = tree.iter().map(|segment| segment.y()).max().unwrap() as usize;

    let leaf_to_trunk: Vec<Vec<usize>> = leaves
        .par_iter()
        .map(|leaf| {
            let mut taps = 0;
            let mut tap_distance = vec![0; height];
            let mut q = VecDeque::new();
            q.push_back((*leaf, 0));
            let mut visited = HashSet::new();
            while let Some((pos, weight)) = q.pop_front() {
                if !visited.insert(pos) {
                    continue;
                }
                if pos.x() == 0 && pos.z() == 0 {
                    tap_distance[(pos.y() - 1) as usize] = weight;
                    taps += 1;
                    if taps == height {
                        break;
                    }
                }
                MOVES
                    .iter()
                    .map(|dir| *dir + pos)
                    .filter(|n_pos| !visited.contains(n_pos) && tree.contains(n_pos))
                    .map(|n_pos| (n_pos, weight + 1))
                    .for_each(|state| q.push_back(state))
            }
            tap_distance
        })
        .collect();

    let mut tap_points = vec![vec![0; leaves.len()]; height];

    for (leaf_index, trunk_counts) in leaf_to_trunk.iter().enumerate() {
        for (trunk_index, &count) in trunk_counts.iter().enumerate() {
            tap_points[trunk_index][leaf_index] = count;
        }
    }

    tap_points
        .iter()
        .map(|distances| distances.iter().sum::<usize>())
        .filter(|&murkiness| murkiness != 0)
        .min()
        .unwrap()
}

fn grow_branch<'a>(input: &'a str) -> impl Iterator<Item = Coord3> + 'a {
    input
        .split(',')
        .flat_map(|instruction| {
            let (dir, distance) = instruction.split_at(1);
            let dir = dir.as_bytes()[0] as char;
            let distance: usize = distance.parse().unwrap();
            vec![dir; distance].into_iter()
        })
        .scan(Coord3::origin(), |pos, dir| {
            *pos += match dir {
                'U' => Coord3::new3d(0, 1, 0),
                'D' => Coord3::new3d(0, -1, 0),
                'L' => Coord3::new3d(-1, 0, 0),
                'R' => Coord3::new3d(1, 0, 0),
                'F' => Coord3::new3d(0, 0, 1),
                'B' => Coord3::new3d(0, 0, -1),
                other => {
                    panic!("Unrecognized direction: {other}.");
                }
            };
            Some(*pos)
        })
}

#[test]
fn tests() {
    let tests = [
        "U5,R3,D2,L5,U4,R5,D2",
        "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1",
        "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1",
        "U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1",
    ];
    assert_eq!(7, part1(tests[0]));
    assert_eq!(32, part2(tests[1]));
    assert_eq!(5, part3(tests[2]));
    assert_eq!(46, part3(tests[3]));
}
