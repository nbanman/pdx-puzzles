use std::{cmp::max, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (Int, Int);
type Output = Int;
type Int = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana(&self) -> Int {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn duration(&self) -> Int {
        match self {
            Spell::MagicMissile => 1,
            Spell::Drain => 1,
            Spell::Shield => 6,
            Spell::Poison => 6,
            Spell::Recharge => 5,
        }
    }

    fn effect(&self) -> Int {
        match self {
            Spell::MagicMissile => 4,
            Spell::Drain => 2,
            Spell::Shield => 7,
            Spell::Poison => 3,
            Spell::Recharge => 101,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct State {
    player_hp: Int,
    boss_hp: Int,
    current_mana: Int,
    mana_spent: Int,
    shield: Int,
    poison: Int,
    recharge: Int,
}

impl State {
    fn available_mana(&self) -> Int {
        self.current_mana + if self.recharge > 0 { Spell::Recharge.effect() } else { 0 }
    }

    fn turn(&self, spell: Spell, damage: Int, constant_drain: Int) -> Self {
        // If player dies from constant drain, boss does not sustain any damage, so return early
        if self.player_hp - constant_drain == 0 {
            return Self { player_hp: 0, ..*self }
        }

        let mut new_player_hp = self.player_hp +
            (if spell == Spell::Drain { Spell::Drain.effect() } else { 0 }) -
            constant_drain;
        let mut new_boss_hp = self.boss_hp -
            if self.poison > 0 { Spell::Poison.effect() } else { 0 } -
            if spell == Spell::MagicMissile { spell.effect() } else { 0 } -
            if spell == Spell::Drain { spell.effect() } else { 0 };
        let mut new_current_mana = self.current_mana - spell.mana() +
            if self.recharge > 0 { Spell::Recharge.effect() } else { 0 };
        let new_mana_spent = self.mana_spent + spell.mana();
        let new_shield = if spell == Spell::Shield { spell.duration() } else { self.shield - 1 };
        let new_poison = if spell == Spell::Poison { spell.duration() } else { self.poison - 1 };
        let new_recharge = if spell == Spell::Recharge { spell.duration() } else { self.recharge - 1 };

        if new_player_hp > 0 {
            let armor = if new_shield > 0 { Spell::Shield.effect() } else { 0 };
            new_player_hp -= max(damage - armor, 1);
            new_boss_hp -= if new_poison > 0 { Spell::Poison.effect() } else { 0 };
            new_current_mana += if new_recharge > 0 { Spell::Recharge.effect() } else { 0 };
        }

        Self {
            player_hp: new_player_hp,
            boss_hp: new_boss_hp,
            current_mana: new_current_mana,
            mana_spent: new_mana_spent,
            shield: new_shield - 1,
            poison: new_poison - 1,
            recharge: new_recharge - 1,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mana_spent.cmp(&other.mana_spent)
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect_tuple().unwrap()
}

fn solve((boss_hp, boss_damage): Input, constant_drain: Int) -> Output {
    let mut q = BinaryHeap::new();
    q.push(State { player_hp: 50, boss_hp, current_mana: 500, mana_spent: 0, shield: 0, poison: 0, recharge: 0 });

    let mut lowest_mana_win = Int::MAX;

    while let Some(current) = q.pop() {
        let next_states = [Spell::MagicMissile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge]
            .into_iter()
            .filter(|spell| {
                if current.available_mana() < spell.mana() {
                    false
                } else {
                    match spell {
                        Spell::Shield => current.shield <= 1,
                        Spell::Poison => current.poison <= 1,
                        Spell::Recharge => current.recharge <= 1,
                        _ => true
                    }
                }
            })
            .map(|spell| {
                current.turn(spell, boss_damage, constant_drain)
            })
            .filter(|state| {
                if state.boss_hp <= 0 {
                    if state.mana_spent < lowest_mana_win {
                        lowest_mana_win = state.mana_spent;
                    }
                    false
                } else if state.player_hp <= 0 || state.available_mana() < 53 || state.mana_spent > lowest_mana_win {
                    false
                } else {
                    true
                }
            })
            .collect_vec();
        for next in next_states {
            q.push(next);
        }
    }
    lowest_mana_win
}

fn part1(input: Input) -> Output {
    solve(input, 0)
}

fn part2(input: Input) -> Output {
    solve(input, 1)
}

#[test]
fn default() {
    let input = get_input(15, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(1824, part1(input));
    assert_eq!(1937, part2(input));
}

// Input parsed (18μs)
// 1. 1824 (98ms)
// 2. 1937 (5ms)
// Total: 104ms