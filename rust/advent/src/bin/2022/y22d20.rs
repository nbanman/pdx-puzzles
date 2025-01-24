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

fn solve(input: &Input, factor: i64, rounds: usize) -> Output {
    // Important nuance, size is one less because we don't consider the moving number.
    let size = input.len() - 1;
    // Another nuance, input contain duplicate numbers, so use index to refer to each number uniquely.
    let indices: Vec<_> = (0..input.len()).collect();
    // Pre-process the numbers, coverting any negative indices to positive indices that will wrap.
    // For example, -1 becomes 4998.
    let numbers: Vec<_> =
        input.iter().map(|n| (n * factor).rem_euclid(size as i64) as usize).collect();

    // Store first and second level indices.
    let mut lookup = Vec::new();
    // Triple nested vec of numbers.
    let mut mixed = Vec::new();
    // Size of each first level element for convenience.
    let mut skip = Vec::new();

    // Break 5000 numbers into roughly equals chunks at each level. 289 = 17 * 17.
    for first in indices.chunks(289) {
        let mut outer = Vec::new();

        for second in first.chunks(17) {
            // Initial first and second level indices.
            (0..second.len()).for_each(|_| lookup.push((mixed.len(), outer.len())));

            // Leave some extra room, as mixing won't balance evenly.
            let mut inner = Vec::with_capacity(100);
            inner.extend_from_slice(second);

            outer.push(inner);
        }

        mixed.push(outer);
        skip.push(first.len());
    }

    for _ in 0..rounds {
        'mix: for index in 0..input.len() {
            // Quickly find the leaf vector storing the number.
            let number = numbers[index];
            let (first, second) = lookup[index];
            // Third level changes as other numbers are added and removed,
            // so needs to be checked each time.
            let third = mixed[first][second].iter().position(|&i| i == index).unwrap();

            // Find the offset of the number by adding the size of all previous `vec`s.
            let position = third
                + skip[0..first].iter().sum::<usize>()
                + mixed[first][0..second].iter().map(Vec::len).sum::<usize>();
            // Update our position, wrapping around if necessary.
            let mut next = (position + number) % size;

            // Remove number from current leaf vector, also updating the first level size.
            mixed[first][second].remove(third);
            skip[first] -= 1;

            // Find our new destination, by checking `vec`s in order until the total elements
            // are greater than our new index.
            for (first, outer) in mixed.iter_mut().enumerate() {
                if next > skip[first] {
                    next -= skip[first];
                } else {
                    for (second, inner) in outer.iter_mut().enumerate() {
                        if next > inner.len() {
                            next -= inner.len();
                        } else {
                            // Insert number into its new home.
                            inner.insert(next, index);
                            skip[first] += 1;
                            lookup[index] = (first, second);
                            continue 'mix;
                        }
                    }
                }
            }
        }
    }

    let indices: Vec<_> = mixed.into_iter().flatten().flatten().collect();
    let zeroth = indices.iter().position(|&i| input[i] == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| (zeroth + offset) % indices.len())
        .map(|index| input[indices[index]] * factor)
        .sum()
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
