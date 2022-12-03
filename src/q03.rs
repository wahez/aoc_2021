use itertools::Itertools;
use std::{collections::HashSet, error::Error, io::BufRead, str::FromStr};

use crate::parsing::parse_by_line;

// might be faster in a bitset, but performance is not an issue (yet)
struct Items(HashSet<u8>);

impl Items {
    fn common(&self, other: &Items) -> Items {
        Items(self.0.intersection(&other.0).copied().collect())
    }
    fn get_item_prio(&self) -> Option<i32> {
        if self.0.len() == 1 {
            let b = *self.0.iter().next().unwrap();
            let prio = if b >= b'a' {
                b - b'a' + 1
            } else {
                b - b'A' + 27
            };
            Some(prio as i32)
        } else {
            None
        }
    }
}

impl FromStr for Items {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Items(s.bytes().collect()))
    }
}

pub fn a(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut total_prio = 0;
    for line in buf.lines() {
        let line = line?;
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let common = Items::from_str(comp1)?.common(&Items::from_str(comp2)?);
        total_prio += common.get_item_prio().ok_or("Expected one common item")?;
    }
    Ok(total_prio)
}

pub fn b(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    for (e1, e2, e3) in parse_by_line::<Items>(buf).tuples() {
        sum += e1??
            .common(&e2??)
            .common(&e3??)
            .get_item_prio()
            .ok_or("No single common item")?;
    }
    Ok(sum)
}
