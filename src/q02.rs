use std::{error::Error, io::BufRead, str::FromStr};

use crate::parsing::parse_by_line;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(i32)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn score(&self) -> i32 {
        *self as i32
    }
}

#[derive(Copy, Clone)]
#[repr(i32)]
enum GameResult {
    Player1Won = 0,
    Draw = 3,
    Player2Won = 6,
}

impl GameResult {
    fn score(&self) -> i32 {
        *self as i32
    }
}

fn partial_parse(line: &str) -> Result<(RPS, &str), &'static str> {
    let (p1, p2) = line.split_once(' ').ok_or("no space in line")?;
    use RPS::*;
    let player1 = match p1 {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => return Err("Expected A, B or C"),
    };
    Ok((player1, p2))
}

struct Game {
    player1: RPS,
    player2: RPS,
}

impl Game {
    fn result(&self) -> GameResult {
        use GameResult::*;
        if self.player1 == self.player2 {
            Draw
        } else if (self.player1 as i32) % 3 == (self.player2 as i32) - 1 {
            Player2Won
        } else {
            Player1Won
        }
    }
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (player1, p2) = partial_parse(line)?;
        use RPS::*;
        let player2 = match p2 {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => return Err("Expected X, Y or Z"),
        };
        Ok(Game { player1, player2 })
    }
}

pub fn a(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;
    for game in parse_by_line::<Game>(buf) {
        let game = game??;
        total += game.result().score() + game.player2.score();
    }
    Ok(total)
}

struct ExpectedGame {
    player1: RPS,
    result: GameResult,
}

impl ExpectedGame {
    fn player2(&self) -> RPS {
        use GameResult::*;
        use RPS::*;
        match self.result {
            Draw => self.player1,
            Player1Won => match self.player1 {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Player2Won => match self.player1 {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
        }
    }
}

impl FromStr for ExpectedGame {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (player1, res) = partial_parse(line)?;
        use GameResult::*;
        let result = match res {
            "X" => Player1Won,
            "Y" => Draw,
            "Z" => Player2Won,
            _ => return Err("Expected X, Y or Z"),
        };
        Ok(ExpectedGame { player1, result })
    }
}

pub fn b(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;
    for game in parse_by_line::<ExpectedGame>(buf) {
        let game = game??;
        total += game.result.score() + game.player2().score();
    }
    Ok(total)
}
