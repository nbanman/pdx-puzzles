use advent::utilities::get_input::get_input;
use utilities::structs::{coord::{Coord, Coord2, Coord2U}, grid::{Grid2, GridRotation}, stopwatch::{ReportDuration, Stopwatch}};

type Int = usize;
type Input<'a> = &'a str;
type Pos = Coord2U;

struct FoldInstruction {
    axis: char,
    amt: usize,
}

impl FoldInstruction {
    fn execute(&self, paper: Grid2<bool>) -> Grid2<bool> {
        match self.axis {
            'x' => {
                let left = paper
                    .sub_grid(Pos::origin(), Pos::new2d(self.amt, paper.height()))
                    .unwrap();
                let right = paper
                    .sub_grid(
                        Pos::new2d(self.amt + 1, 0), 
                        Pos::new2d(paper.width() - 1 - self.amt, paper.height())
                    )
                    .unwrap()
                    .rotate(GridRotation::FlipX);
                let (larger, smaller) = if left.len() > right.len() {
                    (left, right)
                } else {
                    (right, left)
                };
                let offset = larger.width() as i64 - smaller.width() as i64;
                perform_fold(Coord::new2d(offset, 0), larger, smaller)
            },
            'y' => {
                let up = paper
                    .sub_grid(Pos::origin(), Pos::new2d(paper.width(), self.amt))
                    .unwrap();
                let down = paper
                    .sub_grid(Pos::new2d(0, self.amt + 1), Pos::new2d(paper.width(), paper.height() - 1 - amt))
                    .unwrap()
                    .rotate(GridRotation::FlipY);
                let (larger, smaller) = if up.len() > down.len() {
                    (up, down)
                } else {
                    (down, up)
                };
                let offset = smaller.height() as i64 - larger.height() as i64;
                perform_fold(Coord2::new2d(0, offset), larger, smaller)
            },
            _ => panic!("Regex returned illegal value")
        }
    }
}

fn perform_fold(adjustment: Coord2, larger: Grid2<bool>, smaller: Grid2<bool>) -> Grid2<bool> {
    Grid2::new2d_with_fn(larger.width(), larger.len() / larger.width(), |i| {
        let l_pos = larger.coord_of(i).unwrap();

        if larger[l_pos] {
            true
        } else {
            let s_pos = l_pos 
            false
        }
        
    })
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
    todo!()
}

fn part1(input: &Input) -> Int {
    todo!()
}

fn part2(input: &Input) -> String {
    todo!()
}

#[test]
fn default() {
    let input = get_input(21, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(735, part1(&input));
    assert_eq!("UFRZKAUZ".to_string(), part2(&input));
}
