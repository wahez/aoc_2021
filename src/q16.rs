use std::{cmp::Reverse, error::Error, io::BufRead, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    optimize::{optimize, Optimize},
    parsing::parse_by_line,
};

#[derive(Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    destination_names: Vec<String>,
}

impl FromStr for Valve {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z][A-Z].*)$").unwrap();
        }
        match REGEX.captures(s) {
            None => Err("line did not match pattern")?,
            Some(captures) => {
                let name = captures.get(1).unwrap().as_str().to_string();
                let flow_rate = captures.get(2).unwrap().as_str().parse()?;
                let destinations = captures.get(3).unwrap().as_str();
                let destination_names = destinations.split(", ").map(|d| d.to_string()).collect();
                Ok(Valve {
                    name,
                    flow_rate,
                    destination_names,
                })
            }
        }
    }
}

#[derive(Clone)]
struct Runner {
    time_left: i32,
    position: usize,
}

#[derive(Clone)]
struct State {
    runners: [Runner; 2],
    projected_release: i32,
    unopened_valves: Vec<usize>,
}

struct Problem {
    valves: Vec<Valve>,
    shortest_paths: Vec<Vec<i32>>,
}

impl Optimize for Problem {
    type State = State;
    type StateValue = i32;
    type Solutions = ();

    fn guaranteed(&self, state: &Self::State) -> Self::StateValue {
        state.projected_release
    }
    fn potential(&self, state: &Self::State) -> Self::StateValue {
        let mut potential_release = state.projected_release;
        // calculate upper bound for potential release by assuming that every valve can be reached in 1 step
        // this assumes that they are sorted in reverse already
        let mut time_left = [state.runners[0].time_left, state.runners[1].time_left];
        for v in state.unopened_valves.iter() {
            let tl = if time_left[0] > time_left[1] {
                time_left[0] -= 1;
                time_left[0]
            } else {
                time_left[1] -= 1;
                time_left[1]
            };
            match tl {
                -1 => break,
                0 => {}
                t => potential_release += t * self.valves[*v].flow_rate,
            }
        }
        potential_release
    }
    fn next_states(&self, state: &State, next: &mut Vec<State>) {
        next.extend(
            state
                .unopened_valves
                .iter()
                .map(|v| {
                    let mut runners = state.runners.clone();
                    let runner = runners.iter_mut().max_by_key(|r| r.time_left).unwrap(); // unwrap will not fail, there are two
                    let duration = self.shortest_paths[runner.position][*v];
                    runner.time_left = runner.time_left - duration - 1;
                    runner.position = *v;
                    let projected_release =
                        state.projected_release + runner.time_left * self.valves[*v].flow_rate;
                    let mut unopened_valves = state.unopened_valves.clone();
                    unopened_valves.remove(unopened_valves.iter().position(|u| u == v).unwrap()); // unwrap cannot fail
                    State {
                        runners,
                        projected_release,
                        unopened_valves,
                    }
                })
                .filter(|s| s.runners.iter().all(|r| r.time_left >= 0)),
        );
    }
}

fn solve_for_most_pressure(valves: Vec<Valve>, time_left: [i32; 2]) -> Result<i32, Box<dyn Error>> {
    const UNREACHABLE: i32 = 1000000;
    let mut shortest_paths = vec![vec![UNREACHABLE; valves.len()]; valves.len()];
    for (number, valve) in valves.iter().enumerate() {
        for destination in valve.destination_names.iter() {
            let dest = valves.iter().position(|v| v.name == *destination).unwrap(); // unwrap cannot fail
            shortest_paths[number][dest] = 1;
        }
    }
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for v1 in 0..valves.len() {
            for v2 in 0..valves.len() {
                for v3 in 0..valves.len() {
                    let via_dist = shortest_paths[v1][v2] + shortest_paths[v2][v3];
                    if via_dist < shortest_paths[v1][v3] {
                        shortest_paths[v1][v3] = via_dist;
                        has_changed = true;
                    }
                }
            }
        }
    }
    let mut unopened_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_i, v)| v.flow_rate > 0)
        .map(|(i, _v)| i)
        .collect();
    unopened_valves.sort_by_key(|v| Reverse(valves[*v].flow_rate));
    let problem = Problem {
        valves,
        shortest_paths,
    };
    let initial = State {
        runners: [
            Runner {
                time_left: time_left[0],
                position: 0,
            },
            Runner {
                time_left: time_left[1],
                position: 0,
            },
        ],
        projected_release: 0,
        unopened_valves,
    };
    Ok(optimize(problem, initial).projected_release)
}

pub fn a(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut valves = parse_by_line::<Valve>(buf).collect::<Result<Result<Vec<_>, _>, _>>()??;
    valves.sort_by_key(|v| v.name.clone());
    solve_for_most_pressure(valves, [30, 0])
}

pub fn b(buf: impl BufRead) -> Result<i32, Box<dyn Error>> {
    let mut valves = parse_by_line::<Valve>(buf).collect::<Result<Result<Vec<_>, _>, _>>()??;
    valves.sort_by_key(|v| v.name.clone());
    solve_for_most_pressure(valves, [26, 26])
}
