use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct SectionPair {
    start_1: usize,
    end_1: usize,
    start_2: usize,
    end_2: usize,
}

impl SectionPair {
    // Ugly and fast
    pub fn one_fully_contained(&self) -> bool {
        let r0_contains_r1 = self.start_1 <= self.start_2 && self.end_1 >= self.end_2;
        let r1_contains_r0 = self.start_2 <= self.start_1 && self.end_2 >= self.end_1;
        r0_contains_r1 || r1_contains_r0
    }

    // Funny and slow
    pub fn any_overlap(&self) -> bool {
        let r0 = self.start_1..=self.end_1;
        let r1 = self.start_2..=self.end_2;
        r0.into_iter().any(|r0_section| r1.contains(&r0_section))
    }
}

impl FromStr for SectionPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }

        let caps = RE.captures(s).ok_or("Meow".to_string())?;
        let parse_cap = |idx: usize| -> Result<usize, String> {
            caps.get(idx)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .or(Err(format!("Failed to parse: {:?}", caps.get(idx))))
        };

        Ok(SectionPair {
            start_1: parse_cap(1)?,
            end_1: parse_cap(2)?,
            start_2: parse_cap(3)?,
            end_2: parse_cap(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_section_pair_from_str() {
        assert_eq!(
            get_test_input::<SectionPair>()[0],
            SectionPair {
                start_1: 2,
                end_1: 4,
                start_2: 6,
                end_2: 8,
            }
        );
    }
}
