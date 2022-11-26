use itertools::Itertools;
use std::cmp::Reverse;
use std::{error::Error, io::BufRead};

use crate::parsing::FromBufRead;

struct Elf(i32);

impl Elf {
    fn total_calories(&self) -> i32 {
        self.0
    }
}

impl FromBufRead for Elf {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let calories: Result<i32, _> = br
            .lines()
            .map_while(|line| -> Option<Result<i32, _>> {
                match line {
                    Err(_) => None,
                    Ok(l) if l.is_empty() => None,
                    Ok(l) => Some(l.parse()),
                }
            })
            .sum();
        Ok(Elf(calories?))
    }
}

pub fn a(mut buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let max_calories = Elf::read_iter(&mut buf)
        .map_ok(|elf| elf.total_calories())
        .max_by_key(|calories| (calories.is_err(), *calories.as_ref().unwrap_or(&0)))
        .ok_or("No elves read from file")??;
    Ok(max_calories)
}

pub fn b(mut buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut calories: Vec<_> = Elf::read_iter(&mut buf)
        .map_ok(|elf| elf.total_calories())
        .try_collect()?;
    calories.select_nth_unstable_by_key(2, |e| Reverse(*e));
    Ok(calories.iter().take(3).sum())
}
