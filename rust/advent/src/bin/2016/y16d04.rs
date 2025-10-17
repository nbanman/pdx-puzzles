use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Room>;
type Output = usize;

#[derive(Debug)]
struct Room {
    name: String,
    id: usize,
}

impl TryFrom<&str> for Room {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rx = regex!(r"(?<encrypted_name>[a-z-]+)-(?<id>\d+)\[(?<checksum>[a-z]+)\]");
        let caps = rx.captures(value).ok_or("Regex no matchy")?;
        let encrypted_name = caps.name("encrypted_name").unwrap().as_str();
        let id: usize = caps.name("id").unwrap().as_str().parse().unwrap();
        let checksum = caps.name("checksum").unwrap().as_str();

        let generated_checksum: String = encrypted_name
            .replace("-", "")
            .chars()
            .counts()
            .into_iter()
            .sorted_unstable_by(|(ak, av), (bk, bv)| {
                bv.cmp(av).then(ak.cmp(bk))
            })
            .take(5)
            .map(|(k, _)| k)
            .collect();

        if checksum == generated_checksum {
            let name: String = encrypted_name.chars()
                .map(|c| {
                    if c == '-' {
                        ' '
                    } else {
                        ((c as usize + id - 97) % 26 + 97) as u8 as char
                    }
                })
                .collect();
            Ok(Self { name, id })
        } else {
            Err("Checksum no matchy".to_string())
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines().filter_map(|line| line.try_into().ok()).collect()
}

fn part1(rooms: &Input) -> Output {
    rooms.iter().map(|room| room.id).sum()
}

fn part2(rooms: &Input) -> Output {
    rooms.iter()
        .find(|room| room.name == "northpole object storage")
        .map(|room| room.id)
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(16, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(158835, part1(&input));
    assert_eq!(993, part2(&input));
}

// Input parsed (1ms)
// 1. 158835 (5μs)
// 2. 993 (2μs)
// Total: 1ms