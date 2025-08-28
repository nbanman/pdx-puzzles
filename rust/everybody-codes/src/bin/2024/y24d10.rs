use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use std::collections::HashSet;
use utilities::structs::{
    coord::Coord,
    stopwatch::{ReportDuration, Stopwatch},
};

type Pos = Coord<usize, 2>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 10);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!(
        "1. {} ({})",
        solve(&input1, |words| words[0].clone()),
        stopwatch.lap().report()
    );
    println!(
        "2. {} ({})",
        solve(&input2, |words| get_power(&words)),
        stopwatch.lap().report()
    );
    println!(
        "3. {} ({})",
        solve(&input3, |words| get_power(&words)),
        stopwatch.lap().report()
    );
    println!("Total: {}", stopwatch.stop().report());
}

fn solve<F, T>(input: &str, output: F) -> T
where
    F: Fn(Vec<String>) -> T,
{
    let mut wall: Vec<Vec<u8>> = get_wall(input);
    let samples: Vec<Sample> = get_samples(&wall);
    deduce_runes(&mut wall, &samples);
    let runic_words: Vec<String> = samples
        .into_iter()
        .map(|sample| get_runic_word(&wall, sample))
        .collect();
    output(runic_words)
}

fn get_wall(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.as_bytes()
                .iter()
                .filter(|&&c| c != b' ')
                .copied()
                .collect()
        })
        .collect()
}

fn get_samples(wall: &[Vec<u8>]) -> Vec<Sample> {
    let measure_row: &Vec<u8> = &wall[2];
    let samples_per_row: usize = measure_row.iter().filter(|&&c| c == b'.').count() / 4;
    let samples_per_col: usize = (0..wall.len())
        .map(|y| wall[y][2])
        .filter(|&c| c == b'.')
        .count()
        / 4;
    let spacing: usize = match measure_row
        .iter()
        .dropping(8)
        .enumerate()
        .find(|&(_, &c)| c == b'.')
    {
        Some((0, _)) => 6,
        Some((_amt, _)) => 8,
        None => 0,
    };
    (0..samples_per_col)
        .cartesian_product(0..samples_per_row)
        .map(|(y, x)| Sample::new(Pos::new2d(x * spacing, y * spacing)))
        .collect()
}

fn deduce_runes(wall: &mut [Vec<u8>], samples: &[Sample]) {
    let mut changed = true;
    while changed {
        changed = false;
        for sample in samples {
            let hz_symbols: Vec<Vec<u8>> = sample
                .hz
                .iter()
                .map(|row| row.iter().map(|pos| wall[pos.y()][pos.x()]).collect())
                .collect();
            let vt_symbols: Vec<Vec<u8>> = sample
                .vt
                .iter()
                .map(|col| col.iter().map(|pos| wall[pos.y()][pos.x()]).collect())
                .collect();

            for rune_spot in (0..4).cartesian_product(0..4) {
                let rune_spot: Pos = Coord::from(rune_spot);
                let rs_concrete = rune_spot + sample.tl;
                let rune = wall[rs_concrete.y()][rs_concrete.x()];
                let hz_symbols = &hz_symbols[rune_spot.y()];
                let vt_symbols = &vt_symbols[rune_spot.x()];

                if !(rune as char).is_ascii_alphabetic() {
                    let hz_set: HashSet<u8> = hz_symbols.iter().copied().collect();
                    let vt_set: HashSet<u8> = vt_symbols.iter().copied().collect();
                    match hz_set
                        .intersection(&vt_set)
                        .find(|c| c.is_ascii_alphabetic())
                    {
                        Some(&intersection) => {
                            wall[rs_concrete.y()][rs_concrete.x()] = intersection;
                            changed = true;
                        }
                        _ => {
                            if hz_symbols.iter().all(|c| c.is_ascii_alphabetic())
                                && vt_symbols
                                    .iter()
                                    .filter(|c| c.is_ascii_alphabetic())
                                    .count()
                                    == 3
                            {
                                let rune_row: HashSet<u8> = (0..4)
                                    .map(|x_offset| wall[rs_concrete.y()][sample.tl.x() + x_offset])
                                    .collect();
                                if rune_row.iter().filter(|c| c.is_ascii_alphabetic()).count() == 3
                                {
                                    let rune = hz_set
                                        .difference(&rune_row)
                                        .find(|c| c.is_ascii_alphabetic());
                                    if let Some(&rune) = rune {
                                        wall[rs_concrete.y()][rs_concrete.x()] = rune;
                                        wall[rs_concrete.y()][rs_concrete.x()] = rune;
                                        let cross = vt_symbols
                                            .iter()
                                            .enumerate()
                                            .find(|&(_, &c)| c == b'?')
                                            .unwrap()
                                            .0;
                                        let wall_pos = &sample.vt[rune_spot.x()][cross];
                                        wall[wall_pos.y()][wall_pos.x()] = rune;
                                        changed = true;
                                    }
                                }
                            } else if vt_symbols.iter().all(|c| c.is_ascii_alphabetic())
                                && hz_symbols
                                    .iter()
                                    .filter(|c| c.is_ascii_alphabetic())
                                    .count()
                                    == 3
                            {
                                let rune_col: HashSet<u8> = (0..4)
                                    .map(|y_offset| wall[sample.tl.y() + y_offset][rs_concrete.x()])
                                    .collect();
                                if rune_col.iter().filter(|c| c.is_ascii_alphabetic()).count() == 3
                                {
                                    let rune = vt_set
                                        .difference(&rune_col)
                                        .find(|c| c.is_ascii_alphabetic());
                                    if let Some(&rune) = rune {
                                        wall[rs_concrete.y()][rs_concrete.x()] = rune;
                                        let cross = hz_symbols
                                            .iter()
                                            .enumerate()
                                            .find(|&(_, &c)| c == b'?')
                                            .unwrap()
                                            .0;
                                        let wall_pos = &sample.hz[rune_spot.y()][cross];
                                        wall[wall_pos.y()][wall_pos.x()] = rune;
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_runic_word(wall: &[Vec<u8>], sample: Sample) -> String {
    (0..4)
        .cartesian_product(0..4)
        .map(|(y, x)| {
            let wall_pos = Pos::from((x, y)) + sample.tl;
            wall[wall_pos.y()][wall_pos.x()] as char
        })
        .collect()
}

fn get_power(words: &[String]) -> usize {
    words
        .iter()
        .map(|word| {
            if word.chars().all(|c| c.is_ascii_alphabetic()) {
                word.chars()
                    .enumerate()
                    .map(|(idx, c)| (c as usize - 64) * (idx + 1))
                    .sum()
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug)]
struct Sample {
    tl: Pos,
    hz: Vec<Vec<Pos>>,
    vt: Vec<Vec<Pos>>,
}

const OFFSETS: [usize; 4] = [0, 1, 6, 7];

impl Sample {
    fn new(tl: Pos) -> Self {
        let hz: Vec<Vec<Pos>> = (2..6)
            .map(|y_offset| {
                OFFSETS
                    .iter()
                    .map(move |x_offset| Pos::new2d(tl.x() + x_offset, tl.y() + y_offset))
                    .collect()
            })
            .collect();
        let vt: Vec<Vec<Pos>> = (2..6)
            .map(|x_offset| {
                OFFSETS
                    .iter()
                    .map(move |y_offset| Pos::new2d(tl.x() + x_offset, tl.y() + y_offset))
                    .collect()
            })
            .collect();
        let tl = tl + 2;
        Self { tl, hz, vt }
    }
}

#[test]
fn tests() {
    let inputs = [
        "**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**",
        "**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**",
    ];
    assert_eq!(
        "PTBVRCZHFLJWGMNS".to_string(),
        solve(inputs[0], |words| words[0].clone())
    );
    assert_eq!(3889, solve(inputs[1], |words| get_power(&words)));
}
