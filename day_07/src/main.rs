mod data;
mod input;

use data::*;
use input::*;
use std::collections::BTreeSet;

fn main() {
    println!("day: 07");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// To begin, find all of the directories with a total size of at most 100000, then calculate the sum
// of their total sizes. This process can count files more than once! What is the sum of the total
// sizes of those directories?
fn part_1(lines: Vec<String>) -> usize {
    let fs = FileSystem::new(lines);

    fs.dir_sizes()
        .iter()
        .filter_map(|(_path, size)| if *size <= 100000 { Some(size) } else { None })
        .sum()
}

// The total disk space available to the filesystem is 70000000. To run the update, you need unused
// space of at least 30000000. Find the smallest directory that, if deleted, would free up enough
// space on the filesystem to run the update. What is the total size of that directory?
fn part_2(lines: Vec<String>) -> usize {
    let fs = FileSystem::new(lines);
    let dir_sizes = fs.dir_sizes();
    let used_space = dir_sizes.get("").unwrap();
    let free_space = 70000000 - used_space;

    let sizes = dir_sizes
        .values()
        .filter(|size| free_space + **size >= 30000000)
        .collect::<BTreeSet<_>>();

    **sizes.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 24933642);
    }
}
