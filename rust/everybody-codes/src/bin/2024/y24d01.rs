use everybody_codes::utilities::inputs::get_inputs;

fn main() {
    let (input1, input2, input3) = get_inputs(24, 1);
    println!("1. {}", solve(&input1, 1));
    println!("2. {}", solve(&input2, 2));
    println!("3. {}", solve(&input3, 3));
}

fn solve(input: &str, group_size: usize) -> usize {
    input
        .as_bytes()
        .chunks(group_size)
        .map(|baddies| {
            let number_of_baddies = baddies
                .iter()
                .filter(|&&it| it != b'x')
                .count();
            let value = |baddie: &u8| -> usize {
                match &baddie {
                    b'A' => 0,
                    b'B' => 1,
                    b'C' => 3,
                    b'D' => 5,
                    _ => 0,
                }
            };
            let potions: usize = baddies.iter().map(value).sum();
            let bonus_potions: usize = number_of_baddies * 
                number_of_baddies.checked_sub(1).unwrap_or_default();
            potions + bonus_potions
        }).sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_inputs(24, 1);
    assert_eq!(1354, solve(&input1, 1));
    assert_eq!(5639, solve(&input2, 2));
    assert_eq!(28180, solve(&input3, 3));
}