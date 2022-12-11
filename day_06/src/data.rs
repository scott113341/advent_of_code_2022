use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct DataStream {
    chars: Vec<char>,
}

// the start of a packet is indicated by a sequence of four characters that are all different
// The device will send your subroutine a datastream buffer (your puzzle input); your subroutine
// needs to identify the first position where the four most recently received characters were all
// different. Specifically, it needs to report the number of characters from the beginning of the
// buffer to the end of the first such four-character marker.

impl DataStream {
    // The start of a packet is four characters that are all different. Returns the number of
    // characters from the beginning to the end of the first start-of-packet marker.
    pub fn chars_to_first_packet(&self) -> usize {
        self.chars_to_first_n_distinct(4)
    }

    // A start-of-message marker is just like a start-of-packet marker, except it consists of 14
    // distinct characters rather than 4.
    pub fn chars_to_first_message(&self) -> usize {
        self.chars_to_first_n_distinct(14)
    }

    pub fn chars_to_first_n_distinct(&self, n_distinct: usize) -> usize {
        let mut count = 0;
        let mut last_n_chars = VecDeque::with_capacity(n_distinct);

        for c in self.chars.iter() {
            last_n_chars.push_front(c);
            count += 1;

            if last_n_chars.len() >= n_distinct {
                // Trims any excess items from the end
                last_n_chars.resize(n_distinct, c);

                // Returns count if all last N chars are distinct
                if last_n_chars.iter().collect::<HashSet<_>>().len() == n_distinct {
                    return count;
                }
            }
        }

        unreachable!();
    }
}

impl FromStr for DataStream {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DataStream {
            chars: s.chars().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_data_stream_chars_to_first_packet() {
        assert_eq!(get_test_input::<DataStream>().chars_to_first_packet(), 7);
    }

    #[test]
    fn test_data_stream_chars_to_first_message() {
        assert_eq!(get_test_input::<DataStream>().chars_to_first_message(), 19);
    }

    #[test]
    fn test_data_stream_from_str() {
        assert_eq!(get_test_input::<DataStream>().chars.len(), 30);
    }
}
