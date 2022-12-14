use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T: AddAssign<T> + Copy> AddAssign<&Pos<T>> for Pos<T> {
    fn add_assign(&mut self, rhs: &Pos<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<&Pos<T>> for &Pos<T>
where
    T: Sub<T> + Sub<Output = T> + Copy,
{
    type Output = Pos<T>;
    fn sub(self, rhs: &Pos<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Default> Default for Pos<T> {
    fn default() -> Self {
        Pos {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T> Add<&Pos<T>> for &Pos<T>
where
    T: Add<T> + Add<Output = T> + Copy,
{
    type Output = Pos<T>;
    fn add(self, rhs: &Pos<T>) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
