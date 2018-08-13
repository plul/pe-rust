# pe-rust

This is a collection of my own solutions to problems from [Project Euler](https://projecteuler.net/).
Project Euler is a collection of math oriented problems meant to be solved through programming.

These solutions are implemented in the Rust programming language.
I solve these problems for fun, but also as an exercise in writing Rust.

## Spoiler warning

These solutions are obviously spoilers for anyone wanting to solve the problems themselves, so if you want to do that, please turn away.

## Organization

Some techniques are useful for multiple problems, such as finding primes or separating a number into its constituent digits.
Tools for these purposes are collected into a library, and the binary crates leverage that library.