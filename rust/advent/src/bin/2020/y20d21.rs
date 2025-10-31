use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (Vec<&'a str>, FxHashMap<&'a str, &'a str>);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    let mut ingredients: Vec<&str> = Vec::new();
    let mut allergens: FxHashMap<&str, &str> = FxHashMap::default();
    let mut lookup: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();

    for line in input.lines() {
        let (istr, astr) = line.split_once(" (contains ").unwrap();
        let food: FxHashSet<&str> = istr.split(' ').collect();
        ingredients.extend(food.iter().copied());

        for allergen in astr.split([',', ')', ' ']).filter(|&s| s != "") {
            let potential_ingredients = lookup.entry(allergen)
                .or_insert(FxHashSet::default());
            if potential_ingredients.is_empty() {
                *potential_ingredients = food.clone();
            } else {
                *potential_ingredients = potential_ingredients.intersection(&food).copied().collect();
            }
        }
    }

    let mut to_remove = Vec::new();
    while !lookup.is_empty() {
        for (&allergen, potential_ingredients) in lookup.iter() {
            let remaining = potential_ingredients.iter()
                .filter(|&&it| !allergens.contains_key(it))
                .copied()
                .collect::<Vec<_>>();
            if remaining.len() == 1 {
                allergens.insert(allergen, remaining[0]);
                to_remove.push((allergen, remaining[0]));
            }
        }
        for (allergen, ingredient) in to_remove.drain( .. ) {
            lookup.remove(&allergen);
            for potential_ingredients in lookup.values_mut() {
                potential_ingredients.remove(&ingredient);
            }
        }
    }

    (ingredients, allergens)
}

fn part1((ingredients, allergens): &Input) -> usize {
    let allergens: FxHashSet<&str> = allergens.values().copied().collect();
    ingredients.iter()
        .filter(|&&ingredient| !allergens.contains(ingredient))
        .count()
}

fn part2((_, allergens): &Input) -> String {
    allergens.iter()
        .sorted_unstable()
        .map(|(|_, &v)| v)
        .join(",")
}

#[test]
fn default() {
    let input = get_input(20, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(2493, part1(&input));
    assert_eq!("kqv,jxx,zzt,dklgl,pmvfzk,tsnkknk,qdlpbt,tlgrhdh".to_string(), part2(&input));
}

// Input parsed (188μs)
// 1. 2493 (22μs)
// 2. kqv,jxx,zzt,dklgl,pmvfzk,tsnkknk,qdlpbt,tlgrhdh (3μs)
// Total: 216μs
