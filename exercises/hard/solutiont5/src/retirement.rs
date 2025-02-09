use std::{
    ops::{Div, Mul, Rem},
    str::FromStr,
};

const FEMALE50: &str = "原法定退休年龄50周岁女职工";
const FEMALE55: &str = "原法定退休年龄55周岁女职工";
const MALE: &str = "男职工";
pub fn retire_time(time: &str, tp: &str) -> String {
    let parts: Vec<u32> = time.split('-').map(|s| s.parse().unwrap()).collect();
    let birth_year = parts[0];
    let birth_month = parts[1];

    let tp = Employee::from_str(tp).unwrap();

    let (original_retire_age, factor, bound) = tp.infos();

    let period_months = {
        if birth_year + original_retire_age < 2025 {
            0
        } else {
            (birth_year + original_retire_age - 2025) * 12 + birth_month
        }
    };

    let extend_month = {
        match period_months {
            0 => 0,
            num if num < bound => num.div_ceil(factor),
            _ => bound,
        }
    };

    let retire_age = (original_retire_age as f64) + (extend_month as f64).div(12.0);

    let (retire_year, retire_month) = add_month(
        (birth_year + original_retire_age, birth_month),
        extend_month,
    );

    let res = format!(
        "{}-{:02},{},{}",
        retire_year,
        retire_month,
        format_num(retire_age),
        extend_month
    );
    dbg!(&res);
    res
}

enum Employee {
    Male,
    Female50,
    Female55,
}

impl Employee {
    fn infos(&self) -> (u32, u32, u32) {
        match self {
            Employee::Male => (60, 4, 36),
            Employee::Female55 => (55, 4, 36),
            Employee::Female50 => (50, 2, 60),
        }
    }
}

impl FromStr for Employee {
    // presume input is correct.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            FEMALE50 => Ok(Employee::Female50),
            FEMALE55 => Ok(Employee::Female55),
            MALE => Ok(Employee::Male),
            _ => Err("Error input".to_string()),
        }
    }

    type Err = String;
}

fn add_month(ym: (u32, u32), month: u32) -> (u32, u32) {
    let (y, m) = ym;
    let new_m = m - 1 + month;
    (y + new_m.div(12), new_m.rem(12) + 1)
}

fn format_num(num: f64) -> String {
    if num.fract() == 0.0 {
        format!("{:.0}", num)
    } else {
        format!("{:.2}", num)
    }
}
