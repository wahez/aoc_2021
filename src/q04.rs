use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, io::BufRead, ops::RangeInclusive, str::FromStr};

use crate::{parsing::parse_by_line, regex_parse};

struct RangePair<T> {
    left: RangeInclusive<T>,
    right: RangeInclusive<T>,
}

impl<T: PartialOrd> RangePair<T> {
    fn one_included_in_other(&self) -> bool {
        let contains = |r1: &RangeInclusive<T>, r2: &RangeInclusive<T>| {
            r1.contains(r2.start()) && r1.contains(r2.end())
        };
        contains(&self.left, &self.right) || contains(&self.right, &self.left)
    }
    fn has_overlap(&self) -> bool {
        self.left.start() <= self.right.end() && self.right.start() <= self.left.end()
    }
}

impl FromStr for RangePair<i32> {
    // also works for other integers, but no trait to restrict that
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }
        match regex_parse!(REGEX, s, (i32, i32, i32, i32)) {
            None => Err("Range pair did not match".into()),
            Some(Err(e)) => Err(e),
            Some(Ok((s1, e1, s2, e2))) => Ok(RangePair {
                left: RangeInclusive::new(s1, e1),
                right: RangeInclusive::new(s2, e2),
            }),
        }
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for pair in parse_by_line::<RangePair<i32>>(buf) {
        if pair??.one_included_in_other() {
            count += 1;
        }
    }
    Ok(count)
}

pub fn b(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;
    for pair in parse_by_line::<RangePair<i32>>(buf) {
        if pair??.has_overlap() {
            count += 1;
        }
    }
    Ok(count)
}
