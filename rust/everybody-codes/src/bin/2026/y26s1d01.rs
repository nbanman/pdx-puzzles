use everybody_codes::utilities::inputs::get_story_inputs;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

struct Scale {
    value: usize,
    r: u8,
    g: u8,
    b: u8,
    shine_value: Option<u8>,
}

impl Scale {
    fn color_str_to_bin(color_str: &str) -> u8 {
        color_str
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (place, c)| {
                if c.is_lowercase() {
                    acc
                } else {
                    acc + 2u8.pow(place as u32)
                }
            })
    }

    fn color_sum(&self) -> u16 {
        self.r as u16 + self.g as u16 + self.b as u16
    }

    fn shine(&self) -> Shine {
        match self.shine_value {
            Some(n) if n <= 30 => Shine::Matte,
            Some(n) if n >= 33 => Shine::Shiny,
            _ => Shine::Indeterminate,
        }
    }

    fn dominant_color(&self) -> DominantColor {
        if self.r > self.g {
            // can't be g
            if self.r > self.b {
                DominantColor::Red
            } else if self.b > self.r {
                DominantColor::Blue
            } else {
                DominantColor::Indeterminate
            }
        } else if self.g > self.r {
            // can't be r
            if self.g > self.b {
                DominantColor::Green
            } else if self.b > self.g {
                DominantColor:: Blue
            } else {
                DominantColor::Indeterminate
            }
        } else if self.b > self.r {
            DominantColor::Blue
        } else {
            DominantColor::Indeterminate
        }
    }
}

impl From<&str> for Scale {
    fn from(s: &str) -> Self {
        let (value, binary) = s.split_once(':').unwrap();
        let value = value.parse().unwrap();
        let mut split = binary.split(' ');
        let r = split.next().map(Self::color_str_to_bin).unwrap();
        let g = split.next().map(Self::color_str_to_bin).unwrap();
        let b = split.next().map(Self::color_str_to_bin).unwrap();
        let shine = split.next().map(Self::color_str_to_bin);
        Self {
            value,
            r,
            g,
            b,
            shine_value: shine,
        }
    }
}

#[derive(PartialEq, Eq)]
enum DominantColor {
    Red,
    Green,
    Blue,
    Indeterminate,
}

#[derive(PartialEq, Eq)]
enum Shine {
    Matte,
    Shiny,
    Indeterminate,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs(26, 1, 1);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: &str) -> impl Iterator<Item = Scale> {
    input.lines().map(Scale::from)
}

fn part1(input: &str) -> usize {
    parse(input)
        .filter(|scale| scale.dominant_color() == DominantColor::Green)
        .map(|scale| scale.value)
        .sum()
}

fn part2(input: &str) -> usize {
    let scales: Vec<Scale> = parse(input).collect();
    let max_shine = scales
        .iter()
        .map(|scale| scale.shine_value.unwrap())
        .max()
        .unwrap();
    scales
        .iter()
        .filter(|scale| scale.shine_value == Some(max_shine))
        .min_by_key(|scale| scale.color_sum())
        .unwrap()
        .value
}

fn part3(input: &str) -> usize {
    let scales = parse(input).filter(|scale| {
        scale.shine() != Shine::Indeterminate
            && scale.dominant_color() != DominantColor::Indeterminate
    });
    let mut scale_groups: Vec<Vec<usize>> = vec![Vec::new(); 6];
    for scale in scales {
        let index = scale.dominant_color() as usize + 3 * scale.shine() as usize;
        scale_groups[index].push(scale.value);
    }
    scale_groups
        .iter()
        .max_by_key(|group| group.len())
        .map(|group| group.iter().sum())
        .unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(26, 1, 1);
    assert_eq!(51660, part1(&input1));
    assert_eq!(55911, part2(&input2));
    assert_eq!(11645480, part3(&input3));
}

#[test]
fn examples() {
    let example1 = r"2456:rrrrrr ggGgGG bbbbBB
7689:rrRrrr ggGggg bbbBBB
3145:rrRrRr gggGgg bbbbBB
6710:rrrRRr ggGGGg bbBBbB";
    assert_eq!(part1(example1), 9166);
    let example2 = r"2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss";
    assert_eq!(part2(example2), 2456);
    let example3 = r"15437:rRrrRR gGGGGG BBBBBB sSSSSS
94682:RrRrrR gGGggG bBBBBB ssSSSs
56513:RRRrrr ggGGgG bbbBbb ssSsSS
76346:rRRrrR GGgggg bbbBBB ssssSs
87569:rrRRrR gGGGGg BbbbbB SssSss
44191:rrrrrr gGgGGG bBBbbB sSssSS
49176:rRRrRr GggggG BbBbbb sSSssS
85071:RRrrrr GgGGgg BBbbbb SSsSss
44303:rRRrrR gGggGg bBbBBB SsSSSs
94978:rrRrRR ggGggG BBbBBb SSSSSS
26325:rrRRrr gGGGgg BBbBbb SssssS
43463:rrrrRR gGgGgg bBBbBB sSssSs
15059:RRrrrR GGgggG bbBBbb sSSsSS
85004:RRRrrR GgGgGG bbbBBB sSssss
56121:RRrRrr gGgGgg BbbbBB sSsSSs
80219:rRRrRR GGGggg BBbbbb SssSSs";
    assert_eq!(part3(example3), 292320)
}

// Input parsed (47μs)
// 1. 51660 (8μs)
// 2. 55911 (46μs)
// 3. 11645480 (190μs)
// Total: 296μs
