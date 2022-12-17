use itertools::Itertools;
use std::{error::Error, io::BufRead, str::FromStr};

use crate::parsing::parse_by_line;

// might be faster in a bitset, but performance is not an issue (yet)
struct Items(u64);

impl Items {
    fn common(&self, other: &Items) -> Items {
        Items(self.0 & other.0)
    }
    fn try_to_single_prio(self) -> Option<i32> {
        if self.0.count_ones() == 1 {
            Some(self.0.trailing_zeros() as i32)
        } else {
            None
        }
    }
}

impl FromStr for Items {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0u64;
        for b in s.bytes() {
            let prio = match b {
                b'a'..=b'z' => (b - b'a' + 1) as usize,
                b'A'..=b'Z' => (b - b'A' + 27) as usize,
                _ => Err("Unexpected item")?,
            };
            bits |= 1 << prio;
        }
        Ok(Items(bits))
    }
}

pub fn a(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut total_prio = 0;
    for line in buf.lines() {
        let line = line?;
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let common = Items::from_str(comp1)?.common(&Items::from_str(comp2)?);
        total_prio += common
            .try_to_single_prio()
            .ok_or("Expected one common item")?;
    }
    Ok(total_prio)
}

pub fn b(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    for (e1, e2, e3) in parse_by_line::<Items>(buf).tuples() {
        sum += e1??
            .common(&e2??)
            .common(&e3??)
            .try_to_single_prio()
            .ok_or("No single common item")?;
    }
    Ok(sum)
}
