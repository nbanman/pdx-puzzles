use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::{Coord2, Coord2U},
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;
type Pos = Coord2;
type CompressedPos = Coord2U;

struct WallData {
    hz_walls: FxHashMap<i64, Vec<i64>>,
    vt_walls: FxHashMap<i64, Vec<i64>>,
    hz_dots: Vec<i64>,
    vt_dots: Vec<i64>,
    start: CompressedPos,
    end: CompressedPos,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 15);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_wall_data(input: Input) -> WallData {
    let mut hz_walls = FxHashMap::default();
    let mut vt_walls = FxHashMap::default();
    let mut hz_dots: FxHashSet<i64> = FxHashSet::default();
    let mut vt_dots: FxHashSet<i64> = FxHashSet::default();
    let mut dir = Cardinal::North;
    let mut turtle = Pos::origin();
    let mut real_end = Pos::origin();

    let add_dots = |pos: Pos, hz_dots: &mut FxHashSet<i64>, vt_dots: &mut FxHashSet<i64>| {
        hz_dots.insert(pos.x() - 1);
        hz_dots.insert(pos.x() + 1);
        vt_dots.insert(pos.y() - 1);
        vt_dots.insert(pos.y() + 1);
    };

    let commands = input.split(',').collect_vec();
    let last_index = commands.len() - 1;
    for (idx, cmd) in commands.into_iter().enumerate() {
        let mut cmd = cmd.chars();

        // get the new direction
        dir = match cmd.next().unwrap() {
            'L' => dir.left(),
            'R' => dir.right(),
            _ => unreachable!(),
        };

        // if this is the start, move the position 1 space toward where it's going and also record dots
        if idx == 0 {
            turtle = turtle.move_direction(dir, 1).unwrap();
            add_dots(turtle, &mut hz_dots, &mut vt_dots);
        }

        // calculate how far to go
        let mut distance = cmd.fold(0, |acc, c| acc * 10 + (c as u8 - b'0') as i64);

        // if this is the first or last iteration, the distance traveled is actually one shorter. So adjust distance
        // and also record the end point.
        if idx == 0 || idx == last_index {
            real_end = turtle.move_direction(dir, distance).unwrap();
            distance -= 1;
        }

        // move the turtle and record the dots
        let next = turtle.move_direction(dir, distance).unwrap();
        add_dots(next, &mut hz_dots, &mut vt_dots);

        // create wall and add it
        match dir {
            Cardinal::North | Cardinal::South => {
                let ranges = vt_walls.entry(next.x()).or_insert_with(Vec::new);
                ranges.push(turtle.y());
                ranges.push(next.y());
            }
            Cardinal::East | Cardinal::West => {
                let ranges = hz_walls.entry(next.y()).or_insert_with(Vec::new);
                ranges.push(turtle.x());
                ranges.push(next.x());
            }
        }
        turtle = next;
    }

    // add start and end points as positions that the state can travel to
    hz_dots.insert(0);
    hz_dots.insert(real_end.x());
    vt_dots.insert(0);
    vt_dots.insert(real_end.y());

    // convert the set of dots to an ordered list
    let hz_dots: Vec<i64> = hz_dots.into_iter().sorted_unstable().collect();
    let vt_dots: Vec<i64> = vt_dots.into_iter().sorted_unstable().collect();

    let start = CompressedPos::new2d(
        hz_dots.binary_search(&0).unwrap(),
        vt_dots.binary_search(&0).unwrap(),
    );

    let end = CompressedPos::new2d(
        hz_dots.binary_search(&real_end.x()).unwrap(),
        vt_dots.binary_search(&real_end.y()).unwrap(),
    );

    for walls in hz_walls.values_mut() {
        walls.sort_unstable();
    }

    for walls in vt_walls.values_mut() {
        walls.sort_unstable();
    }

    WallData {
        hz_walls,
        vt_walls,
        hz_dots,
        vt_dots,
        start,
        end,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: CompressedPos,
    real_pos: Pos,
    weight: usize,
}

fn shortest_path(input: Input) -> usize {
    let WallData {
        hz_walls,
        vt_walls,
        hz_dots,
        vt_dots,
        start,
        end,
    } = get_wall_data(input);

    let start = State {
        pos: start,
        real_pos: Pos::origin(),
        weight: 0,
    };

    let mut visited = FxHashSet::default();
    visited.insert(start.pos);

    let mut todo = Vec::new();
    todo.push(start);

    let mut next: Vec<State> = Vec::new();

    while !todo.is_empty() {
        for State {
            pos,
            real_pos,
            weight,
        } in todo.drain(..)
        {
            if pos == end {
                return weight;
            }
            for adj_pos in Cardinal::entries()
                .into_iter()
                .filter_map(|dir| {
                    move_pos(pos, dir, &hz_walls, &vt_walls, &hz_dots, &vt_dots)
                })
            {
                if !visited.insert(adj_pos) {
                    continue;
                }
                let adj_real_pos = get_real_pos(adj_pos, &hz_dots, &vt_dots);
                let adj_weight = weight + real_pos.manhattan_distance(adj_real_pos);
                let adj_state = State {
                    pos: adj_pos,
                    real_pos: adj_real_pos,
                    weight: adj_weight,
                };
                next.push(adj_state);
            }
        }
        std::mem::swap(&mut todo, &mut next);
    }
    unreachable!()
}

fn get_real_pos(pos: CompressedPos, hz_dots: &Vec<i64>, vt_dots: &Vec<i64>) -> Pos {
    Pos::new2d(hz_dots[pos.x()], vt_dots[pos.y()])
}

fn move_pos(
    pos: CompressedPos,
    dir: Cardinal,
    hz_walls: &FxHashMap<i64, Vec<i64>>,
    vt_walls: &FxHashMap<i64, Vec<i64>>,
    hz_dots: &Vec<i64>,
    vt_dots: &Vec<i64>,
) -> Option<CompressedPos> {
    let new_x = match dir {
        Cardinal::North | Cardinal::South => pos.x(),
        Cardinal::East => {
            let new_x = pos.x() + 1;
            if new_x == hz_dots.len() {
                return None;
            }
            new_x
        }
        Cardinal::West => pos.x().checked_sub(1)?,
    };
    let new_y = match dir {
        Cardinal::East | Cardinal::West => pos.y(),
        Cardinal::South => {
            let new_y = pos.y() + 1;
            if new_y == vt_dots.len() {
                return None;
            }
            new_y
        }
        Cardinal::North => pos.y().checked_sub(1)?,
    };
    let real_pos = get_real_pos(pos, hz_dots, vt_dots);
    let one_over = real_pos.move_direction(dir, 1).unwrap();

    let no_hz_conflict = check_conflict(hz_walls, one_over.x(), one_over.y());
    let no_wall_conflict = no_hz_conflict
        && check_conflict(vt_walls, one_over.y(), one_over.x());

    if no_wall_conflict {
        Some(CompressedPos::new2d(new_x, new_y))
    } else {
        None
    }
}

fn check_conflict(walls: &FxHashMap<i64, Vec<i64>>, a: i64, b: i64) -> bool {
    walls
        .get(&b)
        .map(|walls| matches!(walls.binary_search(&a), Err(n) if n & 1 == 0))
        .unwrap_or(true)
}

fn part1(input: Input) -> usize {
    shortest_path(input)
}
fn part2(input: Input) -> usize {
    shortest_path(input)
}
fn part3(input: Input) -> usize {
    shortest_path(input)
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 15);
    assert_eq!(110, part1(&input1));
    assert_eq!(5053, part2(&input2));
    assert_eq!(454681238, part3(&input3));
}

// Input parsed (33μs)
// 1. 110 (100μs)
// 2. 5053 (1.725ms)
// 3. 454681238 (7.319ms)
// Total: 9.184ms
