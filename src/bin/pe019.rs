use std::time::Instant;

struct Date {
    weekday: usize,
    day: usize,
    month: usize,
    year: usize,
}

impl Date {
    /// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
    fn is_leap_year(&self) -> bool {
        let is_fourth_year = self.year % 4 == 0;
        let is_century = self.year % 100 == 0;
        let is_fourth_century = self.year % 400 == 0;
        is_fourth_year && (!is_century || is_fourth_century)
    }
}

struct SimpleCalendar {
    date: Date,
}

impl SimpleCalendar {
    /// Returns a SimpleCalendar set to Monday, January 1st 1900.
    fn new() -> SimpleCalendar {
        SimpleCalendar {
            date: Date {
                weekday: 0,
                day: 0,
                month: 0,
                year: 1900,
            },
        }
    }

    /// Go one day forward.
    fn advance(&mut self) {
        let weekday = (self.date.weekday + 1) % 7;

        let mut day = self.date.day + 1;
        match self.date.month {
            0 | 2 | 4 | 6 | 7 | 9 | 11 => {
                day %= 31;
            }
            3 | 5 | 8 | 10 => {
                day %= 30;
            }
            1 => {
                day %= if self.date.is_leap_year() {
                    29
                }
                else {
                    28
                };
            }
            _ => panic!("Unexpected month value"),
        }

        let month = match day {
            0 => (self.date.month + 1) % 12,
            _ => self.date.month,
        };

        let year = if month == 0 && day == 0 {
            self.date.year + 1
        } else {
            self.date.year
        };

        self.date.weekday = weekday;
        self.date.day = day;
        self.date.month = month;
        self.date.year = year;
    }
}

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let mut sc = SimpleCalendar::new();

    let mut result: usize = 0;
    while sc.date.year < 2001 {
        if sc.date.weekday == 6 && sc.date.day == 0 && sc.date.year >= 1901 {
            result += 1;
        }
        sc.advance();
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 171);
    }
}
