mod data;
mod input;

use data::*;
use input::*;
use std::collections::HashSet;

fn main() {
    println!("day: 03");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Find the item type that appears in both compartments of each rucksack. What is the sum of the
// priorities of those item types?
fn part_1(rucksacks: Vec<Rucksack>) -> usize {
    let mut sum_of_priorities = 0;

    for r in rucksacks {
        let uniq_items_0: HashSet<_> = r.items_0.iter().collect();
        let uniq_items_1: HashSet<_> = r.items_1.iter().collect();
        let common_item = uniq_items_0.intersection(&uniq_items_1).next().unwrap();
        sum_of_priorities += Rucksack::priority(common_item);
    }

    sum_of_priorities
}

// Find the item type that corresponds to the badges of each three-Elf group (only item type that
// appears in ALL rucksacks of each group). What is the sum of the priorities of those item types?
fn part_2(rucksacks: Vec<Rucksack>) -> usize {
    let mut sum_of_priorities = 0;

    let mut group_count = 0;
    let mut common_items = HashSet::new();

    for r in rucksacks {
        if group_count == 0 {
            // Seed common_items with all from first rucksack
            common_items.extend(r.items_0.iter());
            common_items.extend(r.items_1.iter());
        } else {
            // Remove all non-common items
            let mut rucksack_items = HashSet::new();
            rucksack_items.extend(r.items_0.iter());
            rucksack_items.extend(r.items_1.iter());
            common_items = &common_items & &rucksack_items;
        }

        group_count += 1;

        // Add the priority of the remaining item and reset for the next group
        if group_count == 3 {
            assert_eq!(common_items.len(), 1);
            let common_item = common_items.iter().next().unwrap();
            sum_of_priorities += Rucksack::priority(common_item);

            common_items.clear();
            group_count = 0;
        }
    }

    sum_of_priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 70);
    }
}
