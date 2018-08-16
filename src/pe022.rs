//! This is a solution to [Project Euler Problem 22](https://projecteuler.net/problem=22).

use std::fmt::Display;
use std::str;

const DATA: &[u8] = include_bytes!("pe022-data.txt");

pub fn solve() -> impl Display {
    let data: &str = str::from_utf8(DATA).unwrap();
    let mut names = load_names(data);

    names.sort();

    names
        .into_iter()
        .enumerate()
        .map(|(idx, name)| name_score(&name) * (idx + 1))
        .sum::<usize>()
}

/// Assumes lower case ascii characters
fn char_score(c: char) -> usize {
    1 + (c as u8 - b'a') as usize
}

fn name_score(name: &str) -> usize {
    name.chars().map(char_score).sum()
}

fn load_names(data: &str) -> Vec<String> {
    data.to_ascii_lowercase()
        .split(',')
        .map(|s| s.trim_matches('"').to_owned())
        .collect()
}

#[test]
fn test_colin() {
    assert_eq!(name_score("colin"), 53);
}
