use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<i64>;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());

    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn solve(numbers: &Input, factor: i64, rounds: usize) -> Output {
    let numbers: Vec<i64> = numbers.iter().map(|&n| n * factor).collect();
    let to_chunk: Vec<usize> = (0..numbers.len()).collect();
    let bucket_size = (numbers.len() as f64).powf(1.0 / 3.0).floor() as usize;
    
    let mut top: Vec<Vec<Vec<usize>>> = Vec::with_capacity(bucket_size + 1);
    let mut top_count: Vec<usize> = vec![0usize; bucket_size + 1];
    let mut mid_count: Vec<Vec<usize>> = vec![vec![0usize; bucket_size]; bucket_size + 1];
    let mut lookup: Vec<(usize, usize)> = Vec::with_capacity(numbers.len());

    for (top_idx, top_chunk) in to_chunk
        .chunks(bucket_size * bucket_size)
        .enumerate() 
    {
        let mut mid: Vec<Vec<usize>> = Vec::with_capacity(bucket_size);
        for (mid_idx, mid_chunk) in top_chunk.chunks(bucket_size).enumerate() {
            let mut bottom: Vec<usize> = Vec::with_capacity(100);
            bottom.extend_from_slice(mid_chunk);
            for _ in mid_chunk {
                lookup.push((top_idx, mid_idx));
            }
            mid.push(bottom);
            mid_count[top_idx][mid_idx] += mid_chunk.len();
            top_count[top_idx] += mid_chunk.len();
        }
        top.push(mid);
    }

    for _ in 0..rounds {
        for n in 0..numbers.len() {
            // find top and mid location
            let (top_idx, mid_idx) = lookup[n];
            
            // find bottom location
            let bottom = &mut top[top_idx][mid_idx];
            let bottom_idx = bottom.iter().position(|&i| i == n).unwrap();

            // find current overall position
            let pos: usize = top_count[0..top_idx].iter().sum::<usize>()
                + mid_count[top_idx][0..mid_idx].iter().sum::<usize>()
                + bottom_idx;

            let movement = (pos as i64 + numbers[n]).rem_euclid(numbers.len() as i64 - 1) as usize;

            // remove index from location
            bottom.remove(bottom_idx);

            // update counts
            mid_count[top_idx][mid_idx] -= 1;
            top_count[top_idx] -= 1;

            // find placement location
            let mut cumulative = 0;
            let top_update = navigate(&top_count, movement, &mut cumulative);
            let mid_update = navigate(&mid_count[top_update], movement, &mut cumulative);

            // place index in new place
            top[top_update][mid_update].insert(movement - cumulative, n);

            // update lookup
            lookup[n] = (top_update, mid_update);

            // update counts
            top_count[top_update] += 1;
            mid_count[top_update][mid_update] += 1;
        }
    }

    let flattened: Vec<_> = top.into_iter().flatten().flatten().collect();

    let zero_index = numbers.iter().position(|&n| n == 0).unwrap();
    let flat_index = flattened.iter().position(|&n| n == zero_index).unwrap();

    (1..=3)
        .map(|i| {
            let index = (i * 1000 + flat_index) % flattened.len();
            numbers[flattened[index]]
        })
        .sum()
}

fn navigate(count: &Vec<usize>, movement: usize, cumulative: &mut usize) -> usize {
    let mut update = 0;
    for (i, &count) in count.iter().enumerate() {
        if *cumulative + count > movement {
            update = i;
            break;
        }
        *cumulative += count;
    }
    update
}

fn part1(numbers: &Input) -> Output {
    solve(numbers, 1, 1)
}

fn part2(numbers: &Input) -> Output {
    solve(numbers, 811589153, 10)
}

#[test]
fn default() {
    let input = get_input(22, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(4151, part1(&input));
    assert_eq!(7848878698663, part2(&input));
}