use crate::day::{Day, Answer};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use priority_queue::PriorityQueue;


// The state of a valve in the optimizer
#[derive(Clone)]
enum ValveState {
    Closed,        // closed since beginning
    Open(usize),   // valve was opened at time given by usize
}

#[derive(Clone)]
struct ValveStates {
    states: Vec<ValveState>,
}

// The state of the whole puzzle
struct Day16State {
    time: usize,                        // elapsed time from start
    position: usize,                    // which valve we are near
    valve_states: ValveStates,          // valve_state[valve_id] -> ValveState
}
struct Optimizer<'a> {
    valve_info: &'a ValveInfo,
    in_progress: PriorityQueue<usize, Box<Day16State>>,
}

struct ValveInfo {
    id: usize,
    flow_rate: usize,
    neighbors: Vec<usize>,
}

pub struct Day16 {
    valve_ids: HashMap<String, usize>,
    valves: HashMap<usize, ValveInfo>,
}

impl Ord for Day16State {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal  // TODO
    }
}

impl PartialEq for Day16State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// This says PartialEq provides a total ordering.
impl Eq for Day16State {}

impl PartialOrd for Day16State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// AStar trait for Day16State
impl Day16State {
    fn value(&self) -> usize {
        // TODO: compute realized value for this state
        0
    }

    fn potential_value(&self) -> usize {
        // TODO: current value plus value if all remaining valves were opened in next time step.
        0
    }

    fn next_states(&self, valve_info: &Vec<ValveInfo>) -> Vec<Day16State> {
        // Generate all possible next states from this state.
        // At each time step we can open the local valve, if it's closed or
        // we can move to any adjacent valve.  Either way, time will advance one step.
        // (Once time reaches the limit, no further states can be reached.)
        // If opening the local valve, our position stays the same and the local
        // valve is given the current time as its open
        let mut next_states: Vec<Day16State> = Vec::new();

        // are we at a valve we can open?
        match &self.valve_states.states[self.position] {
            ValveState::Closed => {
                // Create the option where we open the valve
                let mut new_valve_states = self.valve_states.clone();
                new_valve_states.states[self.position] = ValveState::Open(self.time);
                next_states.push(Day16State {
                    time: self.time-1, 
                    position: self.position, 
                    valve_states: new_valve_states
                });
            }
            _ => ()
        }

        // Try moving to a neighboring valve
        for neighbor in &valve_info[self.position].neighbors {
            next_states.push(Day16State {
                time: self.time-1,
                position: *neighbor,
                valve_states: self.valve_states.clone(),
            });
        }

        next_states
    }
}


impl  Day16 {
    fn get_id(&mut self, name: &str) -> usize {
        if self.valve_ids.contains_key(name) {
            *self.valve_ids.get(name).unwrap()
        }
        else {
            let new_id = self.valve_ids.len();
            self.valve_ids.insert(name.to_string(), new_id);
            new_id
        }
    }

    fn add_valve(&mut self, name: &str, flow_rate: usize, neighbors: Vec<&str>) {
        let valve_id = self.get_id(name);

        let mut neighbor_ids: Vec<usize> = Vec::new();
        for n in neighbors {
            neighbor_ids.push(self.get_id(n));
        }
        let valve_info = ValveInfo { id: valve_id, flow_rate, neighbors: neighbor_ids };

        self.valves.insert(valve_id, valve_info);
    }

    pub fn load(filename: &str) -> Day16 {
        let mut d = Day16 { valve_ids: HashMap::new(), valves: HashMap::new() };
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let valve_name = &caps[1];
                    let flow_rate = caps[2].parse::<usize>().unwrap();
                    let neighbors = caps[3].split(", ").collect();

                    d.add_valve(valve_name, flow_rate, neighbors);
                }
                None => {}
            }
        }

        d
    }
}

impl<'a> Day for Day16 {
    fn part1(&self) -> Answer {
        Answer::Number(1)
    }

    fn part2(&self) -> Answer {
        Answer::Number(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day16::load("examples/day16_example1.txt");
        assert_eq!(d.valves.len(), 10);
    }
}
