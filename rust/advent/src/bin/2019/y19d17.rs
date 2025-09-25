use advent::utilities::{get_input::get_input, intcode::IntCode};
use lazy_regex::regex;
use lazy_regex::regex::Match;
use utilities::{
    enums::cardinals::Cardinal,
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2U,
        grid::{Grid2, GridIterator},
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = (Vec<i64>, Grid2<i64>);
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut code: Vec<_> = input.get_numbers().collect();
    let mut ic = IntCode::new(&code);
    let (_, output) = ic.run_while_able();
    let width = output.iter().position(|&it| it == 10).unwrap();
    let grid = output
        .into_iter()
        .filter(|&it| it != 10)
        .try_collect_grid(width)
        .unwrap();
    code[0] = 2;
    (code, grid)
}

fn part1(input: &Input) -> Output {
    let (_, grid) = input;
    grid.iter()
        .enumerate()
        .filter_map(|(index, &l)| {
            if l != 35
                || grid
                    .adjacent(index, false)
                    .unwrap()
                    .any(|adj| *adj.value != 35)
            {
                None
            } else {
                let pos = grid.coord_of(index).unwrap();
                Some(pos.x() * pos.y())
            }
        })
        .sum()
}

fn part2(input: &Input) -> Output {
    let (code, grid) = input;
    let index = grid.iter().position(|&it| it == 94).unwrap();
    let mut pos = grid.coord_of(index).unwrap();
    let mut dir = Cardinal::North;
    let mut counter = 0;
    let mut path = String::new();
    let l_str = "L,";
    let r_str = "R,";
    loop {
        if let Some(prospect) = cromulent(grid, pos, dir) {
            counter += 1;
            pos = prospect;
        } else if let Some(_) = cromulent(grid, pos, dir.left()) {
            dir = dir.left();
            if counter != 0 {
                path.push_str(counter.to_string().as_str());
                path.push(',');
                counter = 0;
            }
            path.push_str(l_str);
        } else if let Some(_) = cromulent(grid, pos, dir.right()) {
            dir = dir.right();
            if counter != 0 {
                path.push_str(counter.to_string().as_str());
                path.push(',');
                counter = 0;
            }
            path.push_str(r_str);
        } else {
            path.push_str(counter.to_string().as_str());
            path.push(',');
            break;
        }
    }
    let matches: Vec<Match> = regex!(r"(?:L|R),\d+,").find_iter(&path).collect();
    let (form_seq, forms) = get_commands(&path, &matches);
    let video = ['n', '\n'];
    let to_ic: Vec<i64> = form_seq.chars()
        .chain(forms.iter().flat_map(|form| form.chars()))
        .chain(video.into_iter())
        .map(|c| c as i64)
        .collect();
    let mut ic = IntCode::new(&code);
    ic.input_slice(&to_ic);
    let (_, output) = ic.run_while_able();
    output[output.len() - 1] as usize
}

fn cromulent(grid: &Grid2<i64>, pos: Pos, dir: Cardinal) -> Option<Pos> {
    let Some(prospect) = pos.move_direction(dir, 1) else {
        return None;
    };
    if *grid.get(prospect).unwrap_or(&0) == 35 {
        Some(prospect)
    } else {
        None
    }
}

fn get_commands(path: &str, matches: &[Match]) -> (String, Vec<String>) {
    let mut commands: Vec<String> = Vec::new();
    split_commands(path, matches, &mut commands, 3);
    let mut form_sequence = String::new();
    let mut cursor = 0;
    while cursor < path.len() {
        let position = commands.iter()
            .position(|s| path[cursor..].starts_with(s))
            .unwrap();
        let c = match position {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            _ => unreachable!(),
        };
        form_sequence.push(c);
        form_sequence.push(',');
        cursor += commands[position].len();
    }
    for command in commands.iter_mut() {
        command.pop();
        command.push('\n');
    }
    form_sequence.pop();
    form_sequence.push('\n');
    (form_sequence, commands)
}

fn split_commands(
    path: &str,
    matches: &[Match],
    commands: &mut Vec<String>,
    level: usize
) -> bool {
    // base case
    if level == 0 && matches.len() == 0 {
        return true;
    }
    let mut start = matches[0].start();
    'outer: loop {
        for command in commands.iter() {
            if path[start..].starts_with(command) {
                start += command.len();
                continue 'outer;
            }
        }
        break;
    }
    if level == 0 {
        return start >= path.len();
    }
    for (index, mat) in matches.iter()
        .enumerate()
        .filter(|(_, m)| m.end() <= start + 20)
        .rev()
    {
        commands.push(path[start..mat.end()].to_string());
        if split_commands(path, &matches[index + 1..], commands, level - 1) {
            return true;
        } else {
            commands.pop();
        }
    }
    return false
}

#[test]
fn default() {
    let input = get_input(19, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(10632, part1(&input));
    // assert_eq!(1356191, part2(&input));
}

// Input parsed (490μs)
// 1. 10632 (28μs)
// 2. 1356191 (1ms)
// Total: 1ms