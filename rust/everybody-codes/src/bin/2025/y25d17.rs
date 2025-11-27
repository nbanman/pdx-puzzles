use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::grid::{Grid2, GridAdjacent, GridIterator};
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch},
        str_grid::StrGrid,
    },
};

type Input<'a> = &'a str;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 17);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

trait InRange {
    fn in_range(&self, other: &Self, radius: usize) -> bool;
}

impl InRange for Pos {
    fn in_range(&self, other: &Self, radius: usize) -> bool {
        let x = self.x().abs_diff(other.x());
        let y = self.y().abs_diff(other.y());
        let p = x * x + y * y;
        p <= radius * radius
    }
}

fn part1(notes: Input) -> usize {
    let volcano: StrGrid = notes.into();
    let center = volcano.s.iter().position(|&b| b == b'@').unwrap();
    let center = Pos::from_index(center, volcano.width).unwrap();
    volcano
        .s
        .iter()
        .enumerate()
        .filter(|&(idx, &b)| {
            b != b'@'
                && Pos::from_index(idx, volcano.width)
                    .unwrap()
                    .in_range(&center, 10)
        })
        .map(|(idx, &b)| (b - b'0') as usize)
        .sum()
}

fn part2(notes: Input) -> usize {
    let volcano: StrGrid = notes.into();
    let center = volcano.s.iter().position(|&b| b == b'@').unwrap();
    let center = Pos::from_index(center, volcano.width).unwrap();
    let r2: Vec<_> = (0..center.x()).map(|it| it * it).collect();
    let mut destruction = vec![0usize; r2.len()];
    for (idx, &b) in volcano.s.iter().enumerate() {
        if !b.is_ascii_digit() {
            continue;
        }
        let pos = Pos::from_index(idx, volcano.width).unwrap();
        let x = center.x().abs_diff(pos.x());
        let y = center.y().abs_diff(pos.y());
        let p = x * x + y * y;
        let d = (b - b'0') as usize;
        let r = r2.binary_search(&p).unwrap_or_else(|r| r);
        if r < destruction.len() {
            destruction[r] += d;
        }
    }

    destruction
        .iter()
        .enumerate()
        .max_by_key(|&(_, &destruction)| destruction)
        .map(|(radius, &destruction)| radius * destruction)
        .unwrap()
}

fn part3(notes: Input) -> usize {
//     let notes = r"5441525241225111112253553251553
// 133522122534119S911411222155114
// 3445445533355599933443455544333
// 3345333555434334535435433335533
// 5353333345335554434535533555354
// 3533533435355443543433453355553
// 3553353435335554334453355435433
// 5435355533533355533535335345335
// 4353545353545354555534334453353
// 4454543553533544443353355553453
// 5334554534533355333355543533454
// 4433333345445354553533554555533
// 5554454343455334355445533453453
// 4435554534445553335434455334353
// 3533435453433535345355533545555
// 534433533533535@353533355553345
// 4453545555435334544453344455554
// 4353333535535354535353353535355
// 4345444453554554535355345343354
// 3534544535533355333333445433555
// 3535333335335334333534553543535
// 5433355333553344355555344553435
// 5355535355535334555435534555344
// 3355433335553553535334544544333
// 3554333535553335343555345553535
// 3554433545353554334554345343343
// 5533353435533535333355343333555
// 5355555353355553535354333535355
// 4344534353535455333455353335333
// 5444333535533453535335454535553
// 3534343355355355553543545553345";
    let volcano: StrGrid = notes.into();
    let center = volcano.s.iter().position(|&b| b == b'@').unwrap();
    let center = Pos::from_index(center, volcano.width).unwrap();
    let start = volcano.s.iter().position(|&b| b == b'S').unwrap();
    let start = Pos::from_index(start, volcano.width).unwrap();
    let r2: Vec<_> = (0..center.x()).map(|it| it * it).collect();
    let volcano = notes
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b != b'\n')
        .map(|(idx, &b)| {
            let pos = Pos::from_index(idx, volcano.width).unwrap();
            let x = center.x().abs_diff(pos.x());
            let y = center.y().abs_diff(pos.y());
            let p = x * x + y * y;
            let seconds = if b == b'S' {
                0
            } else {
                (b - b'0') as usize
            };
            let r = r2.binary_search(&p).unwrap_or_else(|r| r);
            (r, seconds)
        })
        .try_collect_grid(volcano.width - 1)
        .unwrap();

    for radius in 10..(volcano.width() - 1) / 2 - 1 {
        if let Some(answer) = a_star(&volcano, start, center, radius) {
            return answer;
        }
    }
    unreachable!()
}

fn a_star(
    volcano: &Grid2<(usize, usize)>,
    start: Pos,
    center: Pos,
    radius: usize,
) -> Option<usize> {
    let heuristic = |pos: Pos, phase: Cardinal| {
        let mut pos = pos;
        let mut h = 0;
        if phase == Cardinal::East {
            let target = center.move_direction(Cardinal::East, radius + 1).unwrap();
            h += pos.manhattan_distance(target);
            pos = target;
        }
        if phase == Cardinal::East || phase == Cardinal::South {
            let target = center.move_direction(Cardinal::South, radius + 1).unwrap();
            h += pos.manhattan_distance(target);
            pos = target;
        }
        if phase != Cardinal::North {
            let target = center.move_direction(Cardinal::West, radius + 1).unwrap();
            h += pos.manhattan_distance(target);
            pos = target;
        }
        h + pos.manhattan_distance(start)
    };

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    let initial_state = State {
        pos: start,
        seconds: 0,
        phase: Cardinal::East,
        f: heuristic(start, Cardinal::East),
    };
    open.push(initial_state);
    let mut closed: FxHashSet<(Pos, Cardinal)> = FxHashSet::default();

    while let Some(State {
        pos,
        seconds,
        phase,
        f: f,
    }) = open.pop()
    {
        // if pos == Pos::from((6, 0)) && phase == Cardinal::North {
        //     println!("observe {:?} carefully", pos);
        // }
        if !closed.insert((pos, phase)) {
            continue;
        }
        // println!("{:?}, phase: {}, seconds: {}, f: {}", pos, phase, seconds, f);
        // match phase {
        //     Cardinal::East => {},
        //     Cardinal::South => {
        //         if pos.y() == center.y() {
        //             println!("phase change! {}", phase)
        //         }
        //     }
        //     Cardinal::West => {
        //         if pos.x() == center.x() {
        //             println!("phase change! {}", phase)
        //         }
        //     }
        //     Cardinal::North => {
        //         if pos.y() == center.y() {
        //             println!("phase change! {}", phase)
        //         }
        //     }
        // }
        if pos == start && phase == Cardinal::North {
            return Some(seconds * radius);
        }
        for GridAdjacent {
            index: _,
            pos: adj_pos,
            dir: _,
            value: &(adj_rad, adj_sec),
        } in volcano.adjacent(pos, false).unwrap()
        {
            // abort case 1: ventured into burn area
            if adj_rad <= radius {
                continue;
            }

            // ac2: backtracking once quadrant checkpoint reached
            if match phase {
                Cardinal::North => adj_pos.y() > center.y(),
                Cardinal::East => adj_pos.x() < center.x(),
                Cardinal::South => adj_pos.y() < center.y(),
                Cardinal::West => adj_pos.x() > center.x(),
            } {
                continue;
            }

            let adj_phase = match phase {
                Cardinal::East => if adj_pos.y() == center.y() {
                    Cardinal::South
                } else {
                    phase
                },
                Cardinal::South => {
                    if adj_pos.x() == center.x() {
                        Cardinal::West
                    } else {
                        phase
                    }
                },
                Cardinal::West => {
                    if adj_pos.y() == center.y() {
                        Cardinal::North
                    } else {
                        phase
                    }
                },
                Cardinal::North => phase,
            };

            // ac3: already visited
            if closed.contains(&(adj_pos, adj_phase)) {
                continue;
            }
            let adj_sec = seconds + adj_sec;
            let adj_f = adj_sec + heuristic(adj_pos, adj_phase);

            // ac4: ran out of time
            if adj_f > (radius + 1) * 30 {
                continue;
            }

            let adj_state = State {
                pos: adj_pos,
                seconds: adj_sec,
                phase: adj_phase,
                f: adj_f,
            };
            // println!("-> {:?}", adj_state);
            open.push(adj_state);
        }
    }
    None
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct State {
    pos: Pos,
    seconds: usize,
    phase: Cardinal,
    f: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f).then(other.seconds.cmp(&self.seconds))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn default() {
    // let (input1, input2, input3) = get_event_inputs(25, 17);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}
