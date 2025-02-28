use everybody_codes::utilities::inputs::get_inputs;
use utilities::{
    parsing::{get_numbers::ContainsNumbers, try_get::TryGet},
    structs::stopwatch::{ReportDuration, Stopwatch},
};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 9);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: &str) -> usize {
    let stamps: Vec<usize> = [1, 3, 5, 10].into_iter().rev().collect();
    let brightnesses = get_brightnesses(input);
    let &brightest = brightnesses.iter().max().unwrap();
    let mut cache: Vec<Option<usize>> = vec![None; brightest + 1];
    solve(&brightnesses, &stamps, &mut cache)
}

fn part2(input: &str) -> usize {
    let stamps: Vec<usize> = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30]
        .into_iter()
        .rev()
        .collect();
    let brightnesses = get_brightnesses(input);
    let &brightest = brightnesses.iter().max().unwrap();
    let mut cache: Vec<Option<usize>> = vec![None; brightest + 1];
    solve(&brightnesses, &stamps, &mut cache)
}

fn part3(input: &str) -> usize {
    let stamps: Vec<usize> = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ]
    .into_iter()
    .rev()
    .collect();
    let brightnesses = get_brightnesses(input);
    let &brightest = brightnesses.iter().max().unwrap();
    let mut cache: Vec<Option<usize>> = vec![None; brightest + 1];
    for stamp in stamps.iter() {
        cache[*stamp] = Some(1);
    }
    brightnesses
        .iter()
        .map(|brightness| {
            let half = brightness / 2;
            let limit = if brightness & 1 == 1 { 49 } else { 50 };
            (0..=limit)
                .map(|n| {
                    let ba = half - n;
                    let bb = (brightness - half) + n;
                    let ba_beetles = if let Some(ba_beetles) = cache[ba] {
                        ba_beetles
                    } else {
                        get_beetles(ba, usize::MAX, &mut cache, &stamps)
                    };
                    let bb_beetles = if let Some(bb_beetles) = cache[bb] {
                        bb_beetles
                    } else {
                        get_beetles(bb, usize::MAX, &mut cache, &stamps)
                    };
                    ba_beetles + bb_beetles
                })
                .min()
                .unwrap()
        })
        .sum()
}

fn get_brightnesses(input: &str) -> Vec<usize> {
    input.get_numbers().collect()
}

fn solve(brightnesses: &[usize], stamps: &[usize], cache: &mut Vec<Option<usize>>) -> usize {
    for stamp in stamps {
        cache[*stamp] = Some(1);
    }
    brightnesses
        .iter()
        .map(|brightness| {
            if let Some(beetles) = cache[*brightness] {
                beetles
            } else {
                get_beetles(*brightness, usize::MAX, cache, stamps)
            }
        })
        .sum()
}

fn get_beetles(
    remaining: usize,
    prev_best: usize,
    cache: &mut Vec<Option<usize>>,
    stamps: &[usize],
) -> usize {
    if let Some(count) = cache[remaining] {
        return count;
    }

    // otherwise, carry on
    let mut best = prev_best;

    // iterate through each stamp in descending order
    'outer: for (idx, &stamp) in stamps.iter().enumerate() {
        let beetles = remaining / stamp;
        // if the best possible outcome requires more than the current best, abort
        if beetles >= best {
            return best;
        }

        // if divides cleanly, we have our answer, update cache and pop up with it
        if remaining % stamp == 0 {
            // may not need first conditional
            if cache[remaining].is_none() {
                cache[remaining] = Some(beetles);
            }
            return beetles;
        }

        // otherwise try decreasing amounts of that stamp
        // only runs if the stamp is smaller than the remaining brightness
        for n in (1..=beetles).rev() {
            // there should be break here where we know not to continue because anything
            // remaining will result in something bigger than current best.
            // maybe get the next stamp and do the calculation there...
            if let Some(&next_stamp) = stamps.try_get(idx + 1) {
                let next_stamp_beetles =
                    ((remaining - n * stamp) as f32 / next_stamp as f32).ceil() as usize;
                if n + next_stamp_beetles >= best {
                    break 'outer;
                }
            }
            // this should be the best obtainable with n number of stamp
            if best <= n {
                continue;
            }
            let result = n + get_beetles(remaining - n * stamp, best - n, cache, &stamps[idx..]);
            if result < best {
                best = result;
            }
        }
    }
    cache[remaining] = Some(best);
    best
}

// #[test]
// fn tests() {
//     let tests = [r"2, 4, 7, 16", "27", "156488, 352486, 546212"];
//     let stamps = [vec![1, 3, 5, 10], vec![1, 3, 9, 10], vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101]];
//     assert_eq!(10, part1(tests[0], &stamps[0]));
//     assert_eq!(3, solve(tests[1], &stamps[1]));
//     assert_eq!(10449, part3(tests[2], &stamps[2]));
// }
