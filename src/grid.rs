use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::pos::Pos;

#[derive(Clone)]
pub struct Grid<T, PT> {
    values: Vec<T>,
    size: Pos<PT>,
}

impl<T, PT> Grid<T, PT>
where
    T: Clone,
    Pos<PT>: Copy,
{
    pub fn from_values(values: Vec<T>, size: Pos<PT>) -> Self {
        Grid { values, size }
    }
    pub fn size(&self) -> Pos<PT> {
        self.size
    }
}

impl<T: Clone> Grid<T, i16> {
    pub fn from_value(value: T, size: Pos<i16>) -> Self {
        Grid {
            values: vec![value; size.x as usize * size.y as usize],
            size,
        }
    }
}

impl<T> Grid<T, i16> {
    pub unsafe fn get_unchecked(&self, index: &Pos<i16>) -> &T {
        self.values
            .get_unchecked(index.x as usize + index.y as usize * self.size.x as usize)
    }
}

impl<T> Index<&Pos<i16>> for Grid<T, i16> {
    type Output = T;
    fn index(&self, index: &Pos<i16>) -> &Self::Output {
        &self.values[index.x as usize + index.y as usize * self.size.x as usize]
    }
}

impl<T> IndexMut<&Pos<i16>> for Grid<T, i16> {
    fn index_mut(&mut self, index: &Pos<i16>) -> &mut Self::Output {
        &mut self.values[index.x as usize + index.y as usize * self.size.x as usize]
    }
}

impl<T: Display> Display for Grid<T, i16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.values.chunks(self.size.x as usize) {
            for t in line {
                write!(f, "{:3}", t)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
