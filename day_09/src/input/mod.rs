use std::fmt::Debug;
use std::str::FromStr;

pub fn get_input<ParseAs>() -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn get_test_input_1<ParseAs>() -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("test_input_1.txt")
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn get_test_input_2<ParseAs>() -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("test_input_2.txt")
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}
