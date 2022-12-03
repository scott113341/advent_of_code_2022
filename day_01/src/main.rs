mod input;

use input::*;

fn main() {
    println!("day: 01");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// How many Calories are being carried by the Elf carrying the most Calories?
fn part_1(lines: Vec<String>) -> usize {
    let mut max_elf = 0;
    let mut current_elf = 0;

    for line in lines {
        match line.parse::<usize>() {
            Ok(cals) => current_elf += cals,
            Err(_) => {
                max_elf = max_elf.max(current_elf);
                current_elf = 0;
            }
        }
    }
    max_elf = max_elf.max(current_elf);

    max_elf
}

// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying
// in total?
fn part_2(lines: Vec<String>) -> usize {
    let mut elf_cals = vec![];
    let mut current_elf = 0;

    for line in lines {
        match line.parse::<usize>() {
            Ok(cals) => current_elf += cals,
            Err(_) => {
                elf_cals.push(current_elf);
                current_elf = 0;
            }
        }
    }
    elf_cals.push(current_elf);

    elf_cals.sort();
    elf_cals.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 45000);
    }
}
