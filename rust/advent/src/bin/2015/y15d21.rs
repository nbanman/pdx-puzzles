use std::cmp::max;
use std::ops::Add;
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

const ITEM_LIST: &str = r"
    Weapons:    Cost  Damage  Armor
    Dagger        8     4       0
    Shortsword   10     5       0
    Warhammer    25     6       0
    Longsword    40     7       0
    Greataxe     74     8       0

    Armor:      Cost  Damage  Armor
    Leather      13     0       1
    Chainmail    31     0       2
    Splintmail   53     0       3
    Bandedmail   75     0       4
    Platemail   102     0       5

    Rings:      Cost  Damage  Armor
    Damage +1    25     1       0
    Damage +2    50     2       0
    Damage +3   100     3       0
    Defense +1   20     0       1
    Defense +2   40     0       2
    Defense +3   80     0       3
";

const HP: u16 = 100;
type Input = (Character, Vec<Stats>);
type Output = u16;

#[derive(Debug, Clone, Copy)]
struct Stats {
    cost: u16,
    damage: u16,
    armor: u16,
}

impl Add for Stats {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            cost: self.cost + other.cost,
            damage: self.damage + other.damage,
            armor: self.armor + other.armor,
        }
    }
}

#[derive(Debug)]
struct Character {
    hp: u16,
    damage: u16,
    armor: u16,
}

impl Character {
    fn rounds_to_kill(&self, other: &Self) -> u16 {
        let adjusted_damage =
            max(1, self.damage.checked_sub(other.armor).unwrap_or_default());
        other.hp / adjusted_damage + if other.hp % adjusted_damage == 0 { 0 } else { 1 }
    }
}

impl From<&Stats> for Character {
    fn from(stats: &Stats) -> Self {
        Self {
            hp: HP,
            damage: stats.damage,
            armor: stats.armor,
        }
    }
}
fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (hp, damage, armor) = input.get_numbers().collect_tuple().unwrap();
    let boss = Character { hp, damage, armor };

    let (weapons, armor, rings) = ITEM_LIST.split("\n\n").collect_tuple().unwrap();
    let armor: Vec<Stats> = armor.get_numbers().tuples()
        .map(|(cost, damage, armor)| Stats { cost, damage, armor })
        .chain(std::iter::once(Stats { cost: 0, damage: 0, armor: 0 }))
        .collect();
    let rings: Vec<Stats> = rings.get_numbers().tuples()
        .map(|(_, cost, damage, armor)| Stats { cost, damage, armor })
        .chain(std::iter::once(Stats { cost: 0, damage: 0, armor: 0 }))
        .collect();

    let loadouts = weapons.get_numbers().tuples()
        .map(|(cost, damage, armor)| Stats { cost, damage, armor })
        .flat_map(|weaponized| armor.iter().map(move |&armor| weaponized + armor))
        .flat_map(|armored| {
            let double_rings = rings[..rings.len() - 1].iter()
                .combinations(2)
                .map(|rings| *rings[0] + *rings[1]);
            rings.iter().copied()
                .chain(double_rings)
                .map(move |be_ringed| be_ringed + armored)
        })
        .sorted_unstable_by_key(|stats| stats.cost)
        .collect();
    (boss, loadouts)
}

fn part1((boss, loadouts): &Input) -> Output {
    loadouts.iter()
        .find(|&stats| {
            let player: Character = stats.into();
            player.rounds_to_kill(&boss) <= boss.rounds_to_kill(&player)
        })
        .unwrap()
        .cost
}

fn part2((boss, loadouts): &Input) -> Output {
    loadouts.iter()
        .rev()
        .find(|&stats| {
            let player: Character = stats.into();
            player.rounds_to_kill(&boss) > boss.rounds_to_kill(&player)
        })
        .unwrap()
        .cost
}

#[test]
fn default() {
    let input = get_input(15, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(91, part1(&input));
    assert_eq!(158, part2(&input));
}

// Input parsed (44μs)
// 1. 91 (7μs)
// 2. 158 (2μs)
// Total: 55μs