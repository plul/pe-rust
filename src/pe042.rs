//! This is a solution to [Project Euler Problem 42](https://projecteuler.net/problem=42).

use std::fmt::Display;
use std::str;

const DATA: &[u8] = include_bytes!("pe042-data.txt");

trait TriangleWord {
    fn is_triangle_word(&self) -> bool;
}

impl<T> TriangleWord for T
where
    T: AsRef<str>,
{
    fn is_triangle_word(&self) -> bool {
        let word: &str = self.as_ref();
        let word_value: usize = word.chars().map(|c| usize::from(char_score(&c))).sum();
        let triangle_numbers = (1..).map(|x| x * (x + 1) / 2);

        triangle_numbers
            .take_while(|&n| n <= word_value)
            .find(|&n| n == word_value)
            .is_some()
    }
}

pub fn solve() -> impl Display {
    str::from_utf8(DATA)
        .unwrap()
        .to_ascii_lowercase()
        .split(',')
        .map(|s| s.trim_matches('"'))
        .filter(|word| word.is_triangle_word())
        .count()
}

fn char_score(c: &char) -> u8 {
    assert!(c.is_ascii_lowercase());
    1 + (*c as u8 - b'a')
}

#[test]
fn example() {
    assert_eq!(char_score(&'s'), 19);
    assert_eq!(char_score(&'k'), 11);
    assert_eq!(char_score(&'y'), 25);
}
