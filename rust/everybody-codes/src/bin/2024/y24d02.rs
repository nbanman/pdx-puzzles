use core::str;
use std::{cmp::min, collections::HashSet};

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::structs::coord::Coord;

fn main() {
    let (input1, input2, input3) = get_event_inputs(24, 2);
    println!("1. {}", part1(&input1));
    println!("2. {}", part2(&input2));
    println!("3. {}", part3(&input3));
}

fn parse_words(words: &str) -> impl Iterator<Item = &str> {
    let start_index = words.find(':').unwrap() + 1;
    words[start_index..].split(',')
}

fn part1(input: &str) -> usize {
    let (words, code) = input.split_once("\n\n").unwrap();
    let code = code.as_bytes();
    parse_words(words)
        .map(|word| {
            let word = word.as_bytes();
            code.windows(word.len()).filter(|&it| it == word).count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let (words, code) = input.split_once("\n\n").unwrap();
    let words = get_words(words);
    let word_lengths = get_word_lengths(&words);

    let mut symbols = vec![false; code.len()];

    for (index, c) in code.chars().enumerate() {
        if c == '\n' || c == ' ' {
            continue;
        }
        let mut snippet = &code[index..min(code.len(), index + word_lengths[0])];
        for &length in word_lengths.iter() {
            if snippet.len() < length {
                continue;
            }
            snippet = &snippet[..length];
            if words.contains(snippet) {
                for symbol in symbols.iter_mut().skip(index).take(length) {
                    *symbol = true;
                }
                break;
            }
        }
    }
    symbols.into_iter().filter(|&sym| sym).count()
}

fn get_word_lengths(words: &HashSet<String>) -> Vec<usize> {
    let word_lengths: Vec<usize> = words
        .iter()
        .map(|word| word.len())
        .collect::<HashSet<_>>()
        .into_iter()
        .sorted()
        .rev()
        .collect();
    word_lengths
}

fn get_words(words_str: &str) -> HashSet<String> {
    let mut words: HashSet<String> = HashSet::new();
    let forwards: Vec<&str> = parse_words(words_str).collect();
    for &word in forwards.iter() {
        words.insert(word.to_string());
    }
    forwards
        .iter()
        .map(|&word| word.chars().rev().collect::<String>())
        .for_each(|word| {
            words.insert(word);
        });
    words
}

fn part3(input: &str) -> usize {
    let (words, armor) = input.split_once("\n\n").unwrap();
    let words = get_words(words);
    let word_lengths = get_word_lengths(&words);
    let longest_word = word_lengths[0];

    let armor_width = armor.find('\n').unwrap() + 1;
    let mut symbols = vec![false; armor.len()];

    for (pos, c) in armor.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        let pos = Coord::new2d(pos % armor_width, pos / armor_width);

        let (east_index, east) = east(armor, armor_width, pos, longest_word);
        let mut east = east.as_str();
        for &length in word_lengths.iter() {
            if length != east.len() {
                east = &east[0..length];
            }
            if words.contains(east) {
                for &index in east_index.iter().take(length) {
                    symbols[index] = true;
                }
                break;
            }
        }
        let (south_index, south) = south(armor, armor_width, pos, longest_word);
        let mut south = south.as_str();
        for &length in word_lengths.iter() {
            if south.len() < length {
                continue;
            }
            if length != south.len() {
                south = &south[0..length];
            }
            if words.contains(south) {
                for &index in south_index.iter().take(length) {
                    symbols[index] = true;
                }
                break;
            }
        }
    }
    symbols.into_iter().filter(|&sym| sym).count()
}

fn east(
    armor: &str,
    width: usize,
    pos: Coord<usize, 2>,
    word_length: usize,
) -> (Vec<usize>, String) {
    let armor = armor.as_bytes();
    let indices: Vec<usize> = (0..word_length)
        .map(|i| {
            let x = (pos.x() + i) % (width - 1);
            let coord = Coord::new2d(x, pos.y());

            coord.get_index(&[width]).unwrap()
        })
        .collect();

    let rune: String = indices.iter().map(|&x| armor[x] as char).collect();
    (indices, rune)
}

fn south(
    armor: &str,
    width: usize,
    pos: Coord<usize, 2>,
    word_length: usize,
) -> (Vec<usize>, String) {
    let length = (armor.len() + 1) / width;
    let armor = armor.as_bytes();
    let indices: Vec<usize> = (pos.y()..min(length, pos.y() + word_length))
        .map(|y| Coord::new2d(pos.x(), y).get_index(&[width]).unwrap())
        .collect();
    let rune: String = indices.iter().map(|&x| armor[x] as char).collect();
    (indices, rune)
}

#[test]
fn example() {
    let test1 = r"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
    let test2 = r"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END";
    let test3 = r"WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL";
    assert_eq!(4, part1(test1));
    assert_eq!(37, part2(test2));
    assert_eq!(10, part3(test3));
}

#[test]
fn challenge() {
    let (input1, input2, input3) = get_event_inputs(24, 2);
    assert_eq!(30, part1(&input1));
    assert_eq!(4992, part2(&input2));
    assert_eq!(11816, part3(&input3));
}
