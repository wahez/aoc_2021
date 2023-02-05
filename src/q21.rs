use itertools::Itertools;
use std::{collections::HashMap, error::Error, io::BufRead, mem::replace, str::FromStr};

use crate::parsing::parse_by_line;

#[derive(Clone)]
enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Clone)]
enum Shout {
    Equal((String, String)),
    None,
    Number(i64),
    Operation((String, Operation, String)),
}

struct Monkey {
    name: String,
    shout: Shout,
}

struct Monkeys {
    definitions: HashMap<String, Shout>,
}

impl Operation {
    fn exec(&self, lhs: i64, rhs: i64) -> i64 {
        use Operation::*;
        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Div => lhs / rhs,
            Mul => lhs * rhs,
        }
    }
    fn reverse_r(&self, parent: String, l: String) -> Shout {
        use Operation::*;
        match self {
            Add => Shout::Operation((parent, Sub, l)),
            Sub => Shout::Operation((l, Sub, parent)),
            Div => Shout::Operation((l, Div, parent)),
            Mul => Shout::Operation((parent, Div, l)),
        }
    }
    fn reverse_l(&self, parent: String, r: String) -> Shout {
        use Operation::*;
        match self {
            Add => Shout::Operation((parent, Sub, r)),
            Sub => Shout::Operation((r, Add, parent)),
            Div => Shout::Operation((r, Mul, parent)),
            Mul => Shout::Operation((parent, Div, r)),
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        match s {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "/" => Ok(Div),
            "*" => Ok(Mul),
            _ => Err("Unexpected operation"),
        }
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, def) = s.split_once(": ").ok_or("Expected ': '")?;
        if let Ok(n) = def.parse() {
            Ok(Monkey {
                name: name.to_string(),
                shout: Shout::Number(n),
            })
        } else {
            let (l, op, r) = def.split(' ').tuples().next().ok_or("Expected n <op> m")?;
            Ok(Monkey {
                name: name.to_string(),
                shout: Shout::Operation((l.parse()?, op.parse()?, r.parse()?)),
            })
        }
    }
}

impl Monkeys {
    fn new(definitions: impl Iterator<Item = Monkey>) -> Result<Monkeys, &'static str> {
        let definitions: HashMap<_, _> = definitions.map(|m| (m.name, m.shout)).collect();
        Ok(Monkeys { definitions })
    }

    fn solve(&self, name: &str) -> Result<i64, &'static str> {
        match self.definitions.get(name).ok_or("Could not find monkey")? {
            Shout::Equal((lmonkey, rmonkey)) => {
                Ok(self.solve(lmonkey).or_else(|_| self.solve(rmonkey))?)
            }
            Shout::None => Err("Not solved"),
            Shout::Number(n) => Ok(*n),
            Shout::Operation((lmonkey, op, rmonkey)) => {
                Ok(op.exec(self.solve(lmonkey)?, self.solve(rmonkey)?))
            }
        }
    }

    fn solve_or_reverse(&mut self, name: &str, parent: &str) -> Result<i64, &'static str> {
        let new_shout = match self
            .definitions
            .get(name)
            .ok_or("Could not find monkey")?
            .clone()
        {
            Shout::Equal((lmonkey, rmonkey)) => {
                let lresult = self.solve_or_reverse(&lmonkey, name);
                let rresult = self.solve_or_reverse(&rmonkey, name);
                match (lresult, rresult) {
                    (Ok(l), _) => Shout::Number(l),
                    (_, Ok(r)) => Shout::Number(r),
                    (Err(_), Err(_)) => unreachable!(),
                }
            }
            Shout::None => Shout::Equal((parent.to_string(), parent.to_string())),
            Shout::Number(n) => Shout::Number(n),
            Shout::Operation((lmonkey, op, rmonkey)) => {
                let lresult = self.solve_or_reverse(&lmonkey, name);
                let rresult = self.solve_or_reverse(&rmonkey, name);
                match (lresult, rresult) {
                    (Ok(l), Ok(r)) => {
                        let result = op.exec(l, r);
                        Shout::Number(result)
                    }
                    (Ok(_), Err(_)) => op.reverse_r(parent.to_string(), lmonkey.clone()),
                    (Err(_), Ok(_)) => op.reverse_l(parent.to_string(), rmonkey.clone()),
                    (Err(_), Err(_)) => unreachable!(),
                }
            }
        };
        if let Shout::Number(n) = new_shout {
            *self.definitions.get_mut(name).unwrap() = new_shout;
            Ok(n)
        } else {
            *self.definitions.get_mut(name).unwrap() = new_shout;
            Err("Not solved")
        }
    }
}

pub fn a(buf: impl BufRead) -> Result<i64, Box<dyn Error>> {
    let monkeys_with_name: Result<Result<Vec<_>, _>, _> = parse_by_line::<Monkey>(buf).collect();
    let monkeys = Monkeys::new(monkeys_with_name??.into_iter())?;
    Ok(monkeys.solve("root")?)
}

pub fn b(buf: impl BufRead) -> Result<i64, Box<dyn Error>> {
    let monkeys_with_name: Result<Result<Vec<_>, _>, _> = parse_by_line::<Monkey>(buf).collect();
    let mut monkeys = Monkeys::new(monkeys_with_name??.into_iter())?;
    *monkeys.definitions.get_mut("humn").ok_or("No humn found")? = Shout::None;
    let root = monkeys.definitions.get_mut("root").ok_or("No root found")?;
    if let Shout::Operation((l, _, r)) = replace(root, Shout::None) {
        *root = Shout::Equal((l, r));
    }
    let _root_result = monkeys.solve_or_reverse("root", "root");
    Ok(monkeys.solve("humn")?)
}
