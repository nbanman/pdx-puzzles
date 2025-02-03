use advent::utilities::get_input::get_input;
use utilities::structs::{stopwatch::{ReportDuration, Stopwatch}, str_grid::StrGrid};

type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct Image {
    x_expansions: Vec<usize>,
    y_expansions: Vec<usize>,
    x_galaxies: Vec<(usize, usize)>,
    y_galaxies: Vec<(usize, usize)>,
}



fn parse_input(input: &str) -> Image {
    let StrGrid { s: data, width, height } = StrGrid::new(input).unwrap();

    // for each axis, get indexed iterables of pairs. The first being the index where galaxies reside, the
    // second being the number of galaxies. We separate the axes to avoid repeat calculation.
    let x_galaxies: Vec<(usize, usize)> = (0..width)
        .map(|x| {
            let count = (0..height).map(|y| data[x + y * width])
                .filter(|c| *c == b'#')
                .count();
            (x, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();
    let y_galaxies: Vec<(usize, usize)> = (0..height)
        .map(|y| {
            let count = data[y * width..y * width + width].iter()
                .filter(|&&c| c == b'#')
                .count();
            (y, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();

    // for each axis, track the indices representing expansion fields
    let x_expansion: Vec<usize> = (0..width - 1)
        .filter(|x| {
            (0..height).all(|y| data[*x + y * width] == b'.')
        })
        .collect();
    let y_expansion: Vec<usize> = (0..height)
        .filter(|y| {
            (0..width - 1).all(|x| data[x + *y * width] == b'.')
        })
        .collect();

    Image { x_expansions: x_expansion, y_expansions: y_expansion, x_galaxies, y_galaxies }
}

// run the distance function twice (once for each axis), return the sum
fn solve(image: &Image, expansion_factor: usize) -> usize {
    distance(expansion_factor, &image.x_galaxies, &image.x_expansions) +
        distance(expansion_factor, &image.y_galaxies, &image.y_expansions)
}

// Get the distance between two indices where galaxies reside.
// This involves calculating the unexpanded difference multiplied by the number of expansions passed times
// the expansion factor
// Lastly, the distance is multiplied by #galaxies in the first index multiplied by #galaxies in the second index.
fn distance(
    expansion_factor: usize,
    galaxies: &[(usize, usize)],
    expansions: &[usize],
) -> usize {
    galaxies.iter().enumerate()
        .map(|(i, (a_pos, a_count))| {


            // calculate which expansions are to the left of the source galaxies
            // this returns a negative number due to how binarySearch returns values but this will be rectified
            // later.
            let already_passed = expansions.binary_search(a_pos).err().unwrap();
            galaxies.iter().skip(i + 1)
                .map(|(b_pos, b_count)| {

                    // calculates which expansions are to the left of the destination galaxies, and subtracts the
                    // ones to the left of the source galaxies. The abs function handles the negative value.
                    let expansions_passed = expansions.binary_search(b_pos).err().unwrap()
                        - already_passed;
                    ((b_pos - a_pos) + expansions_passed * (expansion_factor - 1)) * a_count * b_count
                })
                .sum::<usize>()
        })
        .sum()
}

fn part1(image: &Image) -> Output {
    solve(image, 2)
}

fn part2(image: &Image) -> Output {
    solve(image, 1_000_000)
}

#[test]
fn default() {
    let input = get_input(23, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(9545480, part1(&input));
    assert_eq!(406725732046, part2(&input));
}

// Input parsed (53μs)
// 1. 9545480 (51μs)
// 2. 406725732046 (48μs)
// Total: 155μs