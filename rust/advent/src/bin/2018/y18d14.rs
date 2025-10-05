use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 14).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_recipes<F>(predicate: F) -> Vec<usize>
where
    F: Fn(&Vec<usize>) -> bool,
{
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    while !predicate(&recipes) {
        let new1 = recipes[elf1];
        let new2 = recipes[elf2];

        let sum = new1 + new2;
        if sum > 9 {
            recipes.push(1);
            recipes.push(sum - 10);
        } else {
            recipes.push(sum);
        }
        elf1 = (elf1 + new1 + 1) % recipes.len();
        elf2 = (elf2 + new2 + 1) % recipes.len();
    }

    recipes
}

fn part1(input: Input) -> Output {
    let plan: usize = input.trim_end().parse().unwrap();
    let recipes = get_recipes(|recipes| recipes.len() == plan + 10);
    recipes[recipes.len() - 10..]
        .iter()
        .fold(0, |acc, &i| acc * 10 + i)
}

fn part2(input: Input) -> Output {
    let plan: Vec<usize> = input.trim_end().chars().map(|c| (c as u8 - b'0') as usize).collect();
    let recipes = get_recipes(|recipes| {
        recipes.len() > plan.len()
            && (&plan == &recipes[recipes.len() - plan.len() - 1..recipes.len() - 1]
                || &plan == &recipes[recipes.len() - plan.len()..recipes.len()])
    });
    if &plan == &recipes[recipes.len() - plan.len()..recipes.len()] {
        recipes.len() - plan.len()
    } else {
        recipes.len() - plan.len() - 1
    }
}

#[test]
fn default() {
    let input = get_input(18, 14).unwrap();
    assert_eq!(4910101614, part1(&input));
    assert_eq!(20253137, part2(&input));
}

// Input parsed (14μs)
// 1. 4910101614 (6ms)
// 2. 20253137 (146ms)
// Total: 152ms
