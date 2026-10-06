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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Walls(u8);

impl Walls {
    fn add_up(&mut self) {
        self.0 += 8;
    }
    fn add_right(&mut self) {
        self.0 += 4;
    }
    fn add_down(&mut self) {
        self.0 += 2;
    }
    fn add_left(&mut self) {
        self.0 += 1;
    }
    fn has_up(&self) -> bool {
        self.0 >> 3 == 1
    }
    fn has_right(&self) -> bool {
        self.0 >> 2 & 1 == 1
    }
    fn has_down(&self) -> bool {
        self.0 >> 1 & 1 == 1
    }
    fn has_left(&self) -> bool {
        self.0 & 1 == 1
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

fn find_isolated(input: Input) -> [u64; 2] {
    let Pattern {
        height,
        width,
        hz,
        vt,
    } = input;
    todo!()
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

    let mut tiles = vec![Walls::default(); width * height];

    for (x, (&left, &right)) in vt.iter()
        .cycle()
        .take(width + 1)
        .tuple_windows()
        .enumerate()
    {
        for y in 0..height {
            let offset = (y & 1) as u8;
            if left == offset {
                tiles[y * width + x].add_left();
            }
            if right == offset {
                tiles[y * width + x].add_right();
            }
        }
    }

    for (y, (&up, &down)) in hz.iter()
        .cycle()
        .take(height + 1)
        .tuple_windows()
        .enumerate()
    {
        for x in 0..width {
            let offset = (x & 1) as u8;
            if up == offset {
                tiles[y * width + x].add_up();
            }
            if down == offset {
                tiles[y * width + x].add_down();
            }
        }
    }

    let mut count = [0u64; 2];
    let mut start_colors = vec![false; height];
    for y in 0..height {
        let mut color = if y == 0 {
            true
        } else {
           if tiles[y * width].has_up() {
               !start_colors[y - 1]
           } else {
               start_colors[y - 1]
           }
        };
        start_colors[y] = color;
        for x in 0..width {
            let walls = tiles[y * width + x];
           if walls == Walls(15) {
                if color {
                    count[0] += 1;
                } else {
                    count[1] += 1;
                }
            }

            if walls.has_right() {
                color = !color;
            }
        }
    }
    *count.iter().max().unwrap()
}

fn part3(input: Input) -> u64 {
    let Pattern {
        height,
        width,
        hz,
        vt,
    } = input;

    let mut tiles = vec![Walls::default(); hz.len() * vt.len()];
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
