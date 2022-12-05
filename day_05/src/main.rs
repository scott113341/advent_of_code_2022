mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 05");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// After the rearrangement procedure completes, what crate ends up on top of each stack?
fn part_1(lines: Vec<String>) -> String {
    let moves = get_moves(&lines);
    let mut stacks = get_stacks(lines);

    for mov in moves {
        stacks.apply_move_9000(&mov);
    }

    stacks.top_crates()
}

// When moving multiple crates, the order stays the sae. After the rearrangement procedure
// completes, what crate ends up on top of each stack?
fn part_2(lines: Vec<String>) -> String {
    let moves = get_moves(&lines);
    let mut stacks = get_stacks(lines);

    for mov in moves {
        stacks.apply_move_9001(&mov);
    }

    stacks.top_crates()
}

fn get_stacks(lines: Vec<String>) -> Stacks {
    let stacks_string: String = lines
        .into_iter()
        .take_while(|line| line.starts_with(' ') || line.starts_with('['))
        .collect::<Vec<_>>()
        .join("\n");

    stacks_string.parse().unwrap()
}

fn get_moves(lines: &[String]) -> Vec<Move> {
    lines
        .iter()
        .filter_map(|l| l.parse::<Move>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), String::from("CMZ"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), String::from("MCD"));
    }
}
