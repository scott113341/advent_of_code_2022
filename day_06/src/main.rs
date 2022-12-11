mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 06");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// How many characters need to be processed before the first start-of-packet marker is detected?
fn part_1(data_stream: DataStream) -> usize {
    data_stream.chars_to_first_packet()
}

fn part_2(data_stream: DataStream) -> usize {
    data_stream.chars_to_first_message()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 19);
    }
}
