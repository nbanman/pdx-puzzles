use fancy_regex::Regex;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use std::sync::LazyLock;

type Input<'a> = &'a str;
type Output = usize;

static VOWELS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[aeiou]").unwrap()
});
static REPEATED_LETTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([a-z])\1").unwrap()
});
static FORBIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"ab|cd|pq|xy").unwrap()
});
static REPEATED_DUO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([a-z]{2}).*\1").unwrap()
});
static REPEATED_1_BETWEEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([a-z]).\1").unwrap()
});

trait MaybeNice {
    fn at_least_three_vowels(&self) -> bool;
    fn at_least_one_twice(&self) -> bool;
    fn no_forbidden_strings(&self) -> bool;
    fn at_least_two_twice(&self) -> bool;
    fn repeats_with_one_between(&self) -> bool;
    fn is_kinda_nice(&self) -> bool;
    fn is_really_nice(&self) -> bool;
}

impl<'a> MaybeNice for &'a str {
    fn at_least_three_vowels(&self) -> bool {
        VOWELS.find_iter(self).count() >= 3
    }

    fn at_least_one_twice(&self) -> bool {
        REPEATED_LETTER.is_match(self).unwrap_or_default()
    }

    fn no_forbidden_strings(&self) -> bool {
        !FORBIDDEN.is_match(self).unwrap_or_default()
    }

    fn at_least_two_twice(&self) -> bool {
        REPEATED_DUO.is_match(self).unwrap_or_default()
    }

    fn repeats_with_one_between(&self) -> bool {
        REPEATED_1_BETWEEN.is_match(self).unwrap_or_default()
    }

    fn is_kinda_nice(&self) -> bool {
        self.at_least_three_vowels() && self.at_least_one_twice() && self.no_forbidden_strings()
    }

    fn is_really_nice(&self) -> bool {
        self.at_least_two_twice() && self.repeats_with_one_between()
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 5).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(strings: Input) -> Output {
    strings.lines()
        .filter(|line| line.is_kinda_nice())
        .count()
}

fn part2(strings: Input) -> Output {
    strings.lines()
        .filter(|line| line.is_really_nice())
        .count()
}

#[test]
fn default() {
    let input = get_input(15, 5).unwrap();
    assert_eq!(255, part1(&input));
    assert_eq!(55, part2(&input));
}

// Input parsed (27μs)
// 1. 255 (724μs)
// 2. 55 (2ms)
// Total: 3ms
