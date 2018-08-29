//! This is a collection of my own solutions to problems from [Project Euler](https://projecteuler.net/).
//!
//! Project Euler is a collection of math oriented problems meant to be solved through programming.

#![feature(try_from)]
#![feature(no_panic_pow)]

extern crate itertools;
extern crate bit_vec;
extern crate clap;
extern crate num;

mod common;
mod pe001;
mod pe002;
mod pe003;
mod pe004;
mod pe005;
mod pe006;
mod pe007;
mod pe008;
mod pe009;
mod pe010;
mod pe011;
mod pe012;
mod pe013;
mod pe014;
mod pe015;
mod pe016;
mod pe017;
mod pe018;
mod pe019;
mod pe020;
mod pe021;
mod pe022;
mod pe023;
mod pe024;
mod pe025;
mod pe026;
mod pe027;
mod pe028;
mod pe029;
mod pe030;
mod pe031;
mod pe032;
mod pe033;
mod pe034;
mod pe035;
mod pe036;
mod pe037;
mod pe038;
mod pe039;
mod pe208;

use clap::{App, Arg};
use std::time::Instant;

macro_rules! time {
    ($f:expr) => {{
        let t_0 = Instant::now();
        let result = $f;
        let t_1 = Instant::now();

        println!("Result: {}", result);
        println!("Time:   {:?}", t_1 - t_0);
    }};
}

fn main() {
    let matches = App::new("project-euler")
        .version("1.0")
        .author("Asger Juul Brunshøj <asgerbrunshoj@gmail.com>")
        .about("Solves Project Euler problems")
        .arg(
            Arg::with_name("PROBLEM")
                .help("Selects problem to solve")
                .required(true)
                .index(1),
        ).get_matches();

    match matches.value_of("PROBLEM").unwrap() {
        "1" => time!(pe001::solve()),
        "2" => time!(pe002::solve()),
        "3" => time!(pe003::solve()),
        "4" => time!(pe004::solve()),
        "5" => time!(pe005::solve()),
        "6" => time!(pe006::solve()),
        "7" => time!(pe007::solve()),
        "8" => time!(pe008::solve()),
        "9" => time!(pe009::solve()),
        "10" => time!(pe010::solve()),
        "11" => time!(pe011::solve()),
        "12" => time!(pe012::solve()),
        "13" => time!(pe013::solve()),
        "14" => time!(pe014::solve()),
        "15" => time!(pe015::solve()),
        "16" => time!(pe016::solve()),
        "17" => time!(pe017::solve()),
        "18" => time!(pe018::solve()),
        "19" => time!(pe019::solve()),
        "20" => time!(pe020::solve()),
        "21" => time!(pe021::solve()),
        "22" => time!(pe022::solve()),
        "23" => time!(pe023::solve()),
        "24" => time!(pe024::solve()),
        "25" => time!(pe025::solve()),
        "26" => time!(pe026::solve()),
        "27" => time!(pe027::solve()),
        "28" => time!(pe028::solve()),
        "29" => time!(pe029::solve()),
        "30" => time!(pe030::solve()),
        "31" => time!(pe031::solve()),
        "32" => time!(pe032::solve()),
        "33" => time!(pe033::solve()),
        "34" => time!(pe034::solve()),
        "35" => time!(pe035::solve()),
        "36" => time!(pe036::solve()),
        "37" => time!(pe037::solve()),
        "38" => time!(pe038::solve()),
        "39" => time!(pe039::solve()),
        "208" => time!(pe208::solve()),
        _ => println!("Unknown problem input"),
    }
}

#[test]
#[ignore]
fn verify_solutions() {
    assert_eq!(pe001::solve().to_string(), "233168");
    assert_eq!(pe002::solve().to_string(), "4613732");
    assert_eq!(pe003::solve().to_string(), "6857");
    assert_eq!(pe004::solve().to_string(), "906609");
    assert_eq!(pe005::solve().to_string(), "232792560");
    assert_eq!(pe006::solve().to_string(), "25164150");
    assert_eq!(pe007::solve().to_string(), "104743");
    assert_eq!(pe008::solve().to_string(), "23514624000");
    assert_eq!(pe009::solve().to_string(), "31875000");
    assert_eq!(pe010::solve().to_string(), "142913828922");
    assert_eq!(pe011::solve().to_string(), "70600674");
    assert_eq!(pe012::solve().to_string(), "76576500");
    assert_eq!(pe013::solve().to_string(), "5537376230");
    assert_eq!(pe014::solve().to_string(), "837799");
    assert_eq!(pe015::solve().to_string(), "137846528820");
    assert_eq!(pe016::solve().to_string(), "1366");
    assert_eq!(pe017::solve().to_string(), "21124");
    assert_eq!(pe018::solve().to_string(), "1074");
    assert_eq!(pe019::solve().to_string(), "171");
    assert_eq!(pe020::solve().to_string(), "648");
    assert_eq!(pe021::solve().to_string(), "31626");
    assert_eq!(pe022::solve().to_string(), "871198282");
    assert_eq!(pe023::solve().to_string(), "4179871");
    assert_eq!(pe024::solve().to_string(), "2783915460");
    assert_eq!(pe025::solve().to_string(), "4782");
    assert_eq!(pe026::solve().to_string(), "983");
    assert_eq!(pe027::solve().to_string(), "-59231");
    assert_eq!(pe028::solve().to_string(), "669171001");
    assert_eq!(pe029::solve().to_string(), "9183");
    assert_eq!(pe030::solve().to_string(), "443839");
    assert_eq!(pe031::solve().to_string(), "73682");
    assert_eq!(pe032::solve().to_string(), "45228");
    assert_eq!(pe033::solve().to_string(), "100");
    assert_eq!(pe034::solve().to_string(), "40730");
    assert_eq!(pe035::solve().to_string(), "55");
    assert_eq!(pe036::solve().to_string(), "872187");
    assert_eq!(pe037::solve().to_string(), "748317");
    assert_eq!(pe038::solve().to_string(), "932718654");
    assert_eq!(pe039::solve().to_string(), "840");
    assert_eq!(pe208::solve().to_string(), "331951449665644800");
}
