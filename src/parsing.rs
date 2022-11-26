use std::io::BufRead;
use std::marker::PhantomData;

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
