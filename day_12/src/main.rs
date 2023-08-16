mod data;
mod input;

use data::*;
use input::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    println!("day: 12");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

fn find_shortest(
    grid: Grid,
    end: Coord,
    highest: char,
    starting_nodes: Vec<(Coord, char)>,
) -> usize {
    let mut current_nodes = starting_nodes;
    let mut costs: HashMap<Coord, usize> = HashMap::new();
    current_nodes.iter().for_each(|(coord, _val)| {
        costs.insert(*coord, 0);
    });

    loop {
        let mut next_nodes = vec![];

        for (current_coord, current_val) in current_nodes.iter() {
            let adjacent = [
                grid.get_key_value(&(current_coord.0 + 1, current_coord.1)),
                grid.get_key_value(&(current_coord.0 - 1, current_coord.1)),
                grid.get_key_value(&(current_coord.0, current_coord.1 + 1)),
                grid.get_key_value(&(current_coord.0, current_coord.1 - 1)),
            ];

            for a in adjacent {
                let current_cost = *costs.get(current_coord).unwrap();
                if let Some((&next_coord, &next_val)) = a {
                    let is_valid_next = *current_val == 'S'
                        || match next_val == 'E' {
                            true => *current_val == highest,
                            false => next_val as u8 <= *current_val as u8 + 1,
                        };

                    if is_valid_next {
                        match costs.entry(next_coord) {
                            Entry::Occupied(_cost) => (),
                            Entry::Vacant(cost) => {
                                cost.insert(current_cost + 1);
                                next_nodes.push((next_coord, next_val));
                            }
                        }
                    }
                }
            }
        }

        current_nodes = next_nodes;

        if let Some(cost) = costs.get(&end) {
            return *cost;
        }
    }
}

// What is the fewest steps required to move from your current position to the location that should
// get the best signal?
fn part_1(lines: Vec<String>) -> usize {
    let (grid, start, end, highest) = build_grid(lines);
    let starting_nodes = vec![(start, 'S')];
    find_shortest(grid, end, highest, starting_nodes)
}

// What is the fewest steps required to move starting from any square with elevation 'a' to the
// location that should get the best signal?
fn part_2(lines: Vec<String>) -> usize {
    let (grid, start, end, highest) = build_grid(lines);

    let mut starting_nodes = vec![(start, 'S')];
    grid.iter()
        .filter(|(_coord, val)| **val == 'a')
        .for_each(|(coord, val)| starting_nodes.push((*coord, *val)));

    find_shortest(grid, end, highest, starting_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 29);
    }
}
