use advent::utilities::get_input::get_input;
use indexmap::IndexMap;
use lazy_regex::regex;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Output = usize;
type Chemical<'a> = (&'a str, usize);
type Reaction<'a> = (Chemical<'a>, Vec<Chemical<'a>>);
type Upstream<'a> = FxHashMap<&'a str, FxHashSet<Chemical<'a>>>;
type Rules<'a> = FxHashMap<&'a str, Reaction<'a>>;
type Input<'a> = (Upstream<'a>, Rules<'a>);
fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &'_ str) -> Input<'_> {
    let rx = regex!(r"(?<amount>\d+) (?<name>[A-Z]+)");
    let mut lookup = FxHashMap::default();
    let mut rules = FxHashMap::default();

    // build out lookup and rules
    for line in input.lines() {
        let mut chemicals: Vec<Chemical> = rx.captures_iter(line)
            .map(|caps| {
                let name = caps.name("name").unwrap().as_str();
                let amount = caps.name("amount").unwrap().as_str().parse::<usize>().unwrap();
                (name, amount)
            })
            .collect();
        let reaction = (chemicals.pop().unwrap(), chemicals);
        for &(ingredient, _) in reaction.1.iter() {
            lookup.entry(ingredient).or_insert(FxHashSet::default()).insert(reaction.0);
        }
        rules.insert(reaction.0.0, reaction);
    }

    let mut upstream = FxHashMap::default();
    // build out upstream from lookup
    for &name in lookup.keys() {
        build_upstream(name, &mut upstream, &lookup);
    }
    (upstream, rules)
}

fn build_upstream<'a>(
    ingredient: &'a str,
    upstream: &mut Upstream<'a>,
    lookup: &Upstream<'a>,
) -> Option<FxHashSet<Chemical<'a>>> {
    if let Some(immediate_up) = lookup.get(ingredient) {
        let mut up_set = immediate_up.clone();
        for &(next, _) in immediate_up.iter() {
            if let Some(downstream) = build_upstream(next, upstream, lookup) {
                up_set.extend(downstream.into_iter());
            }
        }
        upstream.insert(ingredient, up_set.clone());
        Some(up_set)           
    } else {
        None
    }
}

fn calculate_ore<'a>(fuel: usize, upstream: &Upstream<'a>, rules: &Rules<'a>) -> Output {
    let mut ore = 0;
    let mut repository: IndexMap<&'a str, usize> = IndexMap::new();
    repository.insert("FUEL", fuel);
    let mut potentials = repository.clone();
    while let Some((&potential_name, &potential_amount)) = potentials.iter()
        .find(|&(&potential_name, _)| {
            let &((chemical_name, _), _) = rules.get(potential_name).unwrap();
            if let Some(upstream_chemicals) = upstream.get(chemical_name) {
                upstream_chemicals.iter().all(|&(up_chem_name, _)| {
                    !potentials.contains_key(up_chem_name)
                })
            } else {
                true
            }
        })
    {
        // println!("Repository:");
        // for (&k, &v) in repository.iter().sorted_unstable_by_key(|&(&k, _)| k) {
        //     println!("{}={}", k, v);
        // }
        // println!("Potentials: {:?}", potentials);
        let (rule_result, rule_ingredients) = rules.get(potential_name).unwrap();
        let times_applied = (potential_amount / rule_result.1) +
            if potential_amount % rule_result.1 > 0 { 1 } else { 0 };
        let new_amount = potential_amount.checked_sub(rule_result.1 * times_applied).unwrap_or_default();
        repository.insert(potential_name, new_amount);

        for &(ingredient_name, ingredient_amount) in rule_ingredients.iter() {
            let new_amount = repository.get(ingredient_name).unwrap_or(&0) + ingredient_amount * times_applied;
            repository.insert(ingredient_name, new_amount);
        }

        ore += repository.get("ORE").unwrap_or(&0);
        repository.insert("ORE", 0);
        potentials = repository.iter()
            .filter(|&(_, &v)| v != 0)
            .map(|(&k, &v)| (k, v))
            .collect();
    }
    ore
}

fn part1(input: &Input) -> Output {
    let (upstream, rules) = input;
    calculate_ore(1, upstream, rules)
}

fn part2(input: &Input) -> Output {
    let (upstream, rules) = input;
    let one_fuel = calculate_ore(1, upstream, rules);
    let total_ore = 1_000_000_000_000;
    let lower_bound = total_ore / one_fuel;
    let test = calculate_ore(lower_bound, upstream, rules);
    (lower_bound * total_ore) / test
}

#[test]
fn default() {
    let input = get_input(19, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(751038, part1(&input));
    assert_eq!(2074843, part2(&input));
}

// Input parsed (824μs)
// 1. 751038 (105μs)
// 2. 2074843 (191μs)
// Total: 1ms