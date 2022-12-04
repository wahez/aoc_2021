use std::{error::Error, io::BufRead, ops::RangeInclusive, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::parsing::parse_by_line;

struct RangePair {
    left: RangeInclusive<i32>,
    right: RangeInclusive<i32>,
}

impl RangePair {
    fn one_included_in_other(&self) -> bool {
        let contains = |r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>| {
            r1.contains(r2.start()) && r1.contains(r2.end())
        };
        contains(&self.left, &self.right) || contains(&self.right, &self.left)
    }
    fn has_overlap(&self) -> bool {
        self.left.start() <= self.right.end() && self.right.start() <= self.left.end()
    }
}

impl FromStr for RangePair {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }
        let Some(captures) = REGEX.captures(s) else {
            return Err("Range pair did not match".into());
        };
        let get_match = |i| captures.get(i).unwrap().as_str().parse();
        Ok(RangePair {
            left: RangeInclusive::new(get_match(1)?, get_match(2)?),
            right: RangeInclusive::new(get_match(3)?, get_match(4)?),
        })
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for pair in parse_by_line::<RangePair>(buf) {
        if pair??.one_included_in_other() {
            count += 1;
        }
    }
    Ok(count)
}

pub fn b(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;
    for pair in parse_by_line::<RangePair>(buf) {
        if pair??.has_overlap() {
            count += 1;
        }
    }
    Ok(count)
}
