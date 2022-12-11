use std::{error::Error, io::BufRead, str::FromStr};

use crate::parsing::parse_by_line;

type Pos = crate::pos::Pos<i32>;

struct Move {
    direction: Pos,
    steps: i32,
}

impl FromStr for Move {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None => Err("No space in instruction".into()),
            Some((l, r)) => {
                let direction = match l {
                    "D" => Pos { x: 0, y: -1 },
                    "L" => Pos { x: -1, y: 0 },
                    "R" => Pos { x: 1, y: 0 },
                    "U" => Pos { x: 0, y: 1 },
                    _ => Err("Could not parse Direction")?,
                };
                Ok(Move {
                    direction,
                    steps: r.parse()?,
                })
            }
        }
    }
}

impl Pos {
    fn follow(&mut self, head: &Pos) -> bool {
        let diff = head - self;
        if diff.abs2() > 3 {
            *self += &diff.signum();
            true
        } else {
            false
        }
    }
    fn abs2(&self) -> i32 {
        let a = self.x;
        let b = self.y;
        a * a + b * b
    }
    fn signum(&self) -> Pos {
        Pos {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut head = Pos::default();
    let mut tail = Pos::default();
    let mut history = Vec::new();
    history.push(tail);
    for mov in parse_by_line::<Move>(buf) {
        let mov = mov??;
        for _ in 0..mov.steps {
            head += &mov.direction;
            if tail.follow(&head) {
                history.push(tail);
            }
        }
    }
    history.sort();
    history.dedup();
    Ok(history.len())
}

pub fn b(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut knots = [Pos::default(); 10];
    let mut history = Vec::new();
    history.push(knots[9]);
    for mov in parse_by_line::<Move>(buf) {
        let mov = mov??;
        'mov: for _ in 0..mov.steps {
            knots[0] += &mov.direction;
            for i in 1..10 {
                let prev = knots[i - 1];
                if !knots[i].follow(&prev) {
                    continue 'mov;
                }
            }
            history.push(knots[9]);
        }
    }
    history.sort();
    history.dedup();
    Ok(history.len())
}
