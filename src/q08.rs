use itertools::{iproduct, Itertools};
use std::{error::Error, io::BufRead};

use crate::parsing::FromBufRead;

struct Grid(Vec<Vec<u8>>);

impl FromBufRead for Grid {
    type Error = std::io::Error;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        Ok(Grid(
            br.lines()
                .map_ok(|s| Vec::from_iter(s.bytes()))
                .try_collect()?,
        ))
    }
}

impl Grid {
    fn get_highest_scenic_score(&self) -> usize {
        iproduct!(0..self.0.len(), 0..self.0[0].len())
            .map(|(r, c)| self.get_scenic_score(r, c))
            .max()
            .unwrap()
    }
    fn get_scenic_score(&self, row: usize, col: usize) -> usize {
        let height = self.0[row][col];
        let vis_to_right = self.0[row][col + 1..]
            .iter()
            .position(|h| *h >= height)
            .map_or(self.0[row].len() - col - 1, |p| p + 1);
        let vis_to_left = self.0[row][0..col]
            .iter()
            .rev()
            .position(|h| *h >= height)
            .map_or(col, |p| p + 1);
        let vis_to_top = self.0[0..row]
            .iter()
            .rev()
            .position(|r| r[col] >= height)
            .map_or(row, |p| p + 1);
        let vis_to_bot = self.0[row + 1..]
            .iter()
            .position(|r| r[col] >= height)
            .map_or(self.0.len() - row - 1, |p| p + 1);
        vis_to_left * vis_to_right * vis_to_top * vis_to_bot
    }
    fn count_visible(&self) -> usize {
        iproduct!(0..self.0.len(), 0..self.0[0].len())
            .filter(|(r, c)| self.is_visible(*r, *c))
            .count()
    }
    fn is_visible(&self, row: usize, col: usize) -> bool {
        let height = self.0[row][col];
        self.0[row][col + 1..].iter().all(|h| *h < height)
            || self.0[row][..col].iter().all(|h| *h < height)
            || self.0[0..row].iter().map(|r| r[col]).all(|h| h < height)
            || self.0[row + 1..].iter().map(|r| r[col]).all(|h| h < height)
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    Ok(Grid::read(&mut buf)?.count_visible())
}

pub fn b(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    Ok(Grid::read(&mut buf)?.get_highest_scenic_score())
}
