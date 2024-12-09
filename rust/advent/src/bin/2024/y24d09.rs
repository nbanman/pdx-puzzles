use std::{cmp::Reverse, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 9).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let mut fragmented = Vec::new();
    for (idx, n) in input.chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate() {
        let value = if idx & 1 == 0 { Some(idx / 2) } else { None };
        for _ in 0..n {
            fragmented.push(value);
        }
    }
    let mut left = 0;
    let mut right = fragmented.len() - 1;
    let blocks = fragmented.iter().filter(|it| it.is_some()).count();
    let mut defragged = Vec::new();
    while left < blocks {
        while let Some(block) = fragmented[left] {
            if left < right { 
                defragged.push(block);
                left += 1;
            } else {
                break;
            }
        }
        while let None = fragmented[right] { right -= 1; }
        left += 1;
        defragged.push(fragmented[right].unwrap());
        right -= 1;
    }
    defragged.iter().enumerate().fold(0, |state, (idx, &i)| {
        state + idx * i
    })
}

#[derive(Debug)]
struct Block {
    index: usize,
    size: usize,
    value: usize,
}

impl Block {
    fn checksum(&self) -> usize {
        (self.index..self.index + self.size)
            .map(|index| index * self.value)
            .sum()
    }
}

fn part2(input: Input) -> Output {
    let mut spaces: [BinaryHeap<Reverse<usize>>; 10] = std::array::from_fn(|_| BinaryHeap::new());
    let mut blocks = Vec::new();
    let mut index = 0;
    for (order, size) in input.chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate() {
        let size = size as usize;
        if size > 0 {
            if order & 1 == 0 {
                let block = Block { index, size, value: order / 2 };
                blocks.push(block);
            } else {
                spaces[size].push(Reverse(index));
            }
            index += size
        }
    }

    for block in blocks.iter_mut().rev() {
        if let Some((&space_idx, heap_idx)) = &spaces[block.size..].iter().enumerate()
            .filter_map(|(heap_idx, space)| {
                if let Some(Reverse(space_index)) = space.peek() {
                    if *space_index < block.index {
                        Some((space_index, heap_idx))
                    } else {
                        None
                    }                    
                } else {
                    None
                }
            })
            .sorted_unstable()
            .next()
        {
            let heap_idx = heap_idx + block.size;
            spaces[heap_idx].pop();
            block.index = space_idx;
            if block.size < heap_idx {
                spaces[heap_idx - block.size].push(Reverse(space_idx + block.size));
            }
        }
        
    }
    blocks.iter().map(|block| block.checksum()).sum()
}

#[test]
fn default() {
    let input = get_input(24, 9).unwrap();
    assert_eq!(6390180901651, part1(&input));
    assert_eq!(6412390114238, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"2333133121414131402", ];
    assert_eq!(1928, part1(inputs[0]));
    assert_eq!(2858, part2(inputs[0]));
}

// Input parsed (25μs)
// 1. 6390180901651 (2ms)
// 2. 6412390114238 (1ms)
// Total: 3ms

