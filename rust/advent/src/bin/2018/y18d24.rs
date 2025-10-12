use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::{regex, Regex};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (Vec<UnitType>, Vec<UnitType>);
type Output = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Team { ImmuneSystem, Infection }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DamageType {
    Slashing,
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
}

impl From<&str> for DamageType {
    fn from(value: &str) -> Self {
        match value {
            "slashing" => Self::Slashing,
            "bludgeoning" => Self::Bludgeoning,
            "cold" => Self::Cold,
            "fire" => Self::Fire,
            "radiation" => Self::Radiation,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct UnitType {
    id: usize,
    team: Team,
    units: usize,
    hp: usize,
    weaknesses: Option<Vec<DamageType>>,
    immunities: Option<Vec<DamageType>>,
    damage: usize,
    damage_type: DamageType,
    initiative: usize,
}
impl UnitType {
    fn effective_power(&self, boost: usize, unit_amounts: &[usize]) -> usize {
        unit_amounts[self.id] * (self.damage + if self.team == Team::ImmuneSystem { boost } else { 0 })
    }

    fn attack(&self, defender: &UnitType, unit_amounts: &mut [usize], boost: usize) -> bool {
        let damage_modifier = match &defender.weaknesses {
            Some(weaknesses) => {
                if weaknesses.contains(&self.damage_type) { 2 } else { 1 } 
            },
            None => 1,
        };
        let damage = self.effective_power(boost, unit_amounts) * damage_modifier;
        let defender_units = unit_amounts.get_mut(defender.id).unwrap();
        let removed_units = damage / defender.hp;
        *defender_units = defender_units.checked_sub(removed_units).unwrap_or_default();
        removed_units > 0
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (immune, infection) = input.split_once("\n\n").unwrap();
    let mut id = 0;
    let immune = process_army(immune, Team::ImmuneSystem, &mut id);
    let infection = process_army(infection, Team::Infection, &mut id);
    (immune, infection)
}

fn process_army(haystack: &str, team: Team, id: &mut usize) -> Vec<UnitType> {
    let rx = regex!(r"(?<units>\d+) units each with (?<hp>\d+) hit points (?:\((?<modifiers>[^)]+)\) )?with an attack that does (?<damage>\d+) (?<damage_type>[a-z]+) damage at initiative (?<initiative>\d+)");
    let rx_weaknesses = regex!(r"weak to ((?:[a-z]+(?:, )?)+)");
    let rx_immunities= regex!(r"immune to ((?:[a-z]+(?:, )?)+)");
    let army = rx.captures_iter(haystack)
        .map(|caps| {
            let units: usize = caps.name("units").unwrap().as_str().parse().unwrap();
            let hp: usize = caps.name("hp").unwrap().as_str().parse().unwrap();
            let damage: usize = caps.name("damage").unwrap().as_str().parse().unwrap();
            let initiative: usize = caps.name("initiative").unwrap().as_str().parse().unwrap();
            let modifiers = caps.name("modifiers").map(|mod_match| mod_match.as_str());
            let weaknesses = process_damage_type(rx_weaknesses, modifiers);
            let immunities = process_damage_type(rx_immunities, modifiers);
            let damage_type = DamageType::from(caps.name("damage_type").unwrap().as_str());
            let unit_type =
                UnitType { id: *id, team, units, hp, weaknesses, immunities, damage, damage_type, initiative };
            *id += 1;
            unit_type
        })
        .collect();
    army
}

fn process_damage_type(rx: &Regex, haystack: Option<&str>) -> Option<Vec<DamageType>> {
    let haystack = haystack?;
    rx
        .captures(haystack)
        .map(|caps| {
            let weak_str = caps.get(1).unwrap().as_str();
            weak_str.split(", ")
                .map(|s| DamageType::from(s))
                .collect()
        })
}

fn select_targets<'a>(
    attackers: &[&'a UnitType],
    defenders: &Vec<&'a UnitType>,
) -> Vec<(&'a UnitType, &'a UnitType)> {
    // take all defenders and rank them by their effective power, with initiative as a tiebreaker
    let mut defenders: Vec<&UnitType> = defenders.clone();

    // assign a defender to each attacker as follows:
    // all the tiebreakers are already sorted, so start at the top and grab the first defender who is
    // weak to attacker's damage type. If none such defender exists, grab the first defender who is
    // not immune to attacker's damage type.
    // Assuming such a defender is found, add the attacker/defender pair and remove the defender for
    // future consideration by other attackers.let mut attacker_selections = Vec::new();
    let mut attacker_selections = Vec::new();
    for &attacker in attackers {
        let matched_defender = defenders.iter().enumerate()
            .find(|&(_, &defender)| {
                defender.weaknesses.as_ref()
                    .map(|weaknesses| weaknesses.contains(&attacker.damage_type))
                    .unwrap_or_default()
            })
            .or_else(|| defenders.iter().enumerate()
                .find(|&(_, &defender)| {
                    defender.immunities.as_ref()
                        .map(|immunities| !immunities.contains(&attacker.damage_type))
                        .unwrap_or(true)
                })
            );
        if let Some((idx, &defender)) = matched_defender {
            attacker_selections.push((attacker, defender));
            defenders.remove(idx);
        }
    }
    attacker_selections
}

fn compare_units(a: &UnitType, b: &UnitType, boost: usize, unit_amounts: &[usize]) -> std::cmp::Ordering {
    b.effective_power(boost, unit_amounts)
        .cmp(&a.effective_power(boost, unit_amounts))
        .then(b.initiative.cmp(&a.initiative))
}

fn solve (input: &Input, starting_boost: usize, predicate: fn(bool) -> bool) -> Output {
    let (initial_immune, initial_infection) = input;
    let initial_immune: Vec<&UnitType> = initial_immune.iter().collect();
    let initial_infection: Vec<&UnitType> = initial_infection.iter().collect();
    let initial_unit_amounts: Vec<usize> = initial_immune.iter()
        .chain(initial_infection.iter())
        .map(|unit| unit.units)
        .collect();
    let mut boost = starting_boost;

    'outer:loop {
        // reset units
        let mut immune: Vec<&UnitType> = initial_immune.clone();
        let mut infection: Vec<&UnitType> = initial_infection.clone();
        let mut unit_amounts: Vec<usize> = initial_unit_amounts.clone();

        while !immune.is_empty() && !infection.is_empty() {
            // target selection phase
            immune.sort_unstable_by(|a, b| compare_units(a, b, boost, &unit_amounts));
            infection.sort_unstable_by(|a, b| compare_units(a, b, boost, &unit_amounts));
            let immune_selections = select_targets(&immune, &infection);
            let infection_selections = select_targets(&infection, &immune);

            // attack phase
            let attackers = immune_selections.into_iter()
                .chain(infection_selections.into_iter())
                .sorted_unstable_by(|&(a, _), &(b, _)| b.initiative.cmp(&a.initiative));

            // changed tracks whether a defender actually loses units in a turn. if this stays false,
            // we know it's an unchanging loop and we can exit early.
            let mut changed = false;
            for (attacker, defender) in attackers {
                changed = changed | attacker.attack(defender, &mut unit_amounts, boost);
            }

            // cleanup phase
            immune = immune.into_iter().filter(|it| unit_amounts[it.id] > 0).collect();
            infection = infection.into_iter().filter(|it| unit_amounts[it.id] > 0).collect();

            // exit early if no units were killed
            if !changed {
                boost += 1;
                continue 'outer;
            }
        }
        if predicate(infection.is_empty()) {
            return unit_amounts.iter().sum();
        }
        boost += 1;
    }
}

fn part1(input: &Input) -> Output {
    solve(input, 0, |_| true)
}
fn part2(input: &Input) -> Output {
    solve(input, 1, |is_empty| is_empty)
}

#[test]
fn default() {
    let input = get_input(18, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(15165, part1(&input));
    assert_eq!(4037, part2(&input));
}

// Input parsed (889μs)
// 1. 15165 (189μs)
// 2. 4037 (10ms)
// Total: 11ms