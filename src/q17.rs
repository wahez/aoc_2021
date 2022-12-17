use std::{collections::VecDeque, error::Error, fmt::Display, io::BufRead};

use crate::parsing::FromBufRead;

#[derive(Clone, Copy)]
struct Row(u8);

#[derive(Clone)]
struct Grid {
    rows: VecDeque<Row>,
}

#[derive(Clone)]
struct Shape {
    // TODO make array to prevent allocs
    rows: Vec<Row>,
}

#[derive(Clone)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Clone)]
struct Chamber {
    grid: Grid,
    jets: Vec<JetDirection>,
    jet_index: usize,
    num_rocks: usize,
}

impl Row {
    fn try_shl(&self) -> Option<Row> {
        if 0b1000000 & self.0 == 0 {
            Some(Row(self.0 << 1))
        } else {
            None
        }
    }
    fn try_shr(&self) -> Option<Row> {
        if 0b0000001 & self.0 == 0 {
            Some(Row(self.0 >> 1))
        } else {
            None
        }
    }
}

impl Shape {
    fn get_all() -> Vec<Shape> {
        vec![
            Shape {
                rows: vec![Row(0b0011110)],
            },
            Shape {
                rows: vec![Row(0b0001000), Row(0b0011100), Row(0b0001000)],
            },
            Shape {
                rows: vec![Row(0b0011100), Row(0b0000100), Row(0b0000100)],
            },
            Shape {
                rows: vec![
                    Row(0b0010000),
                    Row(0b0010000),
                    Row(0b0010000),
                    Row(0b0010000),
                ],
            },
            Shape {
                rows: vec![Row(0b0011000), Row(0b0011000)],
            },
        ]
    }
    fn try_shl(&self) -> Shape {
        // TODO no alloc
        let shifted_rows = self.rows.iter().map(Row::try_shl).collect();
        match shifted_rows {
            None => self.clone(),
            Some(rows) => Shape { rows },
        }
    }
    fn try_shr(&self) -> Shape {
        // TODO no alloc
        let shifted_rows = self.rows.iter().map(Row::try_shr).collect();
        match shifted_rows {
            None => self.clone(),
            Some(rows) => Shape { rows },
        }
    }
}

impl FromBufRead for JetDirection {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 1];
        if br.read(&mut buf)? != 1 {
            Err("Could not read from BufRead")?;
        }
        match buf[0] {
            b'<' => Ok(JetDirection::Left),
            b'>' => Ok(JetDirection::Right),
            _ => Err("Unexpected JetDirection".into()),
        }
    }
}

impl Grid {
    fn new() -> Grid {
        Grid {
            rows: VecDeque::new(),
        }
    }
    fn height(&self) -> usize {
        self.rows.len()
    }
    fn would_fit(&self, shape: &Shape, y: i16) -> bool {
        for (i, row) in shape.rows.iter().enumerate() {
            let index = y - i as i16;
            if index >= 0 {
                let index = index as usize;
                if index >= self.rows.len() || row.0 & self.rows[index].0 != 0 {
                    return false;
                }
            }
        }
        true
    }
    fn add(&mut self, shape: &Shape, mut y: i16) {
        for (i, row) in shape.rows.iter().enumerate() {
            if y >= i as i16 {
                self.rows[y as usize - i].0 |= row.0;
            } else {
                self.rows.push_front(*row);
                y += 1;
            }
        }
    }
}

impl Chamber {
    fn new(jets: Vec<JetDirection>) -> Chamber {
        Chamber {
            grid: Grid::new(),
            jets,
            jet_index: 0,
            num_rocks: 0,
        }
    }
    fn add(&mut self, mut shape: Shape) {
        self.num_rocks += 1;
        let mut y = -5;
        loop {
            if self.grid.would_fit(&shape, y + 1) {
                y += 1;
                // let mut c = self.clone();
                // c.grid.add(&shape, y);
                // println!("Shifted down");
                // println!("{c}");
            } else {
                break;
            }
            let index = self.jet_index;
            self.jet_index = (self.jet_index + 1) % self.jets.len();
            let new_shape = match self.jets[index] {
                JetDirection::Left => {
                    // println!("Shifted L");
                    shape.try_shl()
                }
                JetDirection::Right => {
                    // println!("Shifted R");
                    shape.try_shr()
                }
            };
            if self.grid.would_fit(&new_shape, y) {
                shape = new_shape;
                // println!("Shifted LR");
                // let mut c = self.clone();
                // c.grid.add(&shape, y);
                // println!("{c}");
            }
        }
        self.grid.add(&shape, y);
        // println!("Done");
        // println!("{}", self);
    }
    fn height(&self) -> usize {
        self.grid.height()
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.rows.iter() {
            writeln!(f, "{:07b}", row.0)?;
        }
        Ok(())
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let jets = JetDirection::read_iter(&mut buf).collect::<Result<Vec<JetDirection>, _>>()?;
    let mut chamber = Chamber::new(jets);
    let shapes = Shape::get_all();
    let mut shape_iter = shapes.iter().cycle();
    for _ in 0..2022 {
        let shape = shape_iter.next().unwrap(); // cannot fail due to cycle
        chamber.add(shape.clone());
    }
    Ok(chamber.height())
}

pub fn b(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let num_rocks = 1000000000000usize;
    let jets = JetDirection::read_iter(&mut buf).collect::<Result<Vec<JetDirection>, _>>()?;
    let shapes = Shape::get_all();
    // println!("{}", jets.len());
    let repeat_cycle = 1740; //jets.len() * shapes.len() * 4;
    let repeat_initial = num_rocks % repeat_cycle;
    let mut chamber = Chamber::new(jets);
    let mut shape_iter = shapes.iter().cycle();
    for _ in 0..repeat_initial {
        let shape = shape_iter.next().unwrap(); // cannot fail due to cycle
        chamber.add(shape.clone());
    }
    let initial_height = chamber.height();
    for _ in 0..repeat_cycle {
        let shape = shape_iter.next().unwrap(); // cannot fail due to cycle
        chamber.add(shape.clone());
    }
    let cycle_height = chamber.height() - initial_height;
    Ok(initial_height + cycle_height * (num_rocks / repeat_cycle))
}
