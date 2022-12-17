use itertools::Itertools;
use std::{error::Error, io::BufRead, num::ParseIntError};

use crate::parsing::FromBufRead;

type Op = Box<dyn Fn(i64) -> i64>;

struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible_by: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}

fn read_op(line: &str) -> Result<Op, Box<dyn Error>> {
    let (op, rhs) = line.split_once(' ').ok_or("Expected one space")?;
    let get_op = |op: fn(_, _) -> _| -> Result<Op, ParseIntError> {
        if rhs == "old" {
            Ok(Box::new(move |i| op(i, i)))
        } else {
            let r = rhs.parse::<i64>()?;
            Ok(Box::new(move |i| op(i, r)))
        }
    };
    Ok(match op {
        "+" => get_op(|i, j| i + j)?,
        "*" => get_op(|i, j| i * j)?,
        _ => panic!(),
    })
}

impl FromBufRead for Monkey {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let mut read_line = |prefix| -> Result<String, Box<dyn Error>> {
            let mut line = String::new();
            while line.len() < 3 {
                line.clear();
                br.read_line(&mut line)?;
            }
            let line = line.strip_suffix('\n').unwrap_or(&line);
            let line = line
                .strip_prefix(prefix)
                .ok_or(format!("Could not read prefix {prefix}"))?;
            Ok(line.into())
        };
        let _ = read_line("Monkey ")?;
        let items = read_line("  Starting items: ")?
            .split(", ")
            .map(|s| s.parse())
            .try_collect()?;
        Ok(Monkey {
            items,
            op: read_op(&read_line("  Operation: new = old ")?)?,
            divisible_by: read_line("  Test: divisible by ")?.parse()?,
            true_monkey: read_line("    If true: throw to monkey ")?.parse()?,
            false_monkey: read_line("    If false: throw to monkey ")?.parse()?,
            inspected: 0,
        })
    }
}

impl Monkey {
    fn give_up_a(&mut self) -> Option<(usize, i64)> {
        self.items.pop().map(|i| {
            self.inspected += 1;
            let v = (self.op)(i) / 3;
            if v % self.divisible_by == 0 {
                (self.true_monkey, v)
            } else {
                (self.false_monkey, v)
            }
        })
    }
    fn give_up_b(&mut self) -> Option<(usize, i64)> {
        self.items.pop().map(|i| {
            self.inspected += 1;
            let v = (self.op)(i);
            if v % self.divisible_by == 0 {
                (self.true_monkey, v)
            } else {
                (self.false_monkey, v)
            }
        })
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = Monkey::read_iter(&mut buf).try_collect()?;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((new_monkey, item)) = monkeys[i].give_up_a() {
                monkeys[new_monkey].items.push(item);
            }
        }
    }
    let mut activity: Vec<_> = monkeys.iter().map(|m| m.inspected).collect();
    activity.sort(); // Could do select_nth, but the number of monkeys is small
    let a1 = activity.pop().ok_or("Too few activities")?;
    let a2 = activity.pop().ok_or("Too few activities")?;
    Ok(a1 * a2)
}

pub fn b(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = Monkey::read_iter(&mut buf).try_collect()?;
    let common_factor: i64 = monkeys.iter().map(|m| m.divisible_by).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some((new_monkey, mut item)) = monkeys[i].give_up_b() {
                if item > 1000000000 {
                    item %= common_factor;
                }
                monkeys[new_monkey].items.push(item);
            }
        }
    }
    let mut activity: Vec<_> = monkeys.iter().map(|m| m.inspected).collect();
    activity.sort(); // Could do select_nth, but the number of monkeys is small
    let a1 = activity.pop().ok_or("Too few activities")?;
    let a2 = activity.pop().ok_or("Too few activities")?;
    Ok(a1 * a2)
}
