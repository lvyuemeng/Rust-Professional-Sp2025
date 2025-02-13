use std::{cmp::Ordering, ops::Deref};

// The general way to deduce weeks and Chinese new year is hard.
// Recall 2025.01.01 is We, CNY of 2025 is 01.29.
const WEEK_IN_1_1: u32 = 3;
pub fn time_info(time: &str) -> String {
    let parts: Vec<u32> = time.split('-').map(|s| s.parse::<u32>().unwrap()).collect();
    let year = parts[0];
    let month = parts[1];
    let day = parts[2];

    let cur_date = Date::new(year, month, day);
    let (_, days_passed, days_rem) = cur_date.as_num_tuple();
    let cny_2025 = Date::get_cny_date(2025);
    let cny_2026 = Date::get_cny_date(2026);

    let day_until_cny = {
        if cur_date < cny_2025 {
            let cny_passed = cny_2025.days_passed;
            cny_passed - days_passed
        } else {
            let cny_passed = cny_2026.days_passed;
            cny_passed + days_rem
        }
    };

    let (week_num, week_day) = cur_date.calc_week();
    let next_trading_day = cur_date.get_next_trading_day();

    let res = format!(
        "{},{},{},{},{},{}",
        week_num, week_day, days_passed, days_rem, day_until_cny, next_trading_day
    );
    dbg!(&res);
    res
}

struct Date {
    year: u32,
    month: u32,
    day: u32,
    pub days_in_year: u32,
    pub days_passed: u32,
    pub days_rem: u32,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.year.partial_cmp(&other.year)? {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        match self.month.partial_cmp(&other.month)? {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        self.day.partial_cmp(&other.day)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl Date {
    fn new(year: u32, month: u32, day: u32) -> Self {
        let (days_in_year, days_passed, days_rem) = Date::calc_days(year, month, day);
        Date {
            year,
            month,
            day,
            days_in_year,
            days_passed,
            days_rem,
        }
    }
    fn get_cny_date(year: u32) -> Date {
        match year {
            2025 => Date::new(2025, 1, 29),
            2026 => Date::new(2025, 2, 17),
            _ => unreachable!(),
        }
    }

    fn get_closed_days() -> Vec<(Date, Date)> {
        vec![
            (Date::new(2024, 12, 31), Date::new(2025, 1, 1)),
            (Date::new(2025, 1, 27), Date::new(2025, 2, 4)),
            (Date::new(2025, 4, 3), Date::new(2025, 4, 6)),
            (Date::new(2025, 4, 30), Date::new(2025, 5, 5)),
            (Date::new(2025, 5, 30), Date::new(2025, 6, 2)),
            (Date::new(2025, 9, 31), Date::new(2025, 10, 8)),
            (Date::new(2025, 12, 31), Date::new(2026, 1, 1)),
        ]
    }

    fn contains(&self, l: &Date, r: &Date) -> bool {
        self >= l && self <= r
    }

    fn get_next_trading_day(&self) -> u32 {
        let closed_days = Date::get_closed_days();
        for (l, r) in closed_days {
            if self.contains(&l, &r) {
                let gap = if self.year == r.year {
                    r.days_passed - self.days_passed
                } else {
                    r.days_passed + self.days_rem
                };

                return gap;
            }
        }

        // normal day
        let (_, week_day) = self.calc_week();
        let next_trading_day = {
            match week_day {
                1..=4 => 0,
                5 => 2,
                6 => 1,
                7 => 0,
                _ => unreachable!(),
            }
        };
        next_trading_day
    }

    fn calc_week(&self) -> (u32, u32) {
        assert!(self.year == 2025);
        let week_num = if self.days_passed < 4 {
            1
        } else if self.month == 12 && self.day >= 29 {
            1
        } else {
            (self.days_passed) / 7 + 1
        };

        let week_day = {
            let mut week_day = (self.days_passed - 1 + WEEK_IN_1_1) % 7;
            if week_day == 0 {
                week_day = 7
            }
            week_day
        };

        (week_num, week_day)
    }

    fn as_date(&self) -> (u32, u32, u32) {
        (self.year, self.month, self.day)
    }

    fn as_num_tuple(&self) -> (u32, u32, u32) {
        (self.days_in_year, self.days_passed, self.days_rem)
    }

    fn calc_days(year: u32, month: u32, day: u32) -> (u32, u32, u32) {
        let is_leap = if year % 400 == 0 {
            true
        } else if year % 100 == 0 {
            false
        } else {
            year % 4 == 0
        };

        let days_in_year = if is_leap { 366 } else { 365 };

        let days_in_month = vec![
            31,
            28 + if is_leap { 1 } else { 0 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];

        let days_passed = {
            let mut days_passed = day;
            for m in 0..month - 1 {
                days_passed += days_in_month[m as usize];
            }
            days_passed
        };

        let days_rem = days_in_year - days_passed;
        (days_in_year, days_passed, days_rem)
    }
}
