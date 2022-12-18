use itertools::{Itertools, MinMaxResult};
use std::{
    collections::HashSet,
    error::Error,
    io::BufRead,
    ops::{Add, Range},
    str::FromStr,
};

use crate::parsing::FromBufRead;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pos(i16, i16, i16);

impl Pos {
    fn is_contained_by(&self, cube: &[Range<i16>; 3]) -> bool {
        cube[0].contains(&self.0) && cube[1].contains(&self.1) && cube[2].contains(&self.2)
    }
}

const NEIGHBOURS: [Pos; 6] = [
    Pos(0, 0, 1),
    Pos(0, 0, -1),
    Pos(0, 1, 0),
    Pos(0, -1, 0),
    Pos(1, 0, 0),
    Pos(-1, 0, 0),
];

impl Add<&Pos> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl FromStr for Pos {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(FromStr::from_str).tuples();
        let (x, y, z) = iter.next().ok_or("Did not get 3 elements")?;
        if iter.next().is_some() {
            Err("Too many elements")?;
        }
        Ok(Pos(x?, y?, z?))
    }
}

struct Droplet(HashSet<Pos>);

impl Droplet {
    fn get_bounding_cube(&self) -> [Range<i16>; 3] {
        let minmax = |mmresult| match mmresult {
            MinMaxResult::NoElements => Range { start: 0, end: 0 },
            MinMaxResult::OneElement(v) => Range {
                start: v - 1,
                end: v + 2,
            },
            MinMaxResult::MinMax(min, max) => Range {
                start: min - 1,
                end: max + 2,
            },
        };
        [
            minmax(self.0.iter().map(|p| p.0).minmax()),
            minmax(self.0.iter().map(|p| p.1).minmax()),
            minmax(self.0.iter().map(|p| p.2).minmax()),
        ]
    }
    fn count_surfaces(&self) -> usize {
        self.0
            .iter()
            .map(|d| {
                NEIGHBOURS
                    .iter()
                    .map(|n| d + n)
                    .filter(|n| !self.0.contains(n))
                    .count()
            })
            .sum()
    }
}

impl FromBufRead for Droplet {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        Ok(Droplet(
            br.lines()
                .map_ok(|l| Pos::from_str(&l))
                .collect::<Result<Result<HashSet<Pos>, _>, _>>()??,
        ))
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let droplet = Droplet::read(&mut buf)?;
    Ok(droplet.count_surfaces())
}

pub fn b(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let droplet = Droplet::read(&mut buf)?;
    let cube = droplet.get_bounding_cube();
    let mut water = Droplet(HashSet::new());
    water.0.extend([
        Pos(cube[0].start, cube[1].start, cube[2].start),
        Pos(cube[0].start, cube[1].start, cube[2].end - 1),
        Pos(cube[0].start, cube[1].end - 1, cube[2].start),
        Pos(cube[0].start, cube[1].end - 1, cube[2].end - 1),
        Pos(cube[0].end - 1, cube[1].start, cube[2].start),
        Pos(cube[0].end - 1, cube[1].start, cube[2].end - 1),
        Pos(cube[0].end - 1, cube[1].end - 1, cube[2].start),
        Pos(cube[0].end - 1, cube[1].end - 1, cube[2].end - 1),
    ]);
    let mut new_water = Vec::new();
    loop {
        new_water.extend(
            water
                .0
                .iter()
                .flat_map(|p| {
                    let p = p.clone();
                    NEIGHBOURS.iter().map(move |n| &p + n)
                })
                .filter(|n| {
                    !water.0.contains(n) && !droplet.0.contains(n) && n.is_contained_by(&cube)
                }), // TODO prevent always growing
        );
        if new_water.is_empty() {
            break;
        }
        water.0.extend(new_water.drain(..));
    }
    let outside_surface = 2
        * (cube[0].len() * cube[1].len()
            + cube[1].len() * cube[2].len()
            + cube[0].len() * cube[2].len());
    Ok(water.count_surfaces() - outside_surface)
}
