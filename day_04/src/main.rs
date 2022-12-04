mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 04");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// In how many assignment pairs does one range fully contain the other?
fn part_1(section_pairs: Vec<SectionPair>) -> usize {
    section_pairs
        .iter()
        .filter(|sp| sp.one_fully_contained())
        .count()
}

// In how many assignment pairs do the ranges overlap?
fn part_2(section_pairs: Vec<SectionPair>) -> usize {
    section_pairs.iter().filter(|sp| sp.any_overlap()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 4);
    }
}
