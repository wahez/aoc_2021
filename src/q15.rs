use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, io::BufRead, ops::Range, str::FromStr};

use crate::{parsing::parse_by_line, pos::Pos, regex_parse};

struct Sensor {
    center: Pos<i32>,
    beacon: Pos<i32>,
    manhattan: i32,
}

impl Sensor {
    pub fn new(center: Pos<i32>, beacon: Pos<i32>) -> Sensor {
        let dist = &beacon - &center;
        let manhattan = dist.x.abs() + dist.y.abs();
        Sensor {
            center,
            beacon,
            manhattan,
        }
    }
    fn range_at(&self, y: i32) -> Option<Range<i32>> {
        let remaining = self.manhattan - (y - self.center.y).abs();
        if remaining <= 0 {
            None
        } else {
            Some(self.center.x - remaining..self.center.x + remaining + 1)
        }
    }
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
            )
            .unwrap();
        }
        match regex_parse!(REGEX, s, (i32, i32, i32, i32)) {
            None => Err("Sensor could not be parsed".into()),
            Some(Err(e)) => Err(e),
            Some(Ok((sx, sy, bx, by))) => {
                Ok(Sensor::new(Pos { x: sx, y: sy }, Pos { x: bx, y: by }))
            }
        }
    }
}

struct Ranges(Vec<Range<i32>>);

impl Ranges {
    fn new() -> Ranges {
        Ranges(Vec::new())
    }
    fn binary_search(&self, pos: i32) -> Result<usize, usize> {
        match self.0.binary_search_by(|r| r.start.cmp(&pos)) {
            Ok(index) => Ok(index),
            Err(index) => {
                if index > 0 && pos < self.0[index - 1].end {
                    Ok(index - 1)
                } else {
                    Err(index)
                }
            }
        }
    }
    fn reset_from_iter(&mut self, iter: impl Iterator<Item = Range<i32>>) {
        self.0.clear();
        self.0.extend(iter);
        self.0.sort_by_key(|r| r.start);
        let mut i = 0;
        for j in 1..self.0.len() {
            if self.0[i].end >= self.0[j].start {
                self.0[i].end = std::cmp::max(self.0[i].end, self.0[j].end);
            } else {
                i += 1;
                self.0[i] = self.0[j].clone();
            }
        }
        self.0.truncate(i + 1);
    }
    fn remove(&mut self, v: i32) {
        if let Ok(index) = self.binary_search(v) {
            match (v - self.0[index].start, self.0[index].end - v) {
                (0, 1) => {
                    self.0.remove(index);
                }
                (0, _) => self.0[index].start = v + 1,
                (_, 1) => self.0[index].end = v,
                (_, _) => {
                    self.0.insert(index + 1, v + 1..self.0[index].end);
                    self.0[index].end = v;
                }
            }
        }
    }
    fn count(&self) -> usize {
        self.0.iter().map(|r| r.len()).sum()
    }
}

fn range_and(lhs: &Range<i32>, rhs: &Range<i32>) -> Option<Range<i32>> {
    let start = std::cmp::max(lhs.start, rhs.start);
    let end = std::cmp::min(lhs.end, rhs.end);
    if start < end {
        Some(start..end)
    } else {
        None
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let sensors = parse_by_line::<Sensor>(buf).collect::<Result<Result<Vec<Sensor>, _>, _>>()??;
    let y = 2_000_000;
    let mut ranges = Ranges::new();
    ranges.reset_from_iter(sensors.iter().filter_map(|sensor| sensor.range_at(y)));
    let centers = sensors.iter().map(|sensor| sensor.center);
    let beacons = sensors.iter().map(|sensor| sensor.beacon);
    for object in centers.chain(beacons).filter(|o| o.y == y) {
        ranges.remove(object.x);
    }
    Ok(ranges.count())
}

pub fn b(buf: impl BufRead) -> Result<i64, Box<dyn Error>> {
    let mut sensors =
        parse_by_line::<Sensor>(buf).collect::<Result<Result<Vec<Sensor>, _>, _>>()??;
    sensors.sort_by_key(|s| s.center.x);
    let valid_range = 0..4_000_001;
    let mut occupied = Ranges::new();
    for y in valid_range.clone() {
        occupied.reset_from_iter(
            sensors
                .iter()
                .filter_map(|sensor| sensor.range_at(y).and_then(|r| range_and(&r, &valid_range))),
        );
        match valid_range.len() - occupied.count() {
            0 => {}
            1 => return Ok((occupied.0[0].end as i64) * 4_000_000 + y as i64),
            _ => {
                println!("{:?}", occupied.0);
                Err("Found row with multiple free spaces")?
            }
        }
    }
    Err("Did not find solution".into())
}
