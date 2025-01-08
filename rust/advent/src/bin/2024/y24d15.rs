use std::iter::successors;

use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}}};

type Warehouse = FxHashMap<Pos, Entity>;
type Input = (Pos, Warehouse, Vec<Cardinal>, Pos);
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug)]
enum Entity {
    Wall,
    Box,
    LeftBox,
    RightBox,
}

fn parse_input(input: &str) -> Input {
    let (warehouse, directions) = input.split_once("\n\n").unwrap();
    let directions = directions.as_bytes().iter()
        .filter_map(|&it| {
            match it {
                b'^' => Some(Cardinal::North),
                b'>' => Some(Cardinal::East),
                b'v' => Some(Cardinal::South),
                b'<' => Some(Cardinal::West),
                b'\n' => None,
                x => { panic!("Movement '{}' not recognized", x); }
            }
        })
        .collect();
    let width = warehouse.find('\n').unwrap();
    let height = (warehouse.len() + 1) / (width + 1);
    let bounds = Pos::new2d(width, height);
    
    let robot = warehouse.find('@').unwrap();
    let robot = Pos::from_index(robot, width + 1).unwrap();
    
    let warehouse: Warehouse = warehouse.as_bytes().iter().enumerate()
        .filter_map(|(idx, &c)| {    
            match c {
                b'#' => Some(Entity::Wall),
                b'O' => Some(Entity::Box),
                _ => None,
            }
            .map(|entity| {
                let pos = Pos::new2d(idx % (width + 1), idx / (width + 1));
                (pos, entity)
            })
        })
        .collect();
    (bounds, warehouse, directions, robot)
}

fn part1(input: Input) -> Output {
    let (bounds, mut warehouse, directions, mut robot) = input;
    for dir in directions {
        let Some(empty) = find_empty(robot, dir, &warehouse, bounds) else { continue; };
        robot = robot.move_direction(dir, 1).unwrap();
        if warehouse.contains_key(&robot) {
            warehouse.remove(&robot);
            warehouse.insert(empty, Entity::Box);
        }
    }
    warehouse.into_iter()
        .filter(|(_, entity)| matches!(*entity, Entity::Box))
        .map(|(pos, _)| gps(pos))
        .sum()
}

fn find_empty(robot: Pos, dir: Cardinal, warehouse: &Warehouse, bounds: Pos) -> Option<Pos> {
    successors(robot.move_direction(dir, 1), |it| it.move_direction(dir, 1))
        .take_while(|pos| {
            (1..bounds.x() - 1).contains(&pos.x()) 
                && (1.. bounds.y() - 1).contains(&pos.y())
                && !matches!(warehouse.get(pos), Some(Entity::Wall))
        })
        .find(|pos| !warehouse.contains_key(pos))
}

fn gps(pos: Pos) -> Output { pos.y() * 100 + pos.x() }

fn part2(input: Input) -> Output {
    let (_, warehouse, directions, robot) = input;
    let mut robot = Pos::new2d(robot.x() * 2, robot.y());
    let mut  warehouse: Warehouse = warehouse.iter()
        .flat_map(|(old_pos, old_entity)| {
            let pos = Pos::new2d(old_pos.x() * 2, old_pos.y());
            let east = pos.move_direction(Cardinal::East, 1).unwrap();
            if matches!(*old_entity, Entity::Box) {
                [(pos, Entity::LeftBox), (east, Entity::RightBox)]
            } else {
                [(pos, Entity::Wall), (east, Entity::Wall)]
            }
        })
        .collect();

    // print_maze(robot, &warehouse, bounds.x(), bounds.y());

    for dir in directions {
        if matches!(dir, Cardinal::North) || matches!(dir, Cardinal::South) {
            if check_vt(robot, dir, &warehouse, true) {
                push_box_vt(robot, dir, &mut warehouse, true);
                robot = robot.move_direction(dir, 1).unwrap();
            }
        } else if push_hz(robot, dir, 1, &mut warehouse, true) {
            robot = robot.move_direction(dir, 1).unwrap();
        }
        // println!("{:?}", dir);
        // print_maze(robot, &warehouse, bounds.x(), bounds.y());
    }

    warehouse.into_iter()
        .filter(|(_, entity)| matches!(*entity, Entity::LeftBox))
        .map(|(pos, _)| gps(pos))
        .sum()
}

fn check_vt(pos: Pos, dir: Cardinal, warehouse: &Warehouse, is_robot: bool) -> bool {
    let next = pos.move_direction(dir, 1).unwrap();
    let next_right = next.move_direction(Cardinal::East, 1).unwrap();
    match warehouse.get(&next) {
        None => {
            if is_robot {
                true
            } else {
                match warehouse.get(&next_right) {
                    None => true,
                    Some(Entity::Wall) => false,
                    Some(Entity::LeftBox) => check_vt(next_right, dir, warehouse, false),
                    Some(_) => { panic!("Should not be right box."); },
                }
            }
        },
        Some(Entity::Wall) => false,
        Some(Entity::LeftBox) => check_vt(next, dir, warehouse, false),
        Some(Entity::RightBox) => {
            let next_left = next.move_direction(Cardinal::West, 1).unwrap();
            if is_robot {
                check_vt(next_left, dir, warehouse, false)
            } else {
                match warehouse.get(&next_right) {
                    Some(Entity::Wall) => false,
                    Some(Entity::LeftBox) => {
                        check_vt(next_left, dir, warehouse, false) 
                            && check_vt(next_right, dir, warehouse, false)
                    },
                    None => check_vt(next_left, dir, warehouse, false),
                    _ => { panic!("Should not be right box"); }
                }
            }
        }
        Some(_) => { panic!("Normal boxes should not be in map."); },
    }
}

fn push_box_vt(pos: Pos, dir: Cardinal, warehouse: &mut Warehouse, is_robot: bool) {
    let next = pos.move_direction(dir, 1).unwrap();
    let next_right = next.move_direction(Cardinal::East, 1).unwrap();

    // if is_robot && pos == Pos::new2d(12, 2) && dir == Cardinal::South {
    //     println!("match! {:?}", pos);
    // }

    match warehouse.get(&next) {
        Some(Entity::LeftBox) => push_box_vt(next, dir, warehouse, false),
        Some(Entity::RightBox) => {
            let next_left = next.move_direction(Cardinal::West, 1).unwrap();
            if is_robot {
                push_box_vt(next_left, dir, warehouse, false);
            } else {
                match warehouse.get(&next_right) {
                    Some(Entity::LeftBox) => {
                        push_box_vt(next_left, dir, warehouse, false);
                        push_box_vt(next_right, dir, warehouse, false);
                    },
                    None => push_box_vt(next_left, dir, warehouse, false),
                    Some(right) => { panic!("{:?} should be empty or leftBox", right); },
                }
            }
        },
        None => {
            if is_robot { return; }
            match warehouse.get(&next_right) {
                Some(Entity::LeftBox) => push_box_vt(next_right, dir, warehouse, false),
                None => {},
                Some(right) => { panic!("{:?} should be empty or left box.", right); },
            }
        },
        Some(_) => { panic!("There cannot be a wall here.") },
    }
    
    if !is_robot {
        warehouse.remove(&pos);
        warehouse.remove(&pos.move_direction(Cardinal::East, 1).unwrap());
        warehouse.insert(next, Entity::LeftBox);
        warehouse.insert(next_right, Entity::RightBox);
    }
}

fn push_hz(pos: Pos, dir: Cardinal, dist: usize, warehouse: &mut Warehouse, is_robot: bool) -> bool {
    let next = pos.move_direction(dir, dist).unwrap();
    match warehouse.get(&next) {
        Some(Entity::Wall) => false,
        Some(Entity::Box) => { panic!("Normal boxes should not be in map."); },
        None => {
            if warehouse.contains_key(&pos) { push_box_hz(pos, warehouse, dir); }
            true
        },
        _ => {
            let moveable = push_hz(next, dir, 2, warehouse, false);
            if moveable && !is_robot { push_box_hz(pos, warehouse, dir) }
            moveable
        },
    }
}

fn push_box_hz(pos: Pos, warehouse: &mut Warehouse, dir: Cardinal) {
    let next = pos.move_direction(dir, 1).unwrap();
    let next_next = next.move_direction(dir, 1).unwrap();
    let next_entity = if matches!(dir, Cardinal::East) {
        Entity::LeftBox
    } else {
        Entity::RightBox
    };
    let next_next_entity = if matches!(dir, Cardinal::East) {
        Entity::RightBox
    } else {
        Entity::LeftBox
    };
    warehouse.insert(next, next_entity);
    warehouse.insert(next_next, next_next_entity);
    warehouse.remove(&pos);
}

#[allow(dead_code)]
fn print_maze(robot: Pos, warehouse: &Warehouse, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width * 2 {
            let pos = Pos::new2d(x, y);
            if pos == robot {
                print!("@");
            } else {
                let x: char = match warehouse.get(&pos) {
                    Some(Entity::Box) => 'O',
                    Some(Entity::LeftBox) => '[',
                    Some(Entity::RightBox) => ']',
                    Some(Entity::Wall) => '#',
                    None => '.',
                };
                print!("{x}");
            }
        }
        println!();
    }
}

#[test]
fn default() {
    let input = get_input(24, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(1552463, part1(input.clone()));
    assert_eq!(1554058, part2(input));
}

#[test]
fn examples() {
    let inputs: Vec<_> = [r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^", r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"]
        .iter()
        .map(|&input| parse_input(input))
        .collect();
    assert_eq!(618, part2(inputs[0].clone()));
    assert_eq!(9021, part2(inputs[1].clone()));
}

// Input parsed (243μs)
// 1. 1552463 (612μs)
// 2. 1554058 (730μs)
// Total: 1ms
