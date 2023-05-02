mod data;
mod input;

use data::*;
use input::*;
use std::collections::HashSet;
use Direction::*;

fn main() {
    println!("day: 09");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// How many positions does the tail of the rope visit at least once?
fn part_1(steps: Vec<Step>) -> usize {
    count_tail_positions(steps, 2)
}

// Same, but instead of just two knots, there are ten
fn part_2(steps: Vec<Step>) -> usize {
    count_tail_positions(steps, 10)
}

fn count_tail_positions(steps: Vec<Step>, knot_count: usize) -> usize {
    let mut knots = vec![Coord { x: 0, y: 0 }; knot_count];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(*knots.last().unwrap());

    for step in steps {
        for _ in 0..step.magnitude {
            // Move head
            let head = knots.get_mut(0).unwrap();
            match step.direction {
                Right => head.x += 1,
                Left => head.x -= 1,
                Up => head.y += 1,
                Down => head.y -= 1,
            }

            // Move remaining knots in order
            for b_idx in 1..knot_count {
                let lead = *knots.get(b_idx - 1).unwrap();
                let trail = knots.get_mut(b_idx).unwrap();

                // Compute state
                let are_overlapping = |l: &Coord, t: &Coord| l == t;
                let are_touching =
                    |l: &Coord, t: &Coord| (l.x - t.x).abs() <= 1 && (l.y - t.y).abs() <= 1;
                let are_aligned = |l: &Coord, t: &Coord| l.x == t.x || l.y == t.y;

                // Move trailing knot if needed
                match (
                    are_overlapping(&lead, trail),
                    are_touching(&lead, trail),
                    are_aligned(&lead, trail),
                ) {
                    // Overlapping; trailing knot doesn't move
                    (true, _, _) => {}

                    // Touching; trailing knot doesn't move
                    (false, true, _) => {}

                    // Not touching but aligned; drag trailing knot that way
                    (false, false, true) => {
                        trail.x = (lead.x + trail.x) / 2;
                        trail.y = (lead.y + trail.y) / 2;
                    }

                    // Not touching and diagonal; drag trailing knot diagonally
                    (false, false, false) => {
                        let diagonal_candidates = [
                            Coord {
                                x: trail.x + 1,
                                y: trail.y + 1,
                            },
                            Coord {
                                x: trail.x + 1,
                                y: trail.y - 1,
                            },
                            Coord {
                                x: trail.x - 1,
                                y: trail.y + 1,
                            },
                            Coord {
                                x: trail.x - 1,
                                y: trail.y - 1,
                            },
                        ];
                        let go_to = diagonal_candidates
                            .iter()
                            .find(|c| are_touching(&lead, c))
                            .unwrap();
                        trail.x = go_to.x;
                        trail.y = go_to.y
                    }
                }
            }

            tail_positions.insert(*knots.last().unwrap());
        }
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input_1()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input_1()), 1);
        assert_eq!(part_2(get_test_input_2()), 36);
    }
}
