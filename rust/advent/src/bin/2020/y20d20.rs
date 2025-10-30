use std::iter::successors;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;
use utilities::{enums::cardinals::Cardinal, parsing::get_numbers::ContainsNumbers, structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};
use utilities::structs::coord::Coord2U;
use utilities::structs::grid::{GridIterator, GridRotation};

type Input = (Vec<Tile>, Stitcher);
type Output = usize;
type Stitcher = FxHashMap<u16, Vec<TileOrient>>;
type Pos = Coord2U;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tile {
    id: usize,
    image: Grid2<bool>,
    sides: Vec<u16>,
}
impl Tile {
    fn matching_tiles<'a>(&self, tiles: &Vec<Tile>, stitcher: &'a Stitcher) -> Vec<&'a TileOrient> {
        self.sides.iter()
            .flat_map(|side| stitcher.get(side).unwrap().iter())
            .unique()
            .filter(|tile_orient| {
                let tile = &tiles[tile_orient.tile_index];
                tile != self
            })
            .collect()
    }
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let (id, image) = value.split_once('\n').unwrap();
        let id: usize = id.get_numbers().next().unwrap();
        let image = Grid2::new2d_map_str(image, |c| c == '#').unwrap();
        let sides: [Box<dyn Iterator<Item = &bool>>; 4] = [
            Box::new(image.row(0).unwrap()),
            Box::new(image.column(image.width() - 1).unwrap()),
            Box::new(image.row(image.height() - 1).unwrap().rev()),
            Box::new(image.column(0).unwrap().rev()),
        ];
        let width = image.width();
        let mut sides: Vec<u16> = sides.into_iter()
            .map(|side| {
                side.enumerate().fold(0, |acc, (i, &pixel)| {
                    let p = if pixel {
                        1 << (width - i - 1)
                    } else {
                        0
                    };
                    acc + p
                })
            })
            .collect();
        for i in [0, 3, 2, 1] {
            sides.push(reverse(sides[i], width));
        }
        Self { id, image, sides }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TileOrient {
    tile_index: usize,
    dir: Cardinal,
    flipped: bool,
}

impl TileOrient {
    fn match_value(&self, dir: Cardinal, tiles: &[Tile]) -> u16 {
        let value_index = (dir.ordinal() as i8 - self.dir.ordinal() as i8).rem_euclid(4)
            + if self.flipped { 4 } else { 0 };
        tiles[self.tile_index].sides[value_index as usize]
    }

    fn reorient(&self, dir: Cardinal) -> Self {
        let orient = (dir.ordinal() as i8 - self.dir.ordinal() as i8).rem_euclid(4);
        let reorient = Cardinal::entries()[orient as usize];
        Self { tile_index: self.tile_index, dir: reorient, flipped: self.flipped }
    }

    fn borderless(&self, tiles: &[Tile]) -> Grid2<bool> {
        tiles[self.tile_index].image
            .sub_grid(Pos::new2d(1, 1), Pos::new2d(8, 8))
            .map(|grid| {
                let grid = if self.flipped {
                    grid.rotate(GridRotation::FlipX)
                } else {
                    grid
                };
                match self.dir {
                    Cardinal::North => grid,
                    Cardinal::East => grid.rotate(GridRotation::Right),
                    Cardinal::South => grid.rotate(GridRotation::OneEighty),
                    Cardinal::West => grid.rotate(GridRotation::Left),
                }
            })
            .unwrap()
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn reverse(side: u16, digits: usize) -> u16 {
    successors(Some(side), |&acc| {
        if acc >> 1 == 0 {
            None
        } else {
            Some(acc >> 1)
        }
    })
        .enumerate()
        .fold(0, |acc, (idx, n)| {
            acc | ((n & 1) << (digits - idx - 1))
        })
}

fn parse_input(input: &str) -> Input {
    let tiles: Vec<_> = input.split("\n\n")
        .map(Tile::from)
        .collect();
    let mut stitcher: Stitcher = Stitcher::default();
    for (tile_index, tile) in tiles.iter().enumerate() {
        for (side_index, &side) in tile.sides.iter().enumerate() {
            stitcher.entry(side)
                .or_insert(Vec::new())
                .push(
                    TileOrient {
                        tile_index,
                        dir: Cardinal::entries()[side_index % 4],
                        flipped: side_index >= 4,
                    }
                );
        }
    }
    (tiles, stitcher)
}

fn part1((tiles, stitcher): &Input) -> Output {
    tiles.iter()
        .filter(|tile| tile.matching_tiles(tiles, stitcher).len() == 4)
        .fold(1, |acc, tile| acc * tile.id)
}

fn part2((tiles, stitcher): &Input) -> Output {
    // form grid
    let width = tiles.len().isqrt();
    let (corner_index, _) = tiles.iter().enumerate()
        .find(|(_, tile)| tile.matching_tiles(tiles, stitcher).len() == 4)
        .unwrap();
    let mut first_tile = TileOrient {
        tile_index: corner_index,
        dir: Cardinal::North,
        flipped: false,
    };
    while stitcher.get(&first_tile.match_value(Cardinal::South, tiles)).unwrap().len() != 2
        || stitcher.get(&first_tile.match_value(Cardinal::East, tiles)).unwrap().len() != 2
    {
        first_tile.dir = first_tile.dir.right();
    }
    let mut stitched = vec![first_tile];

    // fill in each tile from left to right, top to bottom, connecting each new piece with the tile above
    // and to the left
    for index in 1..tiles.len() {
        // find next tile. Special if case for left-most tiles, because those check against the tile above
        // rather than the tile to the left
        let next_tile = if index % width == 0 { //leftmost
            let reference = stitched[index - width];
            stitcher.get(&reverse(reference.match_value(Cardinal::South, tiles), 10)).unwrap().iter()
                .find(|it| it.tile_index != reference.tile_index)
                .map(|it| it.reorient(Cardinal::North))
                .unwrap()
        } else {
            let reference = stitched[index - 1];
            stitcher.get(&reverse(reference.match_value(Cardinal::East, tiles), 10)).unwrap().iter()
                .find(|it| it.tile_index != reference.tile_index)
                .map(|it| it.reorient(Cardinal::West))
                .unwrap()
        };
        stitched.push(next_tile);
    }

    // stitch image together
    let whole_image = stitched.into_iter()
        .map(|orient| orient.borderless(tiles))
        .try_collect_grid(width)
        .unwrap();
    let whole_image = whole_image
        .rows()
        .map(|row| {
            let mut iter = row.iter();
            let init = (*iter.next().unwrap()).clone();
            iter.fold(init, |acc, &image| {
                acc.add_right(image).unwrap()
            })
        })
        .reduce(|acc, row| acc.add_down(&row).unwrap())
        .unwrap();

    let nessie: [Pos; 15] = [
        Pos::new2d(18, 0),
        Pos::new2d(0, 1),
        Pos::new2d(5, 1),
        Pos::new2d(6, 1),
        Pos::new2d(11, 1),
        Pos::new2d(12, 1),
        Pos::new2d(17, 1),
        Pos::new2d(18, 1),
        Pos::new2d(19, 1),
        Pos::new2d(1, 2),
        Pos::new2d(4, 2),
        Pos::new2d(7, 2),
        Pos::new2d(10, 2),
        Pos::new2d(13, 2),
        Pos::new2d(16, 2),
    ];

    let pixels = whole_image.iter().filter(|&&b| b).count();

    let flipped = whole_image.rotate(GridRotation::FlipX);

    let monsters: usize = (0..8)
        .into_par_iter()
        .map(|i| {
            let image = if i >= 4 {
                &whole_image
            } else {
                &flipped
            };
            let image = match i % 4 {
                0 => image,
                1 => &image.rotate(GridRotation::Right),
                2 => &image.rotate(GridRotation::OneEighty),
                _ => &image.rotate(GridRotation::Left),
            };
            let mut monsters = 0;
            for y in 0..image.height() - 3 {
                for x in 0..image.width() - 20 {
                    let offset = Pos::new2d(x, y);
                    let found_nessie = nessie.iter().all(|&pos| image[pos + offset]);
                    if found_nessie {
                        monsters += 1;
                    }
                }
            }
            monsters
        })
        .sum();
    pixels - monsters * 15
}

#[test]
fn default() {
    let input = get_input(20, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(19955159604613, part1(&input));
    assert_eq!(1639, part2(&input));
}

