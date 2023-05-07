mod data;
mod input;

use data::*;
use input::*;
use std::collections::BTreeMap;

fn main() {
    println!("day: 11");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// During a round, each money inspects its items in order. Item's worry level changes according to
// the operation. Then, worry level is divided by 3, rounded down. Then, item gets thrown to another
// monkey, depending on whether cleanly divisible. Count how many times each monkey inspects items
// over 20 rounds. Choose two most active monkeys. Multiply these two counts together to find the
// level of monkey business.
fn part_1(monkeys: Vec<Monkey>) -> usize {
    find_monkey_business(monkeys, 20, true)
}

// Worry level is no longer divided by three. What is the level of monkey business after 10,000
// rounds?
fn part_2(_monkeys: Vec<Monkey>) -> char {
    // find_monkey_business(monkeys, 10_000, false)
    '?'
}

fn find_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, divide: bool) -> usize {
    let mut inspection_counts = BTreeMap::new();

    for _round in 1..=rounds {
        for monkey_idx in 0..monkeys.len() {
            let throws: Vec<_> = {
                let monkey = monkeys.get_mut(monkey_idx).unwrap();
                *inspection_counts.entry(monkey.id).or_insert(0) += monkey.items.len();

                monkey.items.drain(..).map(|mut item| {
                    item = monkey.operation.compute(item);

                    if divide {
                        item /= 3;
                    }

                    let throw_to_monkey = if item % monkey.test_divisible_by == 0 {
                        monkey.divisible_true
                    } else {
                        monkey.divisible_false
                    };

                    (throw_to_monkey, item)
                })
            }
            .collect();

            for (throw_to_monkey, item) in throws {
                monkeys
                    .get_mut(throw_to_monkey)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    let mut counts: Vec<_> = inspection_counts.values().collect();
    counts.sort();
    counts
        .iter()
        .rev()
        .take(2)
        .fold(1, |tot, count| tot * **count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 10605);
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(get_test_input()), 2713310158);
    }
}
