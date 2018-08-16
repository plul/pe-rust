//! This is a solution to [Project Euler Problem 208](https://projecteuler.net/problem=208).
//!
//! The key insight here is that there is a property that must be true if and only if the robot ends up in the same place it started, facing the same way.
//! The robot must have faced in each possible direction an equal number of times.
//! After that, it is just am matter of recursively exploring paths and caching states to prevent an exponential blowup.
//! A state contains the current facing, and the number of times the robot has faced every possible direction, in its current path.
//! If all those are equal, the states can be considered as duplicates, in the sense that recursive exploration of the subtrees of paths are guaranteed to produce the same number of ways to end up where the robot started.

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
enum Facing {
    Deg18,
    Deg90,
    Deg162,
    Deg234,
    Deg306,
}

#[derive(Clone)]
struct State {
    arcs_remaining: u16,
    facing: Facing,
    deg_18: u8,
    deg_90: u8,
    deg_162: u8,
    deg_234: u8,
    deg_306: u8,
}

impl State {
    /// Represent state uniquely as a u64.
    fn as_unique_u64(&self) -> u64 {
        let f: u64 = match self.facing {
            Facing::Deg18 => 0,
            Facing::Deg90 => 1,
            Facing::Deg162 => 2,
            Facing::Deg234 => 3,
            Facing::Deg306 => 4,
        };

        (self.arcs_remaining as u64)
            | (self.deg_18 as u64) << 16
            | (self.deg_90 as u64) << 24
            | (self.deg_162 as u64) << 32
            | (self.deg_234 as u64) << 40
            | (self.deg_306 as u64) << 48
            | f << 56
    }

    fn branch(&self) -> (Self, Self) {
        let mut s1 = self.clone();
        let mut s2 = self.clone();
        s1.arcs_remaining -= 1;
        s2.arcs_remaining -= 1;

        match self.facing {
            Facing::Deg18 => {
                s1.facing = Facing::Deg306;
                s1.deg_306 += 1;

                s2.facing = Facing::Deg90;
                s2.deg_90 += 1;
            }
            Facing::Deg90 => {
                s1.facing = Facing::Deg18;
                s1.deg_18 += 1;

                s2.facing = Facing::Deg162;
                s2.deg_162 += 1;
            }
            Facing::Deg162 => {
                s1.facing = Facing::Deg90;
                s1.deg_90 += 1;

                s2.facing = Facing::Deg234;
                s2.deg_234 += 1;
            }
            Facing::Deg234 => {
                s1.facing = Facing::Deg162;
                s1.deg_162 += 1;

                s2.facing = Facing::Deg306;
                s2.deg_306 += 1;
            }
            Facing::Deg306 => {
                s1.facing = Facing::Deg234;
                s1.deg_234 += 1;

                s2.facing = Facing::Deg18;
                s2.deg_18 += 1;
            }
        };

        (s1, s2)
    }
}

pub fn solve() -> impl Display {
    problem(70)
}

/// By only going clockwise in the first iteration, the inherent symmetry in the problem is leveraged.
fn problem(arcs: u16) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    2 * closed_loops_rec(
        State {
            arcs_remaining: arcs - 1,
            facing: Facing::Deg18,
            deg_18: 1,
            deg_90: 0,
            deg_162: 0,
            deg_234: 0,
            deg_306: 0,
        },
        &mut cache,
    )
}

/// Recursively explore new states
fn closed_loops_rec(s: State, cache: &mut HashMap<u64, u64>) -> u64 {
    if s.arcs_remaining == 0 {
        return match s.facing {
            Facing::Deg90 => {
                if s.deg_18 == s.deg_90
                    && s.deg_18 == s.deg_162
                    && s.deg_18 == s.deg_234
                    && s.deg_18 == s.deg_306
                {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        };
    }

    // Check if this input has been cached.
    let state_as_unique_u64 = s.as_unique_u64();
    if let Some(x) = cache.get(&state_as_unique_u64) {
        return *x;
    }

    let (s1, s2) = s.branch();
    let l: u64 = closed_loops_rec(s1, cache) + closed_loops_rec(s2, cache);

    cache.insert(state_as_unique_u64, l);

    l
}

#[test]
fn example() {
    assert_eq!(problem(25), 70932);
}
