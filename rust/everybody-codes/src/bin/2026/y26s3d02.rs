use everybody_codes::utilities::inputs::{get_story_inputs_provisional};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use utilities::enums::cardinals::Cardinal;
use utilities::structs::coord::Coord2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Pos = Coord2;

#[derive(Debug, Copy, Clone)]
struct Finity {
    tl: Pos,
    br: Pos,
}

impl Finity {
    fn push(&mut self, pos: Pos) -> bool {
        let mut changed = false;
        if pos.x() < self.tl.x() {
            self.tl.0[0] = pos.x();
            changed = true;
        } else if pos.x() > self.br.x() {
            self.br.0[0] = pos.x();
            changed = true;
        }
        if pos.y() < self.tl.y() {
            self.tl.0[1] = pos.y();
            changed = true;
        } else if pos.y() > self.br.y() {
            self.br.0[1] = pos.y();
            changed = true;
        }
        changed
    }

    fn check(&self, pos: Pos) -> bool {
        (self.tl.x()..=self.br.x()).contains(&pos.x())
            && (self.tl.y()..=self.br.y()).contains(&pos.y())
    }
}

enum SurroundCheck {
    SurroundsSpace(FxHashSet<Pos>),
    EscapesFinity,
}

type Template = FxHashMap<Pos, Wave>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Wave {
    Bone,
    Stream,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs_provisional(26, 3, 2);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    }
    if let Some(input2) = input2 {
        println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    }
    if let Some(input3) = input3 {
        println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input) -> (Template, Finity, Pos) {
    let mut template = FxHashMap::default();
    let mut source: Option<Pos> = None;
    for (y, line) in input.lines().enumerate() {
        for (x, &b) in line.as_bytes().iter().enumerate() {
            let pos = Pos::new2d(x as i64, y as i64);
            match b {
                b'@' => {
                    source = Some(pos);
                    template.insert(pos, Wave::Stream);
                }
                b'#' => {
                    template.insert(pos, Wave::Bone);
                }
                _ => {}
            }
        }
    }
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut y_min = i64::MAX;
    let mut y_max = i64::MIN;

    for (&pos, _) in template.iter() {
        let x = pos.x();
        if x < x_min { x_min = x; }
        if x > x_max { x_max = x; }
        let y = pos.y();
        if y < y_min { y_min = y; }
        if y > y_max { y_max = y; }
    }
    
    let finity = Finity {
        tl: Pos::new2d(x_min, y_min),
        br: Pos::new2d(x_max, y_max),
    };

    // pre-mark as streamed any spots within finity that are completely enclosed by bones
    let touches_infinity: FxHashSet<Pos> = touches_infinity(&template, &finity);

    Pos::for_rectangle(finity.tl, finity.br, |pos| {
        if !template.contains_key(&pos)
            && !touches_infinity.contains(&pos)
        {
            template.insert(pos, Wave::Stream);
        }
    });
    
    (template, finity, source.unwrap())
}

fn touches_infinity(template: &Template, finity: &Finity) -> FxHashSet<Pos> {
    let start = Pos::new2d(finity.tl.x() - 1, finity.tl.y() - 1);
    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    let mut border = FxHashSet::default();
    border.insert(start);
    let mut touches = FxHashSet::default();

    while let Some(current) = frontier.pop_front() {
        for neighbor in current.adjacent(false) {
            if finity.check(neighbor) {
                if touches.contains(&neighbor) || template.contains_key(&neighbor) {
                    continue;
                }
                touches.insert(neighbor);
                frontier.push_back(neighbor);
            } else {
                if border.contains(&neighbor) {
                    continue;
                }
                if neighbor.x() >= finity.tl.x() - 1
                    && neighbor.x() <= finity.br.x() + 1
                    && neighbor.y() >= finity.tl.y() - 1
                    && neighbor.y() <= finity.br.y() + 1
                {
                    border.insert(neighbor);
                    frontier.push_back(neighbor);
                }
            }
        }
    }
    touches
}

fn part1(input: Input) -> u64 {
    let (mut template, mut finity, mut source) = parse(input);
    let mut steps = 0;
    for &dir in Cardinal::entries().iter().cycle() {
        let step = source.move_direction(dir, 1).unwrap();
        match template.get(&step) {
            None => {
                finity.push(step);
                template.insert(step, Wave::Stream);
                source = step;
                steps += 1
            },
            Some(&Wave::Bone) => { return steps + 1 },
            Some(_) => {},
        }
    }
    unreachable!()
}

fn part2(input: Input) -> u64 {
    let (mut template, mut finity, mut source) = parse(input);
    let bone_neighbors = template.iter()
        .find(|&(_, &wave)| wave == Wave::Bone)
        .map(|(pos, _)| pos.adjacent(false))
        .unwrap();
    let mut steps = 0;
    for &dir in Cardinal::entries().iter().cycle() {
        let step = source.move_direction(dir, 1).unwrap();
        match template.get(&step) {
            None => {
                steps += 1;
                source = step;
                finity.push(step);
                template.insert(step, Wave::Stream);
                for adj in source.adjacent(false) {
                    match template.get(&adj) {
                        Some(_) => { }
                        _ => match bfs(adj, &template, finity) {
                            SurroundCheck::SurroundsSpace(stream_spaces) => {
                                for new_stream in stream_spaces.into_iter() {
                                    template.insert(new_stream, Wave::Stream);
                                }
                            }
                            SurroundCheck::EscapesFinity => {
                            }
                        },
                    }
                }
                if bone_neighbors
                    .iter()
                    .all(|adj| template.contains_key(adj))
                {
                    return steps;
                }
            },
            _ => { },
        }
    }
    unreachable!()
}

fn bfs(pos: Pos, template: &Template, finity: Finity) -> SurroundCheck {
    let mut frontier: VecDeque<Pos> = VecDeque::new();
    frontier.push_front(pos);
    let mut visited = FxHashSet::default();
    visited.insert(pos);

    while let Some(current) = frontier.pop_front() {
        for neighbor in current.adjacent(false) {
            if visited.contains(&neighbor) {
                continue;
            }
            if !finity.check(neighbor) {
                return SurroundCheck::EscapesFinity;
            }
            if template.contains_key(&neighbor) {
                continue;
            }

            visited.insert(neighbor);
            frontier.push_back(neighbor);
        }
    }
    SurroundCheck::SurroundsSpace(visited)
}

fn part3(input: Input) -> u64 {
    let (mut template, mut finity, mut source) = parse(input);

    let mut contiguous_bones: FxHashSet<Pos> = FxHashSet::default();
    let mut bone_groups: Vec<FxHashSet<Pos>> = Vec::new();
    let mut bone_surrounds: FxHashSet<Pos> = FxHashSet::default();
    
    for bone in template.iter()
        .filter(|(_, wave)| wave == &&Wave::Bone)
        .map(|(pos, _)| *pos)
    {
        if !contiguous_bones.contains(&bone) {
            let (bone_group, bone_surround) = get_bone_groups_and_surrounds(bone, &template);
            contiguous_bones.extend(bone_group.iter());
            bone_groups.push(bone_group);
            for bone in bone_surround {
                if !template.contains_key(&bone) {
                    bone_surrounds.insert(bone);
                }
            }
        }
    }

    let mut steps = 0;
    for &dir in Cardinal::entries().iter()
        .flat_map(|dir| std::iter::repeat_n(dir, 3))
        .cycle()
    {
        let step = source.move_direction(dir, 1).unwrap();
        match template.get(&step) {
            None => {
                steps += 1;
                source = step;
                finity.push(step);
                template.insert(step, Wave::Stream);

                for adj in source.adjacent(false) {
                    match template.get(&adj) {
                        Some(&Wave::Bone) => { },
                        Some(&Wave::Stream) => {
                            bone_surrounds.remove(&adj);
                        },
                        _ => match bfs(adj, &template, finity) {
                            SurroundCheck::SurroundsSpace(stream_spaces) => {
                                for new_stream in stream_spaces.into_iter() {
                                    bone_surrounds.remove(&new_stream);
                                    template.insert(new_stream, Wave::Stream);
                                }
                            },
                            SurroundCheck::EscapesFinity => { },
                        },
                    }
                }

                if bone_surrounds.is_empty() {
                    return steps;
                }
            },
            _ => { },
        }
    }
    unreachable!()
}

fn get_bone_groups_and_surrounds(bone: Pos, template: &Template) -> (FxHashSet<Pos>, FxHashSet<Pos>) {
    let mut frontier = VecDeque::new();
    frontier.push_back(bone);
    let mut bone_group = FxHashSet::default();
    bone_group.insert(bone);
    let mut surrounds = FxHashSet::default();
    while let Some(current) = frontier.pop_front() {
        for neighbor in current.adjacent(false) {
            if template.get(&neighbor) == Some(&Wave::Bone) {
                if bone_group.contains(&neighbor) {
                    continue;
                }
                bone_group.insert(neighbor);
                frontier.push_back(neighbor);
            } else {
                if surrounds.contains(&neighbor) {
                    continue;
                }
                surrounds.insert(neighbor);
            }
        }
    }
    (bone_group, surrounds)
}

#[test]
fn default() {
    let (input1, input2, input3) =
        everybody_codes::utilities::inputs::get_story_inputs(26, 3, 2);
    assert_eq!(225, part1(&input1));
    assert_eq!(3264, part2(&input2));
    assert_eq!(2388, part3(&input3));
}

#[test]
fn example() {
    let input = r".......
.......
.......
.#.@...
.......
.......
.......";
    assert_eq!(12, part1(input));
    assert_eq!(47, part2(input));
    assert_eq!(87, part3(input));
    let input2 = r"#..#.......#...
...#...........
...#...........
#######........
...#....#######
...#...@...#...
...#.......#...
...........#...
...........#...
#..........#...
##......#######";
    assert_eq!(239, part3(input2));
    let input3 = r"................................................................
.........................###.........###........................
....................##...###########.#####......#.......###.....
.........##.............############....####.............##.....
.......######..............#############.###....................
.........##................#############.###.......##...........
...............##...........########....####....................
...............................####.#######...........##........
........................##################...........####.......
....#.........#########################.....##......######......
..............#.##......##....##..##.##...............##........
..............................##....##..........##..............
........####....#################..######...................##..
........###.....###...####..###..##...##.########...............
.................####....###..##.##.##..###....##.....##........
....##...........#######.....##..##..##......#####..........#...
...........##......#########......#....##.######..........#####.
...........##........###########################....#.......#...
.........######............##################.......#...........
...........##.............#########.............................
............#.........#############....................#........
.....#...........##..####......###......##........#.............
.............##................###..........#.....#.............
..................##...........##...................##..........
..........................###.####.####.........................
................#.###########..###.############.#...............
.....#####....###...............................###.............
.....#####...#############......@......#############............
.....#########.###################################.#............
...###########..##.....###################.....##..##...........
...######...#######.##...###.........##...##...###.##...........
.....##.########........#####..###..####.......#.########.......
............#########################################...........
..............#####################################.............
...............................###..............................
................................................................";
    assert_eq!(1539, part3(input3));
}

// Input parsed (34μs)
// 1. 225 (42μs)
// 2. 3264 (25.342ms)
// 3. 2388 (22.328ms)
// Total: 47.753ms