//! This is a solution to [Project Euler Problem 18](https://projecteuler.net/problem=18).

use common::maximum_path_sum;
use std::fmt::Display;
use std::str;

const DATA: &[u8] = include_bytes!("pe018-data.txt");

pub fn solve() -> impl Display {
    let data: &str = str::from_utf8(DATA).unwrap();
    maximum_path_sum::problem(data)
}
