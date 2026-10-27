use everybody_codes::utilities::inputs::get_story_inputs_provisional;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs_provisional(26, 4, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!(
            "1. {} ({})",
            part1(&input1),
            stopwatch.lap().report()
        );
    }
    if let Some(input2) = input2 {
        println!(
            "2. {} ({})",
            part2(&input2),
            stopwatch.lap().report()
        );
    }
    if let Some(input3) = input3 {
        println!(
            "3. {} ({})",
            part3(&input3),
            stopwatch.lap().report()
        );
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    solve(input, |iso| iso.into_iter().sum())
}

fn part2(input: Input) -> usize {
    solve(input, |iso| iso.into_iter().max().unwrap())
}

fn part3(input: Input) -> usize { part2(input) }

fn solve<F>(input: Input, f: F) -> usize
where
    F: Fn([usize; 2]) -> usize,
{
    // parse input
    let (width, height, hz, vt) = input
        .lines()
        .map(|line| line.rsplit_once('=').unwrap().1)
        .collect_tuple()
        .unwrap();
    let width: usize = width.parse().unwrap();
    let height: usize = height.parse().unwrap();

    // double the offsets, because due to parity issues, an odd length will not repeat exactly
    let offset: fn(&str) -> Vec<usize> = |s: &str| s
        .repeat(2)
        .as_bytes()
        .iter()
        .map(|&b| b as usize - 48)
        .collect();
    let hz = offset(hz);
    let vt = offset(vt);

    let rows = hz.len();
    let cols = vt.len();

    let mut row_parity = 0;
    let mut total = [0; 2];

    for y in 0..rows.min(height) {
        row_parity ^= usize::from(y > 0 && hz[y] == 0);
        let mut col_parity = 0;

        // if hz offsets are 11 or 00, twin vertical stitches can be drawn
        if hz[y] == hz[(y + 1) % rows] {
            for x in 0..cols.min(width) {
                col_parity ^= usize::from(x > 0 && vt[x] == y & 1);

                if hz[y] == x & 1
                    && vt[x] == y & 1
                    && vt[(x + 1) % cols] == y & 1
                {
                    let repeat_x = (width - x).div_ceil(cols);
                    let repeat_y = (height - y).div_ceil(rows);
                    total[row_parity ^ col_parity] += repeat_x * repeat_y;
                }
            }
        }
    }
    f(total)
}

#[test]
fn default() {
    let (input1, input2, input3) =
           everybody_codes::utilities::inputs::get_story_inputs(26, 4, 3);
    assert_eq!(290, part1(&input1));
    assert_eq!(10490, part2(&input2));
    assert_eq!(609374314218696, part3(&input3));
}

#[test]
fn example() {
    let input1 = r"width=30
height=10
horizontal-offsets=10011
vertical-offsets=11011";
    assert_eq!(27, part1(input1));
    assert_eq!(15, part2(input1));
    let input2 = r"width=40
height=12
horizontal-offsets=11100
vertical-offsets=001101";
    assert_eq!(7, part2(input2));
    let input3 = r"width=100
height=70
horizontal-offsets=111101111101101111000100100110
vertical-offsets=110100001110111011101000001111";
    assert_eq!(269, part2(input3));
}

// Input parsed (24μs)
// 1. 290 (7μs)
// 2. 10490 (3μs)
// 3. 609374314218696 (31μs)
// Total: 68μs