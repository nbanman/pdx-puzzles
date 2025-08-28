use std::iter::successors;

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::structs::{
    indexer::Indexer,
    stopwatch::{ReportDuration, Stopwatch},
};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 11);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!(
        "1. {} ({})",
        get_population(&input1, 4, "A"),
        stopwatch.lap().report()
    );
    println!(
        "2. {} ({})",
        get_population(&input2, 10, "Z"),
        stopwatch.lap().report()
    );
    println!(
        "3. {} ({})",
        minmax_population(&input3),
        stopwatch.lap().report()
    );
    println!("Total: {}", stopwatch.stop().report());
}

fn get_population(input: &str, days: usize, start: &str) -> usize {
    // maps the parent termite with child termites
    let generations = get_generations(input, Some(start));

    // define initial population (of one)
    let mut population = vec![0; generations.len()];
    population[0] = 1;

    // commence breeding program!
    breed(population, &generations, days)
}

fn minmax_population(input: &str) -> usize {
    let generations = get_generations(input, None);

    // does the same thing as get_population, except runs it over and over using each termite
    // as the initial seed. Calculate the populations and subtract the min population from the
    // max.
    let (min, max) = (0..generations.len())
        .map(|termite| {
            let mut population = vec![0; generations.len()];
            population[termite] = 1;
            breed(population, &generations, 20)
        })
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

fn get_generations(input: &str, start: Option<&str>) -> Vec<Vec<usize>> {
    // Each termite is assigned a unique ID, starting at 0 and going sequentially
    let mut indexer = Indexer::new();

    // If a "start" termite exists, give it ID 0.
    if let Some(start) = start {
        indexer.assign(start);
    }

    // Parse map using Vecs instead of a slow Hashmap, sorting the outer Vec by ID so that
    // the children's IDs can be looked up by the parent ID.
    input
        .lines()
        .map(|line| {
            let (prev, next) = line.split_once(':').unwrap();
            let id = indexer.get_or_assign_index(prev);
            let children: Vec<_> = next
                .split(',')
                .map(|child| indexer.get_or_assign_index(child))
                .collect();
            (id, children)
        })
        .sorted_unstable()
        .map(|(_, children)| children)
        .collect()
}

fn breed(population: Vec<usize>, generations: &[Vec<usize>], days: usize) -> usize {
    // takes a Vec of Ints, applies the generations rules, and returns it as a Vec of Ints
    let next_gen = |pop: &[usize]| {
        let mut next_gen = vec![0; pop.len()];
        for (termite, &amt) in pop.iter().enumerate() {
            let offspring = generations.get(termite).unwrap();
            for &child in offspring {
                next_gen[child] += amt;
            }
        }
        next_gen
    };

    // runs next_gen for n number of days and returns the total termites at the end of the
    // last day
    successors(Some(population), |pop| Some(next_gen(pop)))
        .take(days + 1)
        .last()
        .unwrap()
        .into_iter()
        .sum()
}

#[test]
fn tests() {
    let tests = [
        "A:B,C
B:C,A
C:A",
        "A:B,C
B:C,A,A
C:A",
    ];
    assert_eq!(8, get_population(tests[0], 4, "A"));
    assert_eq!(268815, minmax_population(tests[1]));
}

// Inputs loaded (43μs)
// 1. 42 (18μs)
// 2. 193253 (24μs)
// 3. 1308907399812 (2ms)
// Total: 2ms
