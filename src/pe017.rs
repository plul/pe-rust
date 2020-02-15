//! This is a solution to [Project Euler Problem 17](https://projecteuler.net/problem=17).

use std::fmt::Display;

pub fn solve() -> impl Display {
    (1_usize..=1000).map(name).map(|name| name.len()).sum::<usize>()
}

fn name_fragment(n: usize) -> &'static str {
    match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => panic!("Cannot match {}", n),
    }
}

fn name(n: usize) -> String {
    match n {
        1..=19 => name_fragment(n).to_owned(),
        20..=99 => name_fragment(n / 10 * 10).to_owned() + name_fragment(n % 10),
        100..=999 => {
            let hundreds = name_fragment(n / 100).to_owned() + "hundred";
            match n % 100 {
                0 => hundreds,
                _ => hundreds + "and" + &name(n % 100),
            }
        }
        1000 => "".to_owned() + "one" + "thousand",
        _ => panic!("Cannot match {}", n),
    }
}

#[test]
fn number_1() {
    assert_eq!(name(1), "one");
}

#[test]
fn number_1000() {
    assert_eq!(name(1000), "onethousand");
}

#[test]
fn number_100() {
    assert_eq!(name(100), "onehundred");
}

#[test]
fn number_115() {
    assert_eq!(name(115), "onehundredandfifteen");
}

#[test]
fn number_342() {
    assert_eq!(name(342), "threehundredandfortytwo");
}
