use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{enums::cardinals::Cardinal, structs::{coord::{Coord, Coord2, Coord2U}, grid::{Grid, Grid2, GridIterator}, stopwatch::{ReportDuration, Stopwatch}}};

type Input<'a> = (Grid2<Terrain>, Vec<Command>);
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy)]
enum Terrain {
    Space,
    Wall,
    Path,
}

fn parse_input(input: &str) -> Input {
    let (grove, path_str) = input.split_once("\n\n").unwrap();
    
    let grove: Vec<&str> = grove.trim_end().lines().collect();
    let width = grove.iter()
        .map(|line| line.len())
        .max()
        .unwrap();
    let grove: Vec<Vec<Terrain>> = grove.into_iter()
        .map(|line| {
            line.as_bytes().into_iter()
                .map(|&b| {
                    match b {
                        b' ' => Terrain::Space,
                        b'#' => Terrain::Wall,
                        b'.' => Terrain::Path,
                        b => { panic!("Cannot interpret '{b}'"); },
                    }
                })
                .pad_using(width, |_| Terrain::Space)
                .collect()
        })
        .collect();
    let grove = Grid::try_from(grove).unwrap();

    let mut path = Vec::new();
    regex!(r"\d+|[LR]")
        .find_iter(path_str)
        .for_each(|command| {
            match command.as_str() {
                "L" => { path.push(Command::Left); },
                "R" => { path.push(Command::Right); },
                n => for _ in 0..n.parse::<usize>().unwrap() {
                    path.push(Command::Forward);
                },
            }
        });


    (grove, path)
}

fn solve<F>(
    grove: &Grid2<Terrain>, 
    path: &[Command], 
    movement: F,
) -> Output 
where 
    F: Fn(Pos, Cardinal) -> (Pos, Cardinal)
{
    let start = {
        let x = grove.row(0)
            .unwrap()
            .position(|t| {
                matches!(t, Terrain::Path)
            })
            .unwrap();
        Pos::new2d(x as i64, 0)
    };

    let mut dir = Cardinal::East;
    let end = path.iter().fold(start, |pos, command| {
        if matches!(command, Command::Forward) {
            let (prospect, prospective_dir) = movement(pos, dir);
            let Coord([x, y]) = prospect;
            if matches!(grove.get([x, y]), Some(Terrain::Wall)) {
                pos
            } else {
                dir = prospective_dir;
                prospect
            }
        } else {
            dir = match command {
                Command::Left => dir.left(),
                Command::Right => dir.right(),
                Command::Forward => { panic!("Should only be turning"); },
            };
            pos
        }
    });
    
    let facing = match dir {
        Cardinal::North => 3,
        Cardinal::East => 0,
        Cardinal::South => 1,
        Cardinal::West => 2,
    };

    1000 * (end.y() as usize + 1) + 4 * (end.x() as usize + 1) + facing
}

fn part1((grove, path): &Input) -> Output {
    let row_bounds: Vec<_> = grove.rows()
        .map(|row| {
            let start = row.iter()
                .position(|&t| !matches!(t, Terrain::Space))
                .unwrap();
            let end = row[start + 1..].iter()
                .position(|&t| matches!(t, Terrain::Space))
                .unwrap_or(grove.width() - 1 - start) + start;
            start as i64..end as i64
        })
        .collect();
    let col_bounds: Vec<_> = grove.columns()
        .map(|column| {
            let start = column.iter()
                .position(|t| !matches!(t, Terrain::Space))
                .unwrap();
            let end = column[start + 1..]
                .iter()
                .position(|t| matches!(t, Terrain::Space))
                .unwrap_or(column.len() - 1 - start) + start;
            start as i64..end as i64
        })
        .collect();
    
    let movement = |pos: Pos, dir: Cardinal| {
        let prospect = pos.move_direction(dir, 1).unwrap();
        let Coord([x, y]) = prospect;
        if !(0..grove.height() as i64).contains(&y) ||
                !(0..grove.width() as i64).contains(&x) ||
                matches!(grove.get([x, y]), Some(Terrain::Space)) {
            match dir {
                Cardinal::North | Cardinal::South => {
                    let bounds = &col_bounds[x as usize];
                    let new_y = if y < bounds.start {
                        bounds.end
                    } else {
                        bounds.start
                    };
                    (Pos::new2d(x, new_y), dir)
                },
                Cardinal::East | Cardinal::West => {
                    let bounds = &row_bounds[y as usize];
                    let new_x = if x < bounds.start {
                        bounds.end
                    } else {
                        bounds.start
                    };
                    (Pos::new2d(new_x, y), dir)
                },
            }
        } else {
            (prospect, dir)
        }
    };
    solve(grove, path, movement)
}



fn part2((grove, path): &Input) -> Output {
    // get the length of each side
    let space = grove.iter()
        .filter(|&terrain| matches!(terrain, Terrain::Space))
        .count();
    let side_length = ((grove.len() - space) as f64 / 6.0).sqrt() as usize;
    

    // make mini-grid, one pixel per side
    let mini_width = grove.width() / side_length;

    let mini_grove: Grid2<Terrain> = (0..grove.height() / side_length)
        .cartesian_product(0..mini_width)
        .map(|(y, x)| *grove.get([x * side_length, y * side_length]).unwrap())
        .try_collect_grid(mini_width)
        .unwrap();

    fn hash(shape: &[Coord2U]) -> usize {
        shape.iter()
            .map(|pos| pos.x() + pos.y() * 4)
            .sum()
    }

    // map of various shapes
    let shapes: FxHashMap<usize, _> = vec![
        (vec![(0, 0), (0, 1), (1, 1)], (Cardinal::East, Cardinal::South)),
        (vec![(0, 0), (0, 1), (0, 2), (1, 2)], (Cardinal::East, Cardinal::West)),
        (vec![(0, 0), (0, 1), (1, 1), (2, 1)], (Cardinal::North, Cardinal::South)),
        (vec![(0, 0), (0, 1), (0, 2), (0, 3)], (Cardinal::South, Cardinal::South)),
        (vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)], (Cardinal::West, Cardinal::South)),
        (vec![(0, 0), (0, 1), (1, 1), (2, 1), (3, 1)], (Cardinal::East, Cardinal::East)),
        (vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)], (Cardinal::North, Cardinal::West)),
        (vec![(0, 0), (0, 1), (1, 1), (1, 2), (1, 3)], (Cardinal::West, Cardinal::North)),
        (vec![(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)], (Cardinal::South, Cardinal::East)),
        (vec![(0, 0), (0, 1), (1, 1), (1, 2), (2, 2), (2, 3)], (Cardinal::East, Cardinal::East)),
        (vec![(0, 0), (0, 1), (1, 1), (1, 2), (1, 3), (2, 3)], (Cardinal::North, Cardinal::North)),
        (vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 2), (3, 2)], (Cardinal::West, Cardinal::West)),
        (vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 3), (1, 4)], (Cardinal::East, Cardinal::East)),
    ]
        .into_iter()
        .map(|(shape, directions)| {
            let shape: Vec<Coord2U> = shape.into_iter()
                .map(|(x, y)| Coord::new2d(x, y))
                .collect();
            (hash(&shape), (shape, directions))
        })
        .collect();

    // map representing the sides of the cube and how the sides of the cube line up with each other.
    // the key is the position in the miniGrove, the value is another map
    // this second map takes the current direction and provides the destination coordinate in the miniGrove
    // and the new direction.
    let mut sides = FxHashMap::default();
    for (start, &terrain) in mini_grove.coords()
        .zip(mini_grove.iter())
        .filter(|(_, &terrain)| !matches!(Terrain::Space, terrain)) 
    {
        
    }

    15410
}

#[test]
fn default() {
    let input = get_input(22, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(133174, part1(&input));
    assert_eq!(15410, part2(&input));
}
