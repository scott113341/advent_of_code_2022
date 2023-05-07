use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Operand {
    Old,
    Scalar(usize),
}

impl Operation {
    pub fn compute(&self, old: usize) -> usize {
        use Operand::*;
        use Operation::*;

        match self {
            Add(lhs, rhs) => {
                (match lhs {
                    Old => old,
                    Scalar(val) => *val,
                }) + (match rhs {
                    Old => old,
                    Scalar(val) => *val,
                })
            }
            Multiply(lhs, rhs) => {
                (match lhs {
                    Old => old,
                    Scalar(val) => *val,
                }) * (match rhs {
                    Old => old,
                    Scalar(val) => *val,
                })
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<usize>,
    pub operation: Operation,
    pub test_divisible_by: usize,
    pub divisible_true: usize,
    pub divisible_false: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.split('\n').collect();

        let id = lines[0]
            .chars()
            .nth(7)
            .unwrap()
            .to_string()
            .parse()
            .unwrap();

        let items: VecDeque<usize> = lines[1]
            .chars()
            .skip(18)
            .collect::<String>()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let operation = {
            use Operand::*;
            use Operation::*;

            let op: String = lines[2].chars().skip(19).collect();
            let parts: Vec<_> = op.split(' ').collect();

            let lhs = if parts[0] == "old" {
                Old
            } else {
                Scalar(parts[0].parse().unwrap())
            };
            let rhs = if parts[2] == "old" {
                Old
            } else {
                Scalar(parts[2].parse().unwrap())
            };

            if parts[1] == "+" {
                Add(lhs, rhs)
            } else {
                Multiply(lhs, rhs)
            }
        };

        let test_divisible_by = lines[3]
            .chars()
            .skip(21)
            .collect::<String>()
            .parse()
            .unwrap();

        let divisible_true = lines[4]
            .chars()
            .nth(29)
            .unwrap()
            .to_string()
            .parse()
            .unwrap();

        let divisible_false = lines[5]
            .chars()
            .nth(30)
            .unwrap()
            .to_string()
            .parse()
            .unwrap();

        Ok(Monkey {
            id,
            items,
            operation,
            test_divisible_by,
            divisible_true,
            divisible_false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_monkey_from_str() {
        assert_eq!(
            get_test_input::<Monkey>()[0],
            Monkey {
                id: 0,
                items: VecDeque::from([79, 98]),
                operation: Operation::Multiply(Operand::Old, Operand::Scalar(19)),
                test_divisible_by: 23,
                divisible_true: 2,
                divisible_false: 3,
            }
        );
    }
}
