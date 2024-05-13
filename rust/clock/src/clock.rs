use chrono::{DateTime, Local, Timelike};

use crate::{
    ascii::{colonate, to_ascii},
    util::join2,
};

fn clock_digit(inp: impl ToString) -> Option<String> {
    let strs = to_ascii(inp)?;
    match strs.len() {
        1 => Some(strs[0].clone()),
        2 => Some(join2(strs[0].clone(), strs[1].clone())),
        _ => None,
    }
}

fn get_hour(now: DateTime<Local>) -> Option<String> {
    clock_digit(now.hour12().1)
}

fn get_min(now: DateTime<Local>) -> Option<String> {
    Some(format!("{:>0}", clock_digit(now.minute())?))
}

pub fn time_str(now: DateTime<Local>) -> Option<String> {
    Some(colonate(get_hour(now)?, get_min(now)?))
}
