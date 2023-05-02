use std::str::FromStr;
use Direction::*;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Step {
    pub direction: Direction,
    pub magnitude: isize,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction_str = s.split(' ').next().ok_or("No instruction")?;
        let direction = match direction_str {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => Err(format!("Unknown direction: {}", direction_str))?,
        };

        let magnitude = s
            .split(' ')
            .nth(1)
            .ok_or("No magnitude")?
            .parse()
            .map_err(|_| "Invalid magnitude")?;

        Ok(Step {
            direction,
            magnitude,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input_1;

    #[test]
    fn test_step_from_str() {
        assert_eq!(
            get_test_input_1::<Step>()[0],
            Step {
                direction: Right,
                magnitude: 4
            }
        );
    }
}
