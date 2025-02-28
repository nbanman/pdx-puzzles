use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<u32>>;
type Output = u32;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn astar(city: &Input, l: usize, h: usize) -> u32 {
    let width = city.first().unwrap().len();
    let height = city.len();
    let mut bq = vec![Vec::with_capacity(300); 100];
    bq[0].push(0);
    bq[0].push(1);
    let mut cost = vec![vec![vec![0_u32; 2]; width]; height];
    let end = width * height - 1;
    let mut bucket_index = 0_usize;

    loop {
        while let Some(state) = bq[bucket_index % 100].pop() {
            let pos = state >> 1;
            let dir = state & 1;
            let x = pos % width;
            let y = pos / width;
            let steps = cost[y][x][dir];

            if pos == end {
                return steps;
            };

            let heuristic = {
                |x: usize, y: usize, steps: u32| (steps as usize + width - x + height - y) % 100
            };

            let mut new_x;
            let mut new_y;
            let mut new_steps;

            if dir == 0 {
                // left
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if i > x {
                        break;
                    };
                    new_x -= 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][1] == 0 || new_steps < cost[new_y][new_x][1]) {
                        bq[heuristic(x - i, y, new_steps)].push(((new_y * width + new_x) << 1) + 1);
                        cost[new_y][new_x][1] = new_steps;
                    }
                }
                // right
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if x + i >= width {
                        break;
                    };
                    new_x += 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][1] == 0 || new_steps < cost[new_y][new_x][1]) {
                        bq[heuristic(x + i, y, new_steps)].push(((new_y * width + new_x) << 1) + 1);
                        cost[new_y][new_x][1] = new_steps;
                    }
                }
            } else {
                // up
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if i > y {
                        break;
                    };
                    new_y -= 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][0] == 0 || new_steps < cost[new_y][new_x][0]) {
                        bq[heuristic(x, y - i, new_steps)].push((new_y * width + new_x) << 1);
                        cost[new_y][new_x][0] = new_steps;
                    }
                }

                // down
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if y + i >= height {
                        break;
                    };
                    new_y += 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][0] == 0 || new_steps < cost[new_y][new_x][0]) {
                        bq[heuristic(x, y + i, new_steps)].push((new_y * width + new_x) << 1);
                        cost[new_y][new_x][0] = new_steps;
                    }
                }
            }
        }
        bucket_index += 1;
    }
}

fn part1(city: &Input) -> Output {
    astar(city, 1, 3)
}

fn part2(city: &Input) -> Output {
    astar(city, 4, 10)
}

#[test]
fn default() {
    let input = get_input(23, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(635, part1(&input));
    assert_eq!(734, part2(&input));
}

// Input parsed (110μs)
// 1. 635 (2ms)
// 2. 734 (4ms)
// Total: 7ms
