use std::sync::LazyLock;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use fancy_regex::Regex;

static PAIR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([a-z])\1").unwrap()
});

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 11).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    let p1 = next_password(input);
    println!("1. {} ({})", p1, stopwatch.lap().report());
    println!("2. {} ({})", next_password(p1), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn increment(password: &mut str) {
    // SAFETY: This is Advent of Code, where in 10 years of puzzle challenges ALL strings have been
    // valid UTF-8. Much of my codebase implicitly assumes this, so this won't be the only thing that
    // breaks if this invariant ever gets broken.
    let password = unsafe { password.as_bytes_mut() };
    let (idx, c) = password.iter_mut().enumerate().rev().find(|(_, c)| **c != b'z').unwrap();
    *c += 1;
    for c in password[idx + 1 ..].iter_mut() {
        *c = b'a';
    }
}

fn is_valid(candidate: &str) -> bool {
    let bytes = candidate.as_bytes();
    let straight = bytes.iter().tuple_windows().any(|(&a, &b, &c)| {
        b.checked_sub(a).map(|x| x== 1).unwrap_or_default()
            && c.checked_sub(b).map(|x| x== 1).unwrap_or_default()
    });
    if !straight { return false; }

    let confusion = bytes.iter().any(|&b| b == b'i' || b == b'o' || b == b'l');
    if confusion { return false; }

    PAIR.find_iter(candidate).take(2).count() == 2
}

fn next_password(mut password: String) -> String {
    loop {
        increment(&mut password);
        if is_valid(&password) {
            return password;
        }
    }
}

#[test]
fn default() {
    let input = get_input(15, 11).unwrap();
    let p1 = next_password(input);
    assert_eq!("hxbxxyzz".to_string(), p1);
    assert_eq!("hxcaabcc".to_string(), next_password(p1));
}

// Input parsed (29μs)
// 1. hxbxxyzz (371μs)
// 2. hxcaabcc (6ms)
// Total: 7ms
