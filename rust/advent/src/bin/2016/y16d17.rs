use advent::utilities::get_input::get_input;
use md5::{Digest, Md5};
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}}};

type Pos = Coord2U;
type State = (String, Pos);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 17).unwrap();
    let (part1, part2) = explore(input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} (0μs)", part1);
    println!("2. {} (0μs)", part2);
    println!("Total: {}", stopwatch.stop().report());
}

fn explore(salt: String) -> (String, usize) {
    let start = (salt.clone(), Pos::origin());
    let end_pos = Pos::new2d(3, 3);
    let open_range = b'B'..=b'F';
    let mut hasher = Md5::new();
    let mut buf = [0u8; 32];
    
    let mut get_edges = |passcode: String, pos: Pos| {
        let mut edges = Vec::new();
        Digest::update(&mut hasher, &passcode);
        let hash: [u8; 16] = Digest::finalize_reset(&mut hasher).into();
        let hash = base16ct::upper::encode_str(&hash, &mut buf).unwrap().to_string();
        let hash = hash.as_bytes();
        for (i, door) in hash[..4].iter().enumerate() {
            if open_range.contains(door) {
                let neighbor = match i {
                    0 => make_edge(&passcode, pos, Cardinal::North, 'U'),
                    1 => make_edge(&passcode, pos, Cardinal::South, 'D'),
                    2 => make_edge(&passcode, pos, Cardinal::West, 'L'),
                    3 => make_edge(&passcode, pos, Cardinal::East, 'R'),
                    _ => None,
                };
                if let Some(neighbor) = neighbor {
                    edges.push(neighbor);
                }
            }
        }
        edges
    };

    let mut first: Option<String> = None;
    let mut last: Option<usize> = None;

    let mut todo = Vec::new();
    todo.push(start);
    let mut next = Vec::new();
    let mut steps = 0;
    while !todo.is_empty() {
        for (passcode, pos) in todo.drain( .. ) {
            // when vault is reached, do not end, but also no paths should be explored leading
            // away from the vault. Mark the moves for the shortest distance, and track the
            // most recent visit to the vault.
            if pos == end_pos {
                if first.is_none() {
                    let passcode = passcode[salt.len()..].to_string();
                    first = Some(passcode);
                }
                last = Some(steps);
            } else {
                for (edge_pass, edge_pos) in get_edges(passcode, pos) {
                    next.push((edge_pass, edge_pos))
                }
            }
        }
        steps += 1;
        std::mem::swap(&mut todo, &mut next);
    }
    (first.unwrap(), last.unwrap())
}

fn make_edge(passcode: &String, pos: Pos, dir: Cardinal, c: char) -> Option<State> {
    let new_pos = pos.move_direction(dir, 1)?;
    if new_pos.x() > 3 || new_pos.y() > 3 { return None; }
    let mut new_pass = passcode.clone();
    new_pass.push(c);
    Some((new_pass, new_pos))
}

#[test]
fn default() {
    let input = get_input(16, 17).unwrap();
    let (part1, part2) = explore(input);
    assert_eq!("DDRUDLRRRD".to_string(), part1);
    assert_eq!(398, part2);
}

// Input parsed (17ms)
// 1. DDRUDLRRRD (0μs)
// 2. 398 (0μs)
// Total: 17ms