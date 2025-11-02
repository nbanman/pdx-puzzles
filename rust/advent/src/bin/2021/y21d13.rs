use advent::utilities::get_input::get_input;
use advent_ocr::ocr;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2U, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};

type Int = usize;
type Input = (Paper, Vec<FoldInstruction>);
type Pos = Coord2U;
type Paper = Grid2<bool>;

enum Axis { X, Y }

struct FoldInstruction {
    axis: Axis,
    amt: usize,
}

impl FoldInstruction {
    fn execute(&self, paper: &mut Paper, tl: Pos, br: Pos) -> (Pos, Pos) {
        match self.axis {
            Axis::X => {
                let lbr = Pos::new2d(tl.x() + self.amt - 1, br.y());
                let rtl = Pos::new2d(tl.x() + self.amt + 1, tl.y());
                self.perform_fold(paper, tl, lbr, rtl, br)
            },
            Axis::Y => {
                let ubr = Pos::new2d(br.x(), tl.y() + self.amt - 1);
                let dtl = Pos::new2d(tl.x(), tl.y() + self.amt + 1);
                self.perform_fold(paper, tl, ubr, dtl, br)
            },
        }
    }

    fn perform_fold(&self, paper: &mut Paper, atl: Pos, abr: Pos, btl: Pos, bbr: Pos) -> (Pos, Pos) {
        match self.axis {
            Axis::X => {
                if abr.x() - atl.x() >= bbr.x() - btl.x() {
                    Pos::for_rectangle(btl, bbr, |pos| {
                        if paper[pos] {
                            let dist = pos.x() - btl.x();
                            paper[Pos::new2d(abr.x() - dist, pos.y())] = true;
                        }
                    });
                    (atl, abr)
                } else {
                    Pos::for_rectangle(atl, abr, |pos| {
                        if paper[pos] {
                            let dist = abr.x() - pos.x();
                            paper[Pos::new2d(btl.x() + dist, pos.y())] = true;
                        }
                    });
                    (btl, bbr)
                }
            },
            Axis::Y => {
                if abr.y() - atl.y() >= bbr.y() - btl.y() {
                    Pos::for_rectangle(btl, bbr, |pos| {
                        if paper[pos] {
                            let dist = pos.y() - btl.y();
                            paper[Pos::new2d(pos.x(), abr.y() - dist)] = true;
                        }
                    });
                    (atl, abr)
                } else {
                    Pos::for_rectangle(atl, abr, |pos| {
                        if paper[pos] {
                            let dist = abr.y() - pos.y();
                            paper[Pos::new2d(pos.x(), btl.y() + dist)] = true;
                        }
                    });
                    (btl, bbr)
                }
            },
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots = dots.lines()
        .map(|line| {
            let (x, y) = line.get_numbers().collect_tuple().unwrap();
            Pos::new2d(x, y)
        })
        .collect_vec();
    let width = dots.iter().map(|pos| pos.x()).max().unwrap() + 1;
    let height= dots.iter().map(|pos| pos.y()).max().unwrap() + 1;

    let mut paper = Grid2::new2d_with_fn(width, height, |_| false);
    for dot in dots {
        paper[dot] = true;
    }
    let folds = folds.lines()
        .map(|line| {
            let (_, _, axis, amt) = line.split([' ', '=']).collect_tuple().unwrap();
            let axis = match axis {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => unreachable!(),
            };
            let amt = amt.parse().unwrap();
            FoldInstruction { axis, amt }
        })
        .collect_vec();
    (paper, folds)
}

fn part1(input: &Input) -> Int {
    let (paper, folds) = input;
    let mut paper = paper.clone();
    let paper_br = Pos::new2d(paper.width() - 1, paper.height() -1);
    let (tl, br) = folds.first().unwrap()
        .execute(&mut paper, Pos::origin(), paper_br);
    let mut count = 0;
    Pos::for_rectangle(tl, br, |pos| {
        if paper[pos] {
            count += 1;
        }
    });
    count
}

fn part2(input: &Input) -> String {
    let (paper, folds) = input;
    let mut paper = paper.clone();
    let paper_br = Pos::new2d(paper.width() - 1, paper.height() -1);

    let (tl, br) = folds.iter()
        .fold((Pos::origin(), paper_br), |(tl, br), fold| {
            fold.execute(&mut paper, tl, br)
        });
    let size = Pos::new2d(br.x() - tl.x() + 1, br.y() - tl.y() + 1);
    let folded = paper.sub_grid(tl, size).unwrap();
    ocr(&folded).unwrap()
}

#[test]
fn default() {
    let input = get_input(21, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(735, part1(&input));
    assert_eq!("UFRZKAUZ".to_string(), part2(&input));
}

// Input parsed (448μs)
// 1. 735 (44ms)
// 2. UFRZKAUZ (48ms)
// Total: 93ms