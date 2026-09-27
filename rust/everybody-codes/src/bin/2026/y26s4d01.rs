use std::ops::RangeInclusive;

use everybody_codes::utilities::inputs::get_story_inputs_provisional;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs_provisional(26, 4, 1);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    }
    if let Some(input2) = input2 {
        println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    }
    if let Some(input3) = input3 {
        println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn solve<F>(input: Input, f: F) -> u64
where
    F: Fn(usize, usize, &Vec<bool>) -> usize,
{
    input.lines()
        .map(|line| {
            let mut number_line = vec![false; 1000];
            number_line[0] = true;
            let mut end = 0usize;
            for jump in line.get_numbers::<usize>() {
                end = f(jump, end, &number_line);
                number_line[end] = true;
            }
            end
        })
        .sum::<usize>() as u64
    
}

fn part1(input: Input) -> u64 {
    solve(input, |jump, end, number_line| {
        if end >= jump && !number_line[end - jump] {
            end - jump
        } else {
            end + jump
        }
    })
}

fn part2(input: Input) -> u64 {
    solve(input, |jump, end, number_line| {
        if end >= jump && !number_line[end - jump] {
            end - jump
        } else {
            (0usize..).find(|&n| !number_line[end + jump + n])
                .map(|n| end + jump + n)
                .unwrap()
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
enum Arc {
    Free,
    Up(RangeInclusive<usize>),
    Down(RangeInclusive<usize>),
    Both { up: RangeInclusive<usize>, down: RangeInclusive<usize> },
}

fn part3(input: Input) -> u64 {
    input.lines()
        .map(|line| {
            let mut number_line = vec![Arc::Free; 1000];
            let mut end = 0;
            let mut bottom = false; // gets toggled right away
            'outer: for jump in line.get_numbers::<usize>() {
                bottom = !bottom;
                // try to go backwards
                'backwards: {
                    // jumping before 0
                    if end < jump {
                        break 'backwards;
                    }

                    // check if landing zone is clear
                    let proposed = end - jump;
                    if number_line[proposed] != Arc::Free {
                        break 'backwards;
                    }

                    // go through all the intervening number locations to see if there arcs that cross
                    for interim in ((end - jump + 1)..end).rev() {
                        match &number_line[interim] {
                            Arc::Free => { },
                            Arc::Both { up: up_rng, down: down_rng } => {
                                if bottom {
                                    if *down_rng.end() > end || *down_rng.start() < proposed {
                                        break 'backwards;
                                    }
                                } else {
                                    if *up_rng.end() > end || *up_rng.start() < proposed {
                                        break 'backwards;
                                    }
                                }
                            },
                            Arc::Up(_) | Arc::Down(_) => unreachable!()
                        }
                    }

                    // if made it this far without breaking, it's a valid landing spot!
                    number_line[end] = match &number_line[end] {
                        Arc::Up(up_rng) => Arc::Both { up: up_rng.clone(), down: proposed..=end },
                        Arc::Down(down_rng) => Arc::Both { up: proposed..=end, down: down_rng.clone() },
                        _ => unreachable!(),
                    };
                    number_line[proposed] = if bottom {
                        Arc::Down(proposed..=end)
                    } else {
                        Arc::Up(proposed..=end)
                    };
                    end = proposed;
                    continue 'outer;
                }
                
                // if backwards failed, try to go forwards
                let min_jump = end + jump;
                let mut interim = end;
                'forwards: loop {
                    interim += 1;
                    match &number_line[interim] {
                        Arc::Free => {
                            if interim >= min_jump {
                                // if made it this far without breaking, it's a valid landing spot!
                                number_line[end] = match &number_line[end] {
                                    Arc::Up(up_rng) => Arc::Both { up: up_rng.clone(), down: end..=interim },
                                    Arc::Down(down_rng) => Arc::Both { up: end..=interim, down: down_rng.clone() },
                                    Arc::Free => {
                                        if bottom {
                                            Arc::Down(end..=interim)
                                        } else {
                                            Arc::Up(end..=interim)
                                        }
                                    },
                                    _ => unreachable!(),
                                };
                                number_line[interim] = if bottom {
                                    Arc::Down(end..=interim)
                                } else {
                                    Arc::Up(end..=interim)
                                };
                                end = interim;
                                continue 'outer;
                            }
                        },
                        Arc::Both { up: up_rng, down: down_rng } => {
                            if bottom {
                                if *down_rng.start() < end {
                                    // this means that any forward loop would cross, so this whole jump is impossible.
                                    // flip the arc because 'outer will flip it back
                                    bottom = !bottom;
                                    continue 'outer;
                                }
                                if *down_rng.end() >= interim {
                                    // this means that forward loop has to be bigger than the end of this to prevent
                                    // crossing, so skip interim to beyond this point
                                    interim = *down_rng.end();
                                    continue 'forwards;
                                }
                            } else {
                                if *up_rng.start() < end {
                                    // this means that any forward loop would cross, so this whole jump is impossible.
                                    // flip the arc because 'outer will flip it back
                                    bottom = !bottom;
                                    continue 'outer;
                                }
                                if *up_rng.end() >= interim {
                                    // this means that forward loop has to be bigger than the end of this to prevent
                                    // crossing, so skip interim to beyond this point
                                    interim = *up_rng.end();
                                    continue 'forwards;
                                }
                            }
                        },
                        Arc::Up(_) | Arc::Down(_) => unreachable!(),
                    }
                }               
            }
            end
        })
        .sum::<usize>() as u64
}

#[test]
fn default() {
    // let (input1, input2, input3) =
    //        everybody_codes::utilities::inputs::get_story_inputs(26, 4, 1);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn example() {
    let input1 = r"1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30";
    assert_eq!(66, part1(input1));
    let input2 = r"1,1,1,1,1
5,1,2,3,4,5,1,2,3,4
2,1,1,2,1,1,2,1,1,2,1,1
5,1,2,1,2,7,1,2,1,2,7,1,2,1,2";
    assert_eq!(34, part1(input2));
    assert_eq!(43, part2(input2));
    assert_eq!(27, part3(input2));
    let input3 = r"5,3,1,1
5,3,1,1,5,1,1,3,4,8,1,1
5,3,1,1,5,1,1,3,4,8,2,1
10,9,9,8,8,7,7,6,6,5,5,4,4,3,3,2,2,1";
    assert_eq!(35, part3(input3));
}
