use itertools::Itertools;
use std::{cmp::Ordering, error::Error, io::BufRead, iter::once, str::FromStr};

use crate::parsing::FromBufRead;

enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Int(l), Int(r)) => l.cmp(r),
            (List(l), List(r)) => Self::partial_cmp_iters(l.iter(), r.iter()),
            (Int(l), List(r)) => Self::partial_cmp_iters(once(&Int(*l)), r.iter()),
            (List(l), Int(r)) => Self::partial_cmp_iters(l.iter(), once(&Int(*r))),
        }
    }
}

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

impl PartialEq<Packet> for Packet {
    fn eq(&self, other: &Packet) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Packet {
    fn parse(mut s: &str) -> Result<(Packet, &str), Box<dyn Error>> {
        if s.is_empty() {
            Err("No data")?;
        }
        match s.as_bytes()[0] {
            b'[' => {
                let mut subs = Vec::new();
                loop {
                    s = &s[1..];
                    if s.as_bytes()[0] != b']' {
                        let (sub, remaining) = Packet::parse(s)?;
                        subs.push(sub);
                        s = remaining;
                    }
                    match s.as_bytes()[0] {
                        b']' => return Ok((Packet::List(subs), &s[1..])),
                        b',' => {}
                        _ => Err("Unexpected token")?,
                    }
                }
            }
            _ => {
                let position = s
                    .find(|c| (c == ',') || (c == ']'))
                    .ok_or("No delimiter found")?;
                Ok((Packet::Int(s[..position].parse()?), &s[position..]))
            }
        }
    }

    fn partial_cmp_iters<'a>(
        mut lhs: impl Iterator<Item = &'a Packet>,
        mut rhs: impl Iterator<Item = &'a Packet>,
    ) -> Ordering {
        loop {
            match (lhs.next(), rhs.next()) {
                (None, Some(_)) => return Ordering::Less,
                (None, None) => return Ordering::Equal,
                (Some(_), None) => return Ordering::Greater,
                (Some(l), Some(r)) => match l.cmp(r) {
                    Ordering::Equal => {}
                    o => return o,
                },
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (packet, remaining) = Packet::parse(s)?;
        if !remaining.is_empty() {
            Err(format!("Did not parse full input: >{remaining}<"))?;
        }
        Ok(packet)
    }
}

impl FromBufRead for (Packet, Packet) {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let mut line = String::new();
        br.read_line(&mut line)?;
        let lhs = line.strip_suffix('\n').unwrap_or(&line).parse()?;
        line.clear();
        br.read_line(&mut line)?;
        let rhs = line.strip_suffix('\n').unwrap_or(&line).parse()?;
        br.read_line(&mut line)?;
        Ok((lhs, rhs))
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    for i in <(Packet, Packet) as FromBufRead>::read_iter(&mut buf)
        .enumerate()
        .map(|(i, res)| res.map(|(lhs, rhs)| (i + 1, lhs < rhs)))
        .filter_ok(|(_index, smaller)| *smaller)
        .map_ok(|(index, _smaller)| index)
    {
        sum += i?;
    }
    Ok(sum)
}

pub fn b(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let packets = buf
        .lines()
        .filter_ok(|l| !l.is_empty())
        .map_ok(|l| l.parse())
        .try_collect::<Result<_, _>, Result<_, _>, _>();
    let mut packets: Vec<Packet> = packets??;
    packets.sort();
    let pos1 = packets
        .binary_search(&"[[2]]".parse()?)
        .err()
        .ok_or("Could not find 2")?
        + 1;
    let pos2 = packets
        .binary_search(&"[[6]]".parse()?)
        .err()
        .ok_or("Could not find 6")?
        + 2;
    Ok(pos1 * pos2)
}
