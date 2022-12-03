use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Rucksack {
    pub items_0: Vec<char>,
    pub items_1: Vec<char>,
}

impl Rucksack {
    // a-z have priorities 1-26; A-Z have priorities 27-52
    pub fn priority(item: &char) -> usize {
        match item {
            'a'..='z' => *item as usize - 96, // a is 97
            'A'..='Z' => *item as usize - 38, // A is 65
            _ => panic!("Unknown item: {}", item),
        }
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let side_count = s.len() / 2;
        let items_0 = s.chars().take(side_count).collect();
        let items_1 = s.chars().skip(side_count).collect();
        Ok(Rucksack { items_0, items_1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_rucksack_priority() {
        assert_eq!(Rucksack::priority(&'a'), 1);
        assert_eq!(Rucksack::priority(&'z'), 26);
        assert_eq!(Rucksack::priority(&'A'), 27);
        assert_eq!(Rucksack::priority(&'Z'), 52);
    }

    #[test]
    fn test_rucksack_from_str() {
        assert_eq!(
            get_test_input::<Rucksack>()[0],
            Rucksack {
                items_0: "vJrwpWtwJgWr".chars().collect(),
                items_1: "hcsFMMfFFhFp".chars().collect(),
            }
        );
    }
}
