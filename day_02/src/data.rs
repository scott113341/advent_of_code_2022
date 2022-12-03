use crate::data::Choice::{Paper, Scissors};
use std::str::FromStr;
use Choice::*;
use RoundOutcome::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(format!("Unknown choice: {}", s)),
        }
    }
}

impl Choice {
    pub fn choice_for_outcome(&self, outcome: &RoundOutcome) -> Choice {
        match (self, outcome) {
            (Rock, Loss) => Scissors,
            (Rock, Tie) => Rock,
            (Rock, Win) => Paper,
            (Paper, Loss) => Rock,
            (Paper, Tie) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Scissors, Tie) => Scissors,
            (Scissors, Win) => Rock,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum RoundOutcome {
    Loss,
    Tie,
    Win,
}

impl FromStr for RoundOutcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Tie),
            "Z" => Ok(Win),
            _ => Err(format!("Unknown choice: {}", s)),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Round {
    pub(crate) them: Choice,
    pub(crate) me: Choice,
}

impl Round {
    // The score for a single round is the score for the shape you selected (1 for Rock, 2 for
    // Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if
    // the round was a draw, and 6 if you won).
    pub fn my_score(&self) -> usize {
        let selection_score = match self.me {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let round_score = match self.outcome() {
            Loss => 0,
            Tie => 3,
            Win => 6,
        };

        selection_score + round_score
    }

    pub fn outcome(&self) -> RoundOutcome {
        match (self.them, self.me) {
            (Rock, Rock) => Tie,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Loss,

            (Paper, Rock) => Loss,
            (Paper, Paper) => Tie,
            (Paper, Scissors) => Win,

            (Scissors, Rock) => Win,
            (Scissors, Paper) => Loss,
            (Scissors, Scissors) => Tie,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_my_score() {
        assert_eq!(
            Round {
                them: Rock,
                me: Paper
            }
            .my_score(),
            8
        );
        assert_eq!(
            Round {
                them: Paper,
                me: Rock
            }
            .my_score(),
            1
        );
        assert_eq!(
            Round {
                them: Scissors,
                me: Scissors
            }
            .my_score(),
            6
        );
    }
}
