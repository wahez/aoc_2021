use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, io::BufRead, mem::take, str::FromStr};

use crate::{
    parsing::{parse_by_line, FromBufRead},
    regex_parse,
};

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        match regex_parse!(REGEX, s, (usize, usize, usize)) {
            None => Err("Instruction could not be parsed".into()),
            Some(Err(e)) => Err(e),
            Some(Ok((amount, from, to))) => Ok(Instruction { amount, from, to }),
        }
    }
}

struct RowOfStacksOfCrates(Vec<Vec<char>>);

impl FromBufRead for RowOfStacksOfCrates {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let mut row_of_stacks = Vec::<Vec<char>>::new();
        for line in br.lines() {
            let line = line?;
            if line.starts_with(" 1   2") {
                break;
            }
            lazy_static! {
                static ref REGEX: Regex = Regex::new(r"\[([A-Z])\]").unwrap();
            }
            for cap in REGEX.captures_iter(&line) {
                let m = cap.get(1).unwrap();
                let stack = m.start() / 4;
                let c = m.as_str().chars().next().unwrap();
                if stack >= row_of_stacks.len() {
                    row_of_stacks.resize(stack + 1, Default::default());
                }
                row_of_stacks[stack].push(c);
            }
        }
        for stack in row_of_stacks.iter_mut() {
            stack.reverse();
        }
        match br.lines().next() {
            None => return Err("Expected empty line".into()),
            Some(Err(e)) => return Err(e.into()),
            Some(_) => {}
        };
        Ok(RowOfStacksOfCrates(row_of_stacks))
    }
}

impl RowOfStacksOfCrates {
    fn move_crates_one_by_one(&mut self, instruction: &Instruction) -> Result<(), &'static str> {
        for _ in 0..instruction.amount {
            let cr = self.0[instruction.from - 1]
                .pop()
                .ok_or("Could not pop from empty stack")?;
            self.0[instruction.to - 1].push(cr);
        }
        Ok(())
    }
    fn move_crate_group(&mut self, instruction: &Instruction) {
        let mut from_row = take(&mut self.0[instruction.from - 1]);
        let new_len = from_row.len() - instruction.amount;
        self.0[instruction.to - 1].extend_from_slice(&from_row[new_len..]);
        from_row.truncate(new_len);
        self.0[instruction.from - 1] = from_row;
    }
    fn top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }
}

pub fn a(mut buf: impl BufRead) -> Result<String, Box<dyn Error>> {
    let mut row = RowOfStacksOfCrates::read(&mut buf)?;
    for instruction in parse_by_line::<Instruction>(buf) {
        row.move_crates_one_by_one(&instruction??)?;
    }
    Ok(row.top_crates())
}

pub fn b(mut buf: impl BufRead) -> Result<String, Box<dyn Error>> {
    let mut row = RowOfStacksOfCrates::read(&mut buf)?;
    for instruction in parse_by_line::<Instruction>(buf) {
        row.move_crate_group(&instruction??);
    }
    Ok(row.top_crates())
}
