use std::{cmp::Reverse, collections::BinaryHeap, error::Error, io::BufRead};

use itertools::repeat_n;

use crate::{grid::Grid, parsing::FromBufRead};
type Pos = crate::pos::Pos<i16>;

const DIRECTIONS: [Pos; 4] = [
    Pos { x: 1, y: 0 },
    Pos { x: -1, y: 0 },
    Pos { x: 0, y: 1 },
    Pos { x: 0, y: -1 },
];

struct HeightGrid {
    heights: Grid<i16, i16>,
    start_pos: Pos,
    end_pos: Pos,
}

impl FromBufRead for HeightGrid {
    type Error = Box<dyn Error>;
    fn read(br: &mut impl BufRead) -> Result<Self, Self::Error> {
        let mut row_size = 0;
        let mut h = Vec::new();
        let mut start_pos = None;
        let mut end_pos = None;
        const UNREACHABLE: i16 = 32000;
        for (y, line) in br.lines().enumerate() {
            let line = line?;
            if row_size != 0 && row_size != line.len() {
                Err("Inconsistent row size")?;
            }
            row_size = line.len();
            h.push(UNREACHABLE);
            for (x, c) in line.as_bytes().iter().enumerate() {
                h.push(match c {
                    b'S' => {
                        start_pos = Some(Pos {
                            x: x as i16,
                            y: y as i16,
                        });
                        0
                    }
                    b'E' => {
                        end_pos = Some(Pos {
                            x: x as i16,
                            y: y as i16,
                        });
                        25
                    }
                    b => (b - b'a').into(),
                })
            }
            h.push(UNREACHABLE);
        }
        row_size += 2;
        h.splice(0..0, repeat_n(UNREACHABLE, row_size));
        h.extend(repeat_n(UNREACHABLE, row_size));
        let start_pos = &start_pos.ok_or("Did not find start pos in grid")? + &Pos { x: 1, y: 1 };
        let end_pos = &end_pos.ok_or("Did not find end pos in grid")? + &Pos { x: 1, y: 1 };
        let size = Pos {
            x: row_size as i16,
            y: (h.len() / row_size) as i16,
        };
        let heights = Grid::from_values(h, size);
        Ok(HeightGrid {
            heights,
            start_pos,
            end_pos,
        })
    }
}

struct Solver {
    grid: HeightGrid,
    solved: Grid<usize, i16>,
    to_solve: BinaryHeap<Reverse<(usize, Pos)>>,
}

impl Solver {
    fn new(grid: HeightGrid) -> Solver {
        let solved = Grid::from_value(32000, grid.heights.size());
        Solver {
            grid,
            solved,
            to_solve: Default::default(),
        }
    }
    fn solve_a(&mut self) -> Result<usize, Box<dyn Error>> {
        self.to_solve.clear();
        self.to_solve.push(Reverse((0, self.grid.start_pos)));
        while let Some(Reverse((rank, pos))) = self.to_solve.pop() {
            if rank < self.solved[&pos] {
                self.solved[&pos] = rank;
                for dir in DIRECTIONS {
                    let next_pos = &pos + &dir;
                    if self.grid.heights[&next_pos] > self.grid.heights[&pos] + 1 {
                        continue;
                    }
                    if next_pos == self.grid.end_pos {
                        return Ok(rank + 1);
                    }
                    if self.solved[&next_pos] <= rank {
                        continue;
                    }
                    self.to_solve.push(Reverse((rank + 1, next_pos)));
                }
            }
        }
        println!("{}", self.solved);
        Err("No solution found".into())
    }
    fn solve_b(&mut self) -> Result<usize, Box<dyn Error>> {
        self.to_solve.clear();
        self.to_solve.push(Reverse((0, self.grid.end_pos)));
        while let Some(Reverse((rank, pos))) = self.to_solve.pop() {
            if rank < self.solved[&pos] {
                self.solved[&pos] = rank;
                for dir in DIRECTIONS {
                    let next_pos = &pos + &dir;
                    if self.solved[&next_pos] <= rank {
                        continue;
                    }
                    let next_height = self.grid.heights[&next_pos];
                    if next_height == 32000 {
                        continue;
                    }
                    if self.grid.heights[&pos] > next_height + 1 {
                        continue;
                    }
                    if next_height == 0 {
                        return Ok(rank + 1);
                    }
                    self.to_solve.push(Reverse((rank + 1, next_pos)));
                }
            }
        }
        println!("{}", self.solved);
        Err("No solution found".into())
    }
}

pub fn a(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let grid = HeightGrid::read(&mut buf)?;
    let mut solver = Solver::new(grid);
    solver.solve_a()
}

pub fn b(mut buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let grid = HeightGrid::read(&mut buf)?;
    let mut solver = Solver::new(grid);
    solver.solve_b()
}
