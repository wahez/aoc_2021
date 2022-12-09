use std::{
    error::Error,
    io::BufRead,
    ops::{AddAssign, Sub},
    str::FromStr,
};

use crate::parsing::parse_by_line;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

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
                    "D" => Pos(0, -1),
                    "L" => Pos(-1, 0),
                    "R" => Pos(1, 0),
                    "U" => Pos(0, 1),
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

impl AddAssign<&Pos> for Pos {
    fn add_assign(&mut self, rhs: &Pos) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<&Pos> for &Pos {
    type Output = Pos;
    fn sub(self, rhs: &Pos) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
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
        let a = self.0;
        let b = self.1;
        a * a + b * b
    }
    fn signum(&self) -> Pos {
        Pos(self.0.signum(), self.1.signum())
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);
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
    let mut knots = [Pos(0, 0); 10];
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
