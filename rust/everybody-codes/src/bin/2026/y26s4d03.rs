use everybody_codes::utilities::inputs::get_story_inputs_provisional;
use itertools::Itertools;
use utilities::structs::{coord::Coord2U, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Pattern;
type Pos = Coord2U;
type Tiles = Grid2<Walls>;

struct Pattern {
    width: usize,
    height: usize,
    hz: Vec<u8>,
    vt: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Walls {
    None,
    Down,
    Right,
    Both,
}

impl Walls {
    fn has_down(&self) -> bool {
        match self {
            Walls::None => false,
            Walls::Down => true,
            Walls::Right => false,
            Walls::Both => true,
        }
    }
    fn has_right(&self) -> bool {
        match self {
            Walls::None => false,
            Walls::Down => false,
            Walls::Right => true,
            Walls::Both => true,
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs_provisional(26, 4, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!(
            "1. {} ({})",
            part1(parse(&input1)),
            stopwatch.lap().report()
        );
    }
    if let Some(input2) = input2 {
        println!(
            "2. {} ({})",
            part2(parse(&input2)),
            stopwatch.lap().report()
        );
    }
    if let Some(input3) = input3 {
        println!(
            "3. {} ({})",
            part3(parse(&input3)),
            stopwatch.lap().report()
        );
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: &str) -> Input {
    let (width, height, hz, vt) = input
        .lines()
        .map(|line| line.rsplit_once('=').unwrap().1)
        .collect_tuple()
        .unwrap();
    let width: usize = width.parse().unwrap();
    let height: usize = height.parse().unwrap();
    let hz = hz.chars().map(|c| (c as u8) - 48).collect();
    let vt = vt.chars().map(|c| (c as u8) - 48).collect();
    Input {
        width,
        height,
        hz,
        vt,
    }
}

fn part1(input: Input) -> u64 {
    let Pattern {
        height,
        width,
        hz,
        vt,
    } = input;
    let mut isolated = 0;
    let vt_walls: Vec<_> = (0..=width)
        .map(|y| vt[y % vt.len()])
        .tuple_windows()
        .enumerate()
        .filter_map(|(index, (left, right))| {
            if left != right {
                None
            } else {
                Some((index, left))
            }
        })
        .collect();
    let hz_walls: Vec<_> = (0..=height)
        .map(|x| hz[x % hz.len()])
        .tuple_windows()
        .enumerate()
        .filter_map(|(index, (top, bottom))| {
            if top != bottom {
                None
            } else {
                Some((index, top))
            }
        })
        .collect();
        
    for (x, y_offset) in vt_walls {
        for &(y, x_offset) in hz_walls.iter() {
            if (y & 1) as u8 == y_offset && (x & 1) as u8 == x_offset {
                isolated += 1;
            }
        }
    }
    isolated
}

fn part2(input: Input) -> u64 {
    let Pattern {
        height,
        width,
        hz,
        vt,
    } = input;

    let mut tiles = Tiles::new2d(vec![Walls::None; width * height], width).unwrap();

    for (x, &y_offset) in vt.iter().cycle().enumerate().take(width) {
        for y in 0..height {
            if (y & 1) as u8 == y_offset {
                tiles.data[y * width + x] = Walls::Right;
            }
        }
    }
    for (y, &x_offset) in hz.iter().cycle().enumerate().take(height) {
        for x in 0..width {
            if (x & 1) as u8 == x_offset {
                let pos = tiles.data.get_mut(y * width + x).unwrap();
                *pos = match pos {
                    Walls::None => Walls::Down,
                    Walls::Right => Walls::Both,
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut count = [0u64; 2];
    for y in 0..height {
        let mut toggle = if y == 0 {
            true
        } else {
            !tiles.data[(y - 1) * width].has_down()
        };
        let up = if y == 0 {
            height - 1
        } else {
            y - 1
        };
        
        for x in 0..width {
            let left = if x == 0 {
                width - 1
            } else {
                x - 1
            };
            print!("({x}, {y}): ");
            if tiles.data[up * width + x].has_down() {
                print!("Up ");
            }
            if tiles.data[y * width + x].has_right() {
                print!("Right ");
            }
            if tiles.data[y * width + x].has_down() {
                print!("Down ");
            }
            if tiles.data[y * width + left].has_right() {
                print!("Left ");
            }
            println!();
            
            if tiles.data[y * width + x] == Walls::Both
                && tiles.data[up * width + x].has_down()
                && tiles.data[y * width + left].has_right()
            {
                if toggle {
                    count[0] += 1;
                } else {
                    count[1] += 1;
                }
            }

            if tiles.data[y * width + x].has_right() {
                toggle = !toggle;
            }
        }
    }
    *count.iter().max().unwrap()
}

fn part3(input: Input) -> u64 {
    todo!()
}

#[test]
fn default() {
    // let (input1, input2, input3) =
    //        everybody_codes::utilities::inputs::get_story_inputs(26, XX, XX);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn example() {
    let input1 = r"width=30
height=10
horizontal-offsets=10011
vertical-offsets=11011";
    assert_eq!(27, part1(parse(input1)));
    assert_eq!(15, part2(parse(input1)));
    println!("\n\n");
    let input2 = r"width=40
height=12
horizontal-offsets=11100
vertical-offsets=001101";
    assert_eq!(7, part2(parse(input2)));
    let input3 = r"width=100
height=70
horizontal-offsets=111101111101101111000100100110
vertical-offsets=110100001110111011101000001111";
    assert_eq!(269, part2(parse(input3)));
}
