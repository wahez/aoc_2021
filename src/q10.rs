use itertools::Itertools;
use std::{error::Error, io::BufRead, str::FromStr};

use crate::parsing::parse_by_line;

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line {
            "noop" => Ok(Instruction::Noop),
            l => Ok(Instruction::Addx(
                l.strip_prefix("addx ").ok_or("Expected addx")?.parse()?,
            )),
        }
    }
}

struct SignalGenerator<I> {
    iter: I,
    value: i32,
    cached: Option<i32>,
}

impl<I> SignalGenerator<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            value: 1,
            cached: Some(0),
        }
    }
}

impl<E1, I> Iterator for SignalGenerator<I>
where
    E1: Error + 'static,
    I: Iterator<Item = Result<Result<Instruction, Box<dyn Error>>, E1>>,
{
    type Item = Result<i32, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cached.take() {
            None => self.iter.next().map(|r| {
                match r?? {
                    Instruction::Noop => {}
                    Instruction::Addx(n) => {
                        self.cached = Some(n);
                    }
                };
                Ok(self.value)
            }),
            Some(n) => {
                self.value += n;
                Some(Ok(self.value))
            }
        }
    }
}

pub fn a(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let generator = SignalGenerator::new(parse_by_line::<Instruction>(buf));
    let signals: Vec<_> = generator.try_collect()?;
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut total = 0i32;
    for c in cycles {
        total += (c as i32) * signals[c - 1];
    }
    Ok(total)
}

pub fn b(buf: impl BufRead) -> Result<&'static str, Box<dyn Error>> {
    let generator = SignalGenerator::new(parse_by_line::<Instruction>(buf));
    for (pos, sprite_pos) in generator.enumerate() {
        let pos = pos % 40;
        if (pos as i32 - sprite_pos?).abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }
        if pos == 39 {
            println!();
        }
    }
    // I did not feel like implementing OCR :)
    Ok("BUCACBUZ")
}
