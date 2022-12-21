use itertools::Itertools;
use std::{error::Error, io::BufRead};

use crate::parsing::parse_by_line;

#[derive(Clone, Copy)]
struct Number {
    n: i64,
    order: usize,
}

struct NumberList {
    numbers: Vec<Number>,
}

impl NumberList {
    fn mix(&mut self) {
        let size = self.numbers.len() as i64;
        for order in 0..self.numbers.len() {
            // This might seem slow, but using a double linked list will probably be slow as well
            let index = self.numbers.iter().position(|n| n.order == order).unwrap(); // should never fail
            let n = self.numbers.remove(index);
            let mut to = (index as i64 + n.n) % (size - 1);
            while to < 0 {
                to += size - 1;
            }
            self.numbers.insert(to as usize, n);
        }
    }

    fn sum_positions_after_0(&self, indices: &[usize]) -> Result<i64, &'static str> {
        let zero_pos = self
            .numbers
            .iter()
            .position(|n| n.n == 0)
            .ok_or("No 0 found")?;
        Ok(indices
            .iter()
            .map(|i| self.numbers[(zero_pos + i) % self.numbers.len() as usize].n)
            .sum())
    }
}

impl FromIterator<i64> for NumberList {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        NumberList {
            numbers: Vec::from_iter(
                iter.into_iter()
                    .enumerate()
                    .map(|(order, n)| Number { n, order }),
            ),
        }
    }
}

pub fn a(buf: impl BufRead) -> Result<i64, Box<dyn Error>> {
    let mut numbers = parse_by_line(buf).collect::<Result<Result<NumberList, _>, _>>()??;
    numbers.mix();
    Ok(numbers.sum_positions_after_0(&[1000usize, 2000, 3000])?)
}

pub fn b(buf: impl BufRead) -> Result<i64, Box<dyn Error>> {
    let mut numbers = parse_by_line::<i64>(buf)
        .map_ok(|rn| rn.map(|n| n * 811_589_153))
        .collect::<Result<Result<NumberList, _>, _>>()??;
    for _ in 0..10 {
        numbers.mix();
    }
    Ok(numbers.sum_positions_after_0(&[1000usize, 2000, 3000])?)
}
