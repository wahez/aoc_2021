use std::{collections::HashMap, error::Error, io::BufRead};

struct Directory {
    subs: HashMap<String, Directory>,
    recursive_size: usize,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            subs: Default::default(),
            recursive_size: 0,
        }
    }
    fn add_file(&mut self, path: &[String], size: usize) -> Result<(), &'static str> {
        self.recursive_size += size;
        if path.is_empty() {
            Ok(())
        } else {
            // TODO remove clone
            self.subs
                .entry(path[0].clone())
                .or_insert_with(Directory::new)
                .add_file(&path[1..], size)
        }
    }
    fn dir_sizes(&self) -> Vec<usize> {
        // TODO allocates
        let mut sizes: Vec<usize> = self.subs.values().flat_map(|d| d.dir_sizes()).collect();
        sizes.push(self.recursive_size);
        sizes
    }
}

fn build_tree(buf: impl BufRead) -> Result<Directory, Box<dyn Error>> {
    // TODO lookup by full path is slow
    let mut root = Directory::new();
    let mut path = Vec::new();
    for line in buf.lines() {
        let line = line?;
        if line == "$ cd /" {
            path.clear();
        } else if line == "$ cd .." {
            path.pop().ok_or("Cannot go up from root")?;
        } else if line == "$ ls" {
            // ignore
        } else if let Some(l) = line.strip_prefix("$ cd ") {
            path.push(l.to_string());
        } else if let Some((size, _name)) = line.split_once(' ') {
            if size != "dir" {
                root.add_file(&path, size.parse()?)?;
            }
        } else {
            return Err(format!("Could not parse line {line}").into());
        }
    }
    Ok(root)
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let root = build_tree(buf)?;
    Ok(root.dir_sizes().iter().filter(|s| **s < 100000).sum())
}

pub fn b(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let root = build_tree(buf)?;
    let to_remove = root.recursive_size - 40000000;
    root.dir_sizes()
        .iter()
        .filter(|s| **s > to_remove)
        .copied()
        .min()
        .ok_or_else(|| "No directory found".into())
}
