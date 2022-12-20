use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io::BufRead,
    mem::take,
    str::FromStr,
};

use crate::{
    optimize::{optimize, Optimize},
    parsing::parse_by_line,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct BotOutput {
    num_bots: usize,
    output: usize,
}

#[derive(Clone, Debug)]
struct State {
    bot_outputs: [BotOutput; 4],
    building: Option<Material>,
    time_left: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(usize)]
enum Material {
    Ore = 0,
    Clay,
    Obisidian,
    Geode,
}

#[derive(Debug)]
struct BotCost {
    output: Material,
    cost: Vec<(Material, usize)>,
}

struct BluePrint {
    id: usize,
    rules: [BotCost; 4],
    max_num_bots: [usize; 4],
}

impl FromStr for Material {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let material = match s {
            "ore" => Material::Ore,
            "clay" => Material::Clay,
            "obsidian" => Material::Obisidian,
            "geode" => Material::Geode,
            _ => Err("unknown material")?,
        };
        Ok(material)
    }
}

impl FromStr for BotCost {
    type Err = Box<dyn Error>;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix(" Each ").ok_or("Expected Each")?;
        let (material, mut s) = s.split_once(' ').ok_or("Expected material")?;
        s = s.strip_prefix("robot costs ").ok_or("Expected robot")?;
        let cost = s
            .split(" and ")
            .map(|num_mat| -> Result<(Material, usize), Box<dyn Error>> {
                let (num, mat) = num_mat.split_once(' ').ok_or("Expected space")?;
                Ok((mat.parse::<Material>()?, num.parse::<usize>()?))
            })
            .try_collect()?;
        Ok(BotCost {
            output: material.parse()?,
            cost,
        })
    }
}

impl FromStr for BluePrint {
    type Err = Box<dyn Error>;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix("Blueprint ").ok_or("Parse error")?;
        let (id, s) = s.split_once(':').ok_or("Expected :")?;
        let mut rules = s
            .split('.')
            .filter(|p| !p.is_empty())
            .map(|p| p.parse())
            .collect::<Result<Vec<BotCost>, _>>()?;
        rules.sort_by_key(|r| r.output);
        let mut max_num_bots = [0; 4];
        // println!("{rules:?}");
        for (material, max_num) in max_num_bots.iter_mut().enumerate() {
            *max_num = rules
                .iter()
                .flat_map(|bc| {
                    bc.cost.iter().filter_map(|c| {
                        if c.0 as usize == material {
                            Some(c.1)
                        } else {
                            None
                        }
                    })
                })
                .max()
                .unwrap_or(0);
        }
        max_num_bots[Material::Geode as usize] = 8_000_000_000;
        // println!("{max_num_bots:?}");
        Ok(BluePrint {
            id: id.parse()?,
            rules: rules.try_into().map_err(|_| "Expected 4 rules")?,
            max_num_bots,
        })
    }
}

impl BotOutput {
    fn project(&self, time_left: usize) -> usize {
        self.output + self.num_bots * time_left
    }
    fn evolve(&mut self) {
        self.output += self.num_bots;
    }
}

impl Optimize for BluePrint {
    type State = State;
    type StateValue = usize;
    type Solutions = HashMap<([usize; 4], Option<Material>), Vec<([usize; 4], usize)>>;

    fn guaranteed(&self, state: &Self::State) -> Self::StateValue {
        state.bot_outputs[Material::Geode as usize].project(state.time_left)
    }

    fn potential(&self, state: &Self::State) -> Self::StateValue {
        let mut output = state.bot_outputs[Material::Geode as usize].clone();
        output.num_bots += state.time_left - 1;
        output.project(state.time_left)
    }

    fn next_states(&self, state: &Self::State, next_states: &mut Vec<Self::State>) {
        if let Some(state) = state.evolved() {
            for bot_cost in self.rules.iter() {
                let m = bot_cost.output as usize;
                if state.bot_outputs[m].num_bots < self.max_num_bots[m] {
                    if let Some(next) = state.try_build(bot_cost) {
                        next_states.push(next);
                    }
                }
            }
            if next_states.len() < self.rules.len() {
                next_states.push(state);
            }
        }
    }

    fn add_if_improvement(&self, solutions: &mut Self::Solutions, state: &Self::State) -> bool {
        // TODO clean up
        let mut key: ([usize; 4], Option<Material>) = ([0; 4], state.building);
        let mut value: ([usize; 4], usize) = ([0; 4], state.time_left);
        for (i, bot) in state.bot_outputs.iter().enumerate() {
            key.0[i] = bot.num_bots;
            value.0[i] = bot.output;
        }
        let is_l_strictly_better_than_r = |l: &([usize; 4], usize), r: &([usize; 4], usize)| {
            r.1 <= l.1 && r.0.iter().zip(l.0.iter()).all(|(v, o)| o >= v)
        };
        match solutions.entry(key) {
            Entry::Vacant(v) => {
                v.insert(vec![value]);
                true
            }
            Entry::Occupied(mut o) => {
                if o.get()
                    .iter()
                    .any(|old| is_l_strictly_better_than_r(old, &value))
                {
                    false
                } else {
                    o.get_mut()
                        .retain(|e| !is_l_strictly_better_than_r(&value, e));
                    o.get_mut().push(value);
                    true
                }
            }
        }
    }
}

impl State {
    fn new(time_left: usize) -> State {
        State {
            bot_outputs: [
                BotOutput {
                    num_bots: 1,
                    output: 0,
                },
                BotOutput {
                    num_bots: 0,
                    output: 0,
                },
                BotOutput {
                    num_bots: 0,
                    output: 0,
                },
                BotOutput {
                    num_bots: 0,
                    output: 0,
                },
            ],
            time_left,
            building: None,
        }
    }
    fn evolved(&self) -> Option<State> {
        let mut state = self.clone();
        for bot in state.bot_outputs.iter_mut() {
            bot.evolve();
        }
        if let Some(bot) = take(&mut state.building) {
            state.bot_outputs[bot as usize].num_bots += 1;
        }
        if state.time_left == 0 {
            None
        } else {
            state.time_left -= 1;
            Some(state)
        }
    }
    fn try_build(&self, cost: &BotCost) -> Option<State> {
        let mut new_state = self.clone();
        for (material, amount) in cost.cost.iter() {
            if new_state.bot_outputs[*material as usize].output < *amount {
                return None;
            } else {
                new_state.bot_outputs[*material as usize].output -= *amount
            }
        }
        new_state.building = Some(cost.output);
        Some(new_state)
    }
}

impl BluePrint {
    fn calc_max_geodes(&self, time: usize) -> usize {
        let initial = State::new(time);
        let solution = optimize(self, initial);
        solution.bot_outputs[Material::Geode as usize].project(solution.time_left)
    }
}

pub fn a(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let sum: Result<Result<usize, _>, _> = parse_by_line::<BluePrint>(buf)
        .map_ok(|blueprint| blueprint.map(|bp| bp.id * bp.calc_max_geodes(24)))
        .sum();
    sum?
}

pub fn b(buf: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let product: Result<Result<usize, _>, _> = parse_by_line::<BluePrint>(buf)
        .take(3)
        .map_ok(|blueprint| blueprint.map(|bp| bp.calc_max_geodes(32)))
        .product();
    product?
}
