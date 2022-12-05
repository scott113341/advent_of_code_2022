use parse_display::FromStr;
use std::str::FromStr;

#[derive(FromStr, Eq, PartialEq, Debug)]
#[display("move {count} from {from} to {to}")]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    // Moves containers one at a time
    pub fn apply_move_9000(&mut self, mov: &Move) {
        for _ in 0..mov.count {
            let container = self.0[mov.from - 1].pop().unwrap();
            self.0[mov.to - 1].push(container);
        }
    }

    // Can move multiple containers at one; they stay in the same order
    pub fn apply_move_9001(&mut self, mov: &Move) {
        let mut temp_stack = vec![];

        for _ in 0..mov.count {
            let container = self.0[mov.from - 1].pop().unwrap();
            temp_stack.push(container);
        }

        temp_stack.reverse();
        self.0[mov.to - 1].append(&mut temp_stack);
    }

    pub fn top_crates(&self) -> String {
        self.0.iter().filter_map(|s| s.last()).collect::<String>()
    }
}

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = vec![];

        let stacks_count = s
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .count();

        for _ in 0..stacks_count {
            stacks.push(vec![]);
        }

        for line in s.lines().rev() {
            for char_idx in 0..line.len() {
                let char = line.chars().nth(char_idx).unwrap();
                if char.is_alphabetic() {
                    let stack_idx = (char_idx - 1) / 4;
                    stacks[stack_idx].push(char);
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_move_from_str() {
        assert_eq!(
            get_test_input::<String>()[5].parse::<Move>().unwrap(),
            Move {
                count: 1,
                from: 2,
                to: 1
            }
        );
    }

    #[test]
    fn test_stacks_from_str() {
        assert_eq!(
            crate::get_stacks(get_test_input::<String>()),
            Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']])
        );
    }
}
