use everybody_codes::utilities::inputs::get_event_inputs;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;
type Pos = Coord2;
type Triangle = FxHashMap<Cell, Maze>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cell {
    pos: Pos,
    side: Side,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Maze {
    Trampoline,
    Empty,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 20);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_triangle(notes: Input) -> (Triangle, Option<Cell>, Option<Cell>) {
    let mut triangle = Triangle::default();
    let mut start: Option<Cell> = None;
    let mut end: Option<Cell> = None;
    for (y, line) in notes.lines().enumerate() {
        for (xx, &maze_byte) in line[y..]
            .as_bytes()
            .iter()
            .enumerate()
            .take_while(|&(_, &b)| b != b'.')
        {
            let x = xx / 2;
            let pos = Pos::from((x as i64, y as i64));
            let side = match xx & 1 {
                0 => Side::Left,
                _ => Side::Right,
            };
            let maze = match maze_byte {
                b'T' | b'S' | b'E' => Maze::Trampoline,
                b'#' => Maze::Empty,
                b => panic!("{} is invalid character!", b as char),
            };
            let cell = Cell { pos, side };
            if maze_byte == b'S' {
                start = Some(cell);
            } else if maze_byte == b'E' {
                end = Some(cell);
            }
            triangle.insert(cell, maze);
        }
    }
    (triangle, start, end)
}

fn get_right_triangle(og: &Triangle, bottom: i64, og_end: Cell) -> (Triangle, Cell) {
    let mut triangle = Triangle::default();
    let mut end: Option<Cell> = None;
    for x in 0..=bottom {
        let first_cell = Cell {
            pos: Pos::from((x, bottom - x)),
            side: Side::Left,
        };
        let maze = og.get(&first_cell).unwrap();
        let new_cell = Cell {
            pos: Pos::from((0, x)),
            side: first_cell.side,
        };
        if let Some(existing) = triangle.insert(new_cell, *maze) {
            println!("{:?} already exists at {:?}", existing, new_cell);
        };
        for (hz, y) in (0..bottom - x).rev().enumerate() {
            let hz = hz as i64;
            let right_cell = Cell {
                pos: Pos::from((x, y)),
                side: Side::Right,
            };
            let maze = og.get(&right_cell).unwrap();
            let new_cell = Cell {
                pos: Pos::from((hz, x)),
                side: Side::Right,
            };
            if let Some(existing) = triangle.insert(new_cell, *maze) {
                println!("{:?} already exists at {:?}", existing, new_cell);
            };
            if og_end == right_cell {
                end = Some(new_cell);
            }
            let left_cell = Cell { side: Side::Left, ..right_cell };
            let maze = og.get(&left_cell).unwrap();
            let new_cell = Cell {
                pos: Pos::from((hz + 1, x)),
                side: Side::Left,
            };
            if let Some(existing) = triangle.insert(new_cell, *maze) {
                println!("{:?} already exists at {:?}", existing, new_cell);
            };
            if og_end == left_cell {
                end = Some(new_cell);
            }
        }
    }
    (triangle, end.unwrap())
}

fn adjacent_trampolines(cell: &Cell, triangle: &Triangle) -> [Option<Cell>; 4] {
    let mut adjacent: [Option<Cell>; 4] = [None; 4];
    // look up
    if cell.side == Side::Left {
        let up = Cell {
            pos: cell.pos.move_direction(Cardinal::North, 1).unwrap(),
            side: Side::Right,
        };
        if trampoline_exists(&up, triangle) {
            adjacent[0] = Some(up);
        }
    }

    // look down
    if cell.side == Side::Right {
        let down = Cell {
            pos: cell.pos.move_direction(Cardinal::South, 1).unwrap(),
            side: Side::Left,
        };
        if trampoline_exists(&down, triangle) {
            adjacent[1] = Some(down);
        }
    }

    // look left
    let left = if cell.side == Side::Left {
        Cell {
            pos: cell.pos.move_direction(Cardinal::West, 1).unwrap(),
            side: Side::Right,
        }
    } else {
        Cell {
            pos: cell.pos,
            side: Side::Left,
        }
    };
    if trampoline_exists(&left, triangle) {
        adjacent[2] = Some(left);
    }

    // look right
    let right = if cell.side == Side::Right {
        Cell {
            pos: cell.pos.move_direction(Cardinal::East, 1).unwrap(),
            side: Side::Left,
        }
    } else {
        Cell {
            pos: cell.pos,
            side: Side::Right,
        }
    };
    if trampoline_exists(&right, triangle) {
        adjacent[3] = Some(right);
    }

    adjacent
}

fn trampoline_exists(cell: &Cell, triangle: &Triangle) -> bool {
    triangle
        .get(cell)
        .map(|&maze| maze == Maze::Trampoline)
        .unwrap_or_default()
}

fn part1(notes: Input) -> usize {
    let (triangle, _, _) = get_triangle(notes);
    triangle
        .iter()
        .filter(|&(cell, &maze)| maze == Maze::Trampoline && cell.side == Side::Right)
        .map(|(cell, _)| {
            adjacent_trampolines(&cell, &triangle)
                .iter()
                .filter(|cell| cell.is_some())
                .count()
        })
        .sum()
}

fn part2(notes: Input) -> usize {
    let (triangle, Some(start), Some(end)) = get_triangle(notes) else {
        panic!("No start or end found for part2!");
    };

    let mut todo: Vec<Cell> = vec![start];
    let mut next: Vec<Cell> = Vec::new();
    let mut visited: FxHashSet<Cell> = FxHashSet::default();
    visited.insert(start);
    let mut jumps = 0;

    while !todo.is_empty() {
        for cell in todo.drain(..) {
            if cell == end {
                return jumps;
            }
            for adj in adjacent_trampolines(&cell, &triangle) {
                let Some(adj) = adj else {
                    continue;
                };
                if visited.insert(adj) {
                    next.push(adj);
                }
            }
        }
        jumps += 1;
        std::mem::swap(&mut todo, &mut next);
    }
    unreachable!()
}

fn part3(notes: Input) -> usize {
    let (triangle, Some(start), Some(end)) = get_triangle(notes) else {
        panic!("No start or end found for part2!");
    };

    // println!("Triangle 1:\n{}", triangle_to_string(&triangle, start.pos.y()));
    let (triangle2, end2) = get_right_triangle(&triangle, start.pos.y(), end);
    // println!("Triangle 2:\n{}", triangle_to_string(&triangle2, start.pos.y()));
    let (triangle3, end3) = get_right_triangle(&triangle2, start.pos.y(), end2);
    // println!("Triangle 3:\n{}", triangle_to_string(&triangle3, start.pos.y()));
    let triangle = [triangle, triangle2, triangle3];
    let ends = [end, end2, end3];
    let mut todo: Vec<Cell> = vec![start];
    let mut next: Vec<Cell> = Vec::new();
    let mut visited: FxHashSet<(Cell, usize)> = FxHashSet::default();
    visited.insert((start, 0));
    let mut jumps = 0;

    while !todo.is_empty() {
        for cell in todo.drain(..) {
            if cell == ends[jumps % 3] {
                return jumps;
            }
            let next_triangle = &triangle[(jumps + 1) % 3];
            if next_triangle.get(&cell) == Some(&Maze::Trampoline) {
                if visited.insert((cell, (jumps + 1) % 3)) {
                    next.push(cell);
                }
            }
            for adj in adjacent_trampolines(&cell, next_triangle) {
                let Some(adj) = adj else {
                    continue;
                };
                if visited.insert((adj, (jumps + 1) % 3)) {
                    next.push(adj);
                }
            }
        }
        jumps += 1;
        std::mem::swap(&mut todo, &mut next);
    }
    unreachable!()
}

#[allow(dead_code)]
fn triangle_to_string(triangle: &Triangle, bottom: i64) -> String {
    let mut rep = String::new();
    for (i, y) in (0..=bottom).enumerate() {
        for _ in 0..i {
            rep.push('.');
        }
        for x in 0..=bottom - (i as i64 / 2) {
            let cell = Cell {
                pos: Pos::from((x, y)),
                side: Side::Left,
            };
            let c = match triangle.get(&cell) {
                Some(maze) => {
                    match maze {
                        Maze::Trampoline => 'T',
                        Maze::Empty => '#',
                    }
                }
                None => '.',
            };
            rep.push(c);
            let cell = Cell { side: Side::Right, ..cell };
            let c = match triangle.get(&cell) {
                Some(maze) => {
                    match maze {
                        Maze::Trampoline => 'T',
                        Maze::Empty => '#',
                    }
                }
                None => '.',
            };
            rep.push(c);
        }
        rep.push('\n');
    }
    rep.pop();
    rep
}

#[test]
fn test1() {
    let notes = r"T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....";
    assert_eq!(7, part1(notes));
}

#[test]
fn test2() {
    let notes = r"TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........";
    assert_eq!(32, part2(notes));
}

#[test]
fn test3() {
    let notes = r"T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S.........";
    assert_eq!(23, part3(notes));
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 20);
    assert_eq!(125, part1(&input1));
    assert_eq!(573, part2(&input2));
    assert_eq!(470, part3(&input3));
}

// Input parsed (42μs)
// 1. 125 (16μs)
// 2. 573 (694μs)
// 3. 470 (1.235ms)
// Total: 1.993ms
