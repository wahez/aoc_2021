use itertools::Itertools;
use std::{collections::VecDeque, error::Error, io::BufRead};

struct RingBufferWithSort<T: Ord + Clone> {
    queue: VecDeque<T>,
    sorted: Vec<T>,
}

impl<T: Ord + Clone> RingBufferWithSort<T> {
    fn with_capacity(capacity: usize) -> RingBufferWithSort<T> {
        RingBufferWithSort {
            queue: VecDeque::with_capacity(capacity),
            sorted: Vec::with_capacity(capacity),
        }
    }
    fn len(&self) -> usize {
        debug_assert_eq!(self.queue.len(), self.sorted.len());
        self.queue.len()
    }
    fn push_back(&mut self, value: T) {
        self.queue.push_back(value.clone());
        let new_pos = self.sorted.binary_search(&value).unwrap_or_else(|e| e);
        self.sorted.insert(new_pos, value);
    }
    fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front().map(|old| {
            let old_pos = self.sorted.binary_search(&old).unwrap();
            self.sorted.remove(old_pos);
            old
        })
    }
}

impl<T: Ord + Clone + Eq> RingBufferWithSort<T> {
    fn has_duplicates(&self) -> bool {
        self.sorted.iter().tuple_windows().any(|(l, r)| l == r)
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    const NUMBYTES: usize = 4;
    let mut queue = RingBufferWithSort::with_capacity(NUMBYTES + 1);
    for (i, c) in buf.bytes().enumerate() {
        queue.push_back(c?);
        if queue.len() > NUMBYTES {
            queue.pop_front();
            if !queue.has_duplicates() {
                return Ok(i + 1);
            }
        }
    }
    Err("Start of packet not found".into())
}

pub fn b(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    const NUMBYTES: usize = 14;
    let mut queue = RingBufferWithSort::with_capacity(NUMBYTES + 1);
    for (i, c) in buf.bytes().enumerate() {
        queue.push_back(c?);
        if queue.len() > NUMBYTES {
            queue.pop_front();
            if !queue.has_duplicates() {
                return Ok(i + 1);
            }
        }
    }
    Err("Start of message not found".into())
}
