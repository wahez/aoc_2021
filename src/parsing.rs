use itertools::Itertools;
use std::io::BufRead;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait FromBufRead: Sized {
    type Error;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error>;

    fn read_iter<B: BufRead>(buf_read: &mut B) -> ParseIter<B, Self> {
        ParseIter {
            buf_read,
            _t: PhantomData,
        }
    }
}

pub struct ParseIter<'a, BR: 'a + BufRead, T: FromBufRead> {
    buf_read: &'a mut BR,
    _t: PhantomData<T>,
}

impl<'a, BR: 'a + BufRead, T: FromBufRead> Iterator for ParseIter<'a, BR, T> {
    type Item = Result<T, T::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.buf_read.fill_buf() {
            Err(_) => None,
            Ok(d) if d.is_empty() => None,
            Ok(_) => Some(T::read(self.buf_read)),
        }
    }
}

pub fn parse_by_line<T: FromStr>(
    buf: impl BufRead,
) -> impl Iterator<Item = Result<Result<T, T::Err>, std::io::Error>> {
    buf.lines().map_ok(|l| T::from_str(&l))
}

// this macro will only be able to parse a regex with a fixed number of (non-optional) groups.
// it doesn't work with &str and is inefficient with String
#[macro_export]
macro_rules! regex_parse {
    ($reg:ident, $text:ident, ($t0:ty,$($t:ty),*)) => {
        match $reg.captures($text) {
            None => None,
            Some(captures) => {
                // if any 'unwrap' panics, there is a logic error in the code or the regex
                let mut iter = captures.iter().skip(1).map(|m| m.unwrap().as_str());
                let mut wrap = || -> Result<($t0,$($t),*), Box<dyn Error>> {
                    let result: ($t0,$($t),*) = (iter.next().unwrap().parse::<$t0>()?,$((iter.next().unwrap().parse::<$t>()?)),*);
                    Ok(result)
                };
                Some(wrap())
            }
        }
    };
}
