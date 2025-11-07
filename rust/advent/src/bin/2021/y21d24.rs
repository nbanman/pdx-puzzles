/**
 * Input boils down to an 18 line function run 14 times. The function has two parameters, x and y, and
 * the algorithm depends on whether x is positive. 7 of these have a positive x value and 7 have a
 * negative x value. When x is positive, z := 26 * z + y + the supplied digit. Thus, z increases. When x
 * is negative, if (z % 26) + x equals the supplied digit, z := z / 26, rounded down to nearest integer. If
 * it doesn't equal the supplied digit, z becomes an even larger number.
 *
 * In order for the final value to be 0, every step with a negative x must result in division by 26. Thus,
 * when such a step is encountered, the z value must be such that (z % 26) + x is a number between 1..9. We
 * use a stack to keep track of what z will have to be for the negative x steps. We pair the z-increasing
 * functions with the z-decreasing functions, and find shared values that will satisfy both.
 */

use std::cmp::{max, min};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<PairedSteps>;
type Output = i64;

struct PairedSteps {
    push: Step,
    pop: Step,
}

struct Step {
    order: usize,
    x: i64,
    y: i64,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let steps = input.lines().chunks(18).into_iter().enumerate()
        .map(|(index, lines)| {
            let lines = lines.collect_vec();
            let x_inc = lines[5];
            let x_inc: i64 = x_inc[x_inc.as_bytes().iter().rposition(|&b| b == b' ').unwrap() + 1 ..]
                .parse()
                .unwrap();
            let y_inc = lines[15];
            let y_inc: i64 = y_inc[y_inc.as_bytes().iter().rposition(|&b| b == b' ').unwrap() + 1 ..]
                .parse()
                .unwrap();
            Step { order: index, x: x_inc, y: y_inc }
        })
        .collect_vec();

    let mut paired_steps = Vec::new();
    let mut order_stack: Vec<Step> = Vec::new();

    for step in steps {
        if step.x >= 0 {
            order_stack.push(step);
        } else {
            paired_steps.push(PairedSteps { push: order_stack.pop().unwrap(), pop: step })
        }
    }
    
    paired_steps.sort_by_key(|it| it.push.order);
    paired_steps
}

fn solve<F>(paired_steps: &Input, find_intersection: F) -> Output
where
    F: Fn(i64, i64) -> i64,
{
    let mut z = 0;
    let mut model_number = vec![0; paired_steps.len() * 2];

    for step in paired_steps {
        let increase_z = step.push.y + 26 * z;
        let push_max = (increase_z + 9) % 26 ;
        let pop_max = -step.pop.x + 9;

        let intersection = find_intersection(push_max, pop_max);

        model_number[step.push.order] = 9 - (push_max - intersection);
        model_number[step.pop.order] = 9 - (pop_max - intersection);
        
        z = increase_z + model_number[step.push.order];
    }

    model_number.into_iter().reduce(|acc, n| acc * 10 + n).unwrap()
}

fn part1(paired_steps: &Input) -> Output {
    solve(paired_steps, |push_max, pop_max| min(push_max, pop_max))
}

fn part2(paired_steps: &Input) -> Output {
    solve(paired_steps, |push_max, pop_max| max(push_max, pop_max) - 8)
}

#[test]
fn default() {
    let input = get_input(21, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(92969593497992, part1(&input));
    assert_eq!(81514171161381, part2(&input));
}

// Input parsed (29μs)
// 1. 92969593497992 (9μs)
// 2. 81514171161381 (2μs)
// Total: 44μs