use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{
    minmax::minmax,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 9);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input<'_>) -> Vec<&'_ str> {
    input
        .lines()
        .map(|line| {
            let (_, dna) = line.split_once(':').unwrap();
            dna
        })
        .collect()
}

fn part1(input: Input) -> usize {
    let (a, b, c) = input
        .lines()
        .map(|line| &line[2..])
        .collect_tuple()
        .unwrap();
    let dna: Vec<[char; 3]> = a
        .chars()
        .zip(b.chars())
        .zip(c.chars())
        .map(|((x, y), z)| [x, y, z])
        .collect();

    let mut sim = vec![Some([0usize; 2]); 3];
    for pos in dna {
        for scale in 0..3 {
            if let Some(similars) = &mut sim[scale] {
                let a = pos[scale];
                let b = pos[(scale + 1) % 3];
                let c = pos[(scale + 2) % 3];
                let mut matching = false;
                if a == b {
                    similars[0] += 1;
                    matching = true;
                }
                if a == c {
                    similars[1] += 1;
                    matching = true;
                }
                if !matching {
                    sim[scale] = None;
                }
            }
        }
    }
    sim.into_iter().flatten().map(|it| it[0] * it[1]).sum()
}

fn part2(input: Input) -> usize {
    let dna = parse(input);
    let mut sim_sum = 0;
    'outer: for scale in 0..dna.len() {
        let aa = dna[scale];
        'mid: for (p1, p2) in (0..dna.len())
            .tuple_combinations()
            .filter(|&(a, b)| a != scale && b != scale)
        {
            let bb = dna[p1];
            let mut b_sim = 0;
            let cc = dna[p2];
            let mut c_sim = 0;
            for ((a, b), c) in aa.chars().zip(bb.chars()).zip(cc.chars()) {
                let mut matching = false;
                if a == b {
                    b_sim += 1;
                    matching = true;
                }
                if a == c {
                    c_sim += 1;
                    matching = true;
                }
                if !matching {
                    continue 'mid;
                }
            }
            sim_sum += b_sim * c_sim;
            continue 'outer;
        }
    }
    sim_sum
}

fn part3(input: Input) -> usize {
    let dna = parse(input);
    let mut tree: [Option<usize>; 500] = [None; 500];
    let mut next_group = 0;
    let mut group_merge: Vec<Option<usize>> = Vec::new();
    'outer: for child in 0..dna.len() {
        let aa = dna[child];
        'mid: for (p1, p2) in (0..dna.len())
            .tuple_combinations()
            .filter(|&(a, b)| a != child && b != child)
        {
            let bb = dna[p1];
            let cc = dna[p2];
            for ((a, b), c) in aa.chars().zip(bb.chars()).zip(cc.chars()) {
                if a != b && a != c {
                    continue 'mid;
                }
            }
            // parents found; merge them first
            let p1_group = tree[p1];
            let p2_group = tree[p2];
            if p1_group.is_some() && p2_group.is_none() {
                tree[p2] = p1_group;
            } else if p2_group.is_some() && p1_group.is_none() {
                tree[p1] = p2_group;
            } else if let Some(p1_group) = p1_group && let Some(p2_group) = p2_group {
                if p1_group != p2_group {
                    let root1 = get_root(&mut group_merge, p1_group);
                    let root2 = get_root(&mut group_merge, p2_group);
                    let (&min, &max) = minmax(&root1, &root2);
                    if min != max {
                        group_merge[max] = Some(min);
                    }
                }
            }
            // merge child with parents
            if let Some(child_group) = tree[child] {
                if let Some(p1_group) = tree[p1] {
                    if child_group != p1_group {
                        let root1 = get_root(&mut group_merge, child_group);
                        let root2 = get_root(&mut group_merge, p1_group);
                        let (&min, &max) = minmax(&root1, &root2);
                        if min != max {
                            group_merge[max] = Some(min);
                        }
                    }
                } else {
                    tree[p1] = Some(child_group);
                    tree[p2] = Some(child_group);
                }
            } else {
                if let Some(p1_group) = tree[p1] {
                    tree[child] = Some(p1_group);
                } else {
                    let assign = Some(next_group);
                    tree[child] = assign;
                    tree[p1] = assign;
                    tree[p2] = assign;
                    next_group += 1;
                    group_merge.push(None);
                }
            }
            continue 'outer;
        }
    }
    // done with finding parents, now need to assign scales to merged groups
    let mut scales: Vec<Vec<usize>> = vec![Vec::new(); group_merge.len()];
    for (scale, group) in tree.into_iter().enumerate() {
        let Some(group) = group else {
            continue;
        };
        scales[get_root(&mut group_merge, group)].push(scale + 1);
    }
    let test = scales.into_iter()
        .max_by_key(|it| it.len())
        .map(|it| it.iter().sum())
        .unwrap();
    test
}

fn get_root(group_merge: &mut Vec<Option<usize>>, group: usize) -> usize {
    let Some(root) = group_merge[group] else {
        return group;
    };
    let root = get_root(group_merge, root);
    group_merge[group] = Some(root);
    root
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 9);
    assert_eq!(6478, part1(&input1));
    assert_eq!(316671, part2(&input2));
    assert_eq!(40905, part3(&input3));
}

// Input parsed (63μs)
// 1. 6478 (9μs)
// 2. 316671 (2.102ms)
// 3. 40905 (233.631ms)
// Total: 235.816ms
