use itertools::Itertools;
use std::{
    error::Error,
    fmt::Display,
    io::BufRead,
    ops::{Index, IndexMut},
    str::FromStr,
};
// use std::{thread::sleep, time::Duration};

use crate::{
    grid::Grid,
    parsing::{parse_by_line, FromBufRead},
    pos::Pos,
};

impl Pos<i16> {
    fn signum(&self) -> Pos<i16> {
        Pos {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

#[derive(Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Clone)]
struct Cave {
    grid: Grid<Material, i16>,
    offset: Pos<i16>,
}

enum FillResult {
    Filled,
    Blocked,
    FloorAtX(i16),
}

impl Cave {
    fn new(rocks: &Vec<Rock>, min: Pos<i16>, max: Pos<i16>) -> Cave {
        let mut cave = Cave {
            grid: Grid::from_value(Material::Air, &max - &min),
            offset: min,
        };
        for rock in rocks {
            for (start, end) in rock.0.iter().tuple_windows() {
                let direction = (end - start).signum();
                let mut current = *start;
                cave[&current] = Material::Rock;
                loop {
                    current += &direction;
                    cave[&current] = Material::Rock;
                    if &current == end {
                        break;
                    }
                }
            }
        }
        cave
    }
    fn fill_next_from(&mut self, pos: &Pos<i16>) -> FillResult {
        let mut x = pos.x - self.offset.x;
        for y in pos.y..self.grid.size().y - 1 {
            x = match [x, x - 1, x + 1].iter().find(|nextx| {
                let pos = Pos {
                    x: **nextx,
                    y: y + 1,
                };
                matches!(
                    // self.grid[&pos],
                    unsafe { self.grid.get_unchecked(&pos) }, // this is faster but only safe because the grid is wide enough
                    Material::Air
                )
            }) {
                Some(nextx) => *nextx,
                None => {
                    self.grid[&Pos { x, y }] = Material::Sand;
                    if y == 0 {
                        return FillResult::Blocked;
                    } else {
                        return FillResult::Filled;
                    }
                }
            }
        }
        FillResult::FloorAtX(x + self.offset.x)
    }
}

impl Index<&Pos<i16>> for Cave {
    type Output = Material;
    fn index(&self, index: &Pos<i16>) -> &Self::Output {
        &self.grid[&(index - &self.offset)]
    }
}

impl IndexMut<&Pos<i16>> for Cave {
    fn index_mut(&mut self, index: &Pos<i16>) -> &mut Self::Output {
        &mut self.grid[&(index - &self.offset)]
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, ".")?,
            Self::Rock => write!(f, "#")?,
            Self::Sand => write!(f, "o")?,
        }
        Ok(())
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.grid.size();
        for y in 0..size.y {
            for x in 0..size.x {
                write!(f, "{}", self.grid[&Pos { x, y }])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Rock(Vec<Pos<i16>>);

impl FromStr for Rock {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        for point in s.split(" -> ") {
            let (x, y) = point.split_once(',').ok_or("Expected ,")?;
            points.push(Pos {
                x: x.parse()?,
                y: y.parse()?,
            });
        }
        Ok(Rock(points))
    }
}

impl FromBufRead for Cave {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let rocks: Vec<Rock> =
            parse_by_line::<Rock>(br).try_collect::<Result<_, _>, Result<_, _>, _>()??;
        let minx = rocks
            .iter()
            .flat_map(|rock| rock.0.iter().map(|p| p.x))
            .min()
            .ok_or("Too few rocks")?;
        let maxx = rocks
            .iter()
            .flat_map(|rock| rock.0.iter().map(|p| p.x))
            .max()
            .ok_or("Too few rocks")?;
        let maxy = rocks
            .iter()
            .flat_map(|rock| rock.0.iter().map(|p| p.y))
            .max()
            .ok_or("Too few rocks")?;
        let min = Pos {
            x: minx - maxy - 10,
            y: 0,
        };
        let max = Pos {
            x: maxx + maxy + 10,
            y: maxy + 2,
        };
        Ok(Cave::new(&rocks, min, max))
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut cave = Cave::read(&mut buf)?;
    for i in 0.. {
        // println!("{cave}");
        // sleep(Duration::from_millis(30));
        match cave.fill_next_from(&Pos { x: 500, y: 0 }) {
            FillResult::Blocked => Err("Cave filled up")?,
            FillResult::FloorAtX(_) => return Ok(i),
            FillResult::Filled => {}
        }
    }
    unreachable!()
}

pub fn b(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut cave = Cave::read(&mut buf)?;
    for i in 1.. {
        // println!("{cave}");
        // sleep(Duration::from_millis(1));
        match cave.fill_next_from(&Pos { x: 500, y: 0 }) {
            FillResult::Blocked => return Ok(i),
            FillResult::FloorAtX(x) => {
                let y = cave.grid.size().y - 1;
                cave[&Pos { x, y }] = Material::Sand;
            }
            FillResult::Filled => {}
        }
    }
    unreachable!()
}
