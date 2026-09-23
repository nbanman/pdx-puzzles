use std::collections::VecDeque;
use rustc_hash::{FxHashMap, FxHashSet};
use everybody_codes::utilities::inputs::{get_story_inputs, get_story_inputs_provisional};
use utilities::enums::cardinals::Cardinal;
use utilities::minmax::minmax;
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
            || (self.tl.y()..=self.br.y()).contains(&pos.x())
    }
}

enum SurroundCheck {
    SurroundsSpace(FxHashSet<Pos>),
    EscapesFinity,
}

type Template = FxHashSet<Pos>;

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

fn parse(input: Input) -> (Template, Finity, Pos, Pos) {
    let mut template = FxHashSet::default();
    let mut source: Option<Pos> = None;
    let mut bone: Option<Pos> = None;
    'outer: for (y, line) in input.lines().enumerate() {
        for (x, &b) in line.as_bytes().iter().enumerate() {
            let pos = Pos::new2d(x as i64, y as i64);
            match b {
                b'@' => {
                    source = Some(pos);
                    template.insert(pos);
                    if template.len() == 2 {
                        break 'outer;
                    }
                }
                b'#' => {
                    bone = Some(pos);
                    template.insert(pos);
                    if template.len() == 2 {
                        break 'outer;
                    }
                },
                _ => { },
            }
        }
    }
    let mut template_iterator = template.iter();
    let [x1, y1] = template_iterator.next().unwrap().0;
    let [x2, y2] = template_iterator.next().unwrap().0;
    let (&xmin, &xmax) = minmax(&x1, &x2);
    let (&ymin, &ymax) = minmax(&y1, &y2);
    let finity = Finity {
        tl: Pos::new2d(xmin, ymin),
        br: Pos::new2d(xmax, ymax),
    };
    (template, finity, source.unwrap(), bone.unwrap())
}

fn part1(input: Input) -> u64 {
    let (mut template, mut finity, mut source, bone) = parse(input);
    let mut steps = 0;
    for &dir in Cardinal::entries().iter().cycle() {
        let step = source.move_direction(dir, 1).unwrap();
        if step == bone {
            return steps + 1;
        }
        match template.get(&step) {
            None => {
                finity.push(step);
                template.insert(step);
                source = step;
                steps += 1
            },
            Some(_) => { },
        }
    }
    unreachable!()
}

fn part2(input: Input) -> u64 {
    let (mut template, mut finity, mut source, bone) = parse(input);
    let mut steps = 0;
    for &dir in Cardinal::entries().iter().cycle() {
        let step = source.move_direction(dir, 1).unwrap();
        match template.get(&step) {
            None => {
                steps += 1;
                source = step;
                finity.push(step);
                template.insert(step);

                println!("Step {}: Move to {}", steps, step);

                for adj in source.adjacent(false) {
                    print!("BFS {}: ", adj);
                    match template.get(&adj) {
                        Some(_) => {
                            println!("hit stream; abort");
                        },
                        _ => {
                            match bfs(adj, &template, finity) {
                                SurroundCheck::SurroundsSpace(stream_spaces) => {
                                    println!("surrounds space");
                                    for new_stream in stream_spaces.into_iter() {
                                        template.insert(new_stream);
                                    }
                                },
                                SurroundCheck::EscapesFinity => {
                                    println!("escapes to infinity");
                                },
                            }
                        },
                    }
                }
                print_finite(finity, &template, source, bone);
                if bone.adjacent(false)
                    .iter()
                    .all(|adj| template.contains(adj)) {

                    return steps;
                }
            },
            _ => {
                println!("Step {}: Attempt to move to {}, but not empty.", steps + 1, step);
            },
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
            if template.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor);
            frontier.push_back(neighbor);
        }
    }
    SurroundCheck::SurroundsSpace(visited)
}

fn print_finite(finity: Finity, template: &Template, source: Pos, bone: Pos) {
    for y in finity.tl.y()..=finity.br.y() {
        for x in finity.tl.x()..=finity.br.x() {
            let pos = Pos::new2d(x, y);
            if pos == source {
                print!("@");
                continue;
            }
            if pos == bone {
                print!("#");
                continue;
            }
            let c = if template.contains(&pos) {
                '+'
            } else {
                '.'
            };
            print!("{c}");
        }
        println!();
    }
}

fn part3(input: Input) -> u64 {

    todo!()
}

#[test]
fn default() {
    // let (input1, input2, input3) = get_story_inputs(26, 3, 2);
    // assert_eq!(225, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn example() {
    let input1 = r".......
.......
.......
.#.@...
.......
.......
.......";
    assert_eq!(12, part1(input1));
    assert_eq!(47, part2(input1));
    let input3 = r"";
    // assert_eq!(ZZ, part1(input3));
}
