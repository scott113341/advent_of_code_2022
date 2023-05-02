mod constant_iterator;
mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 08");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

fn make_tree_grid(lines: &Vec<String>) -> TreeGrid {
    let width = lines[0].len();
    let height = lines.len();
    let mut trees = Vec::new();

    for l in lines {
        for c in l.chars() {
            trees.push(c.to_digit(10).unwrap() as i8);
        }
    }

    TreeGrid::new(trees, width, height)
}

// How many trees are visible from outside the grid?
fn part_1(lines: Vec<String>) -> usize {
    let tree_grid = make_tree_grid(&lines);
    tree_grid.visible_count()
}

// What is the highest scenic score of any tree?
fn part_2(lines: Vec<String>) -> usize {
    let tree_grid = make_tree_grid(&lines);
    let mut highest_scenic_score = 0;

    for row in 0..tree_grid.height() {
        for col in 0..tree_grid.width() {
            let scenic_score = tree_grid.scenic_score(row, col);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    highest_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 8);
    }
}
