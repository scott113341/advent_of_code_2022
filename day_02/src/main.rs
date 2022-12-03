mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 02");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// What would your total score be if everything goes exactly according to your strategy guide? Where
// ABC / XYZ = RockPaperScissors.
fn part_1(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let col_1 = split.next().ok_or("Bad strat").unwrap();
            let col_2 = split.next().ok_or("Bad strat").unwrap();

            Round {
                them: col_1.parse().unwrap(),
                me: col_2.parse().unwrap(),
            }
        })
        .map(|round| round.my_score())
        .sum()
}

// What would your total score be if everything goes exactly according to your strategy guide? Where
// ABC = RockPaperScissors and XYZ = LossTieWin.
fn part_2(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let col_1 = split.next().ok_or("Bad strat").unwrap();
            let col_2 = split.next().ok_or("Bad strat").unwrap();

            let them = col_1.parse::<Choice>().unwrap();
            let me = them.choice_for_outcome(&col_2.parse::<RoundOutcome>().unwrap());

            Round { them, me }
        })
        .map(|round| round.my_score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 12);
    }
}
