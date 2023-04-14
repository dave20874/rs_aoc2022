use crate::day::{Day, Answer};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use priority_queue::PriorityQueue;

/*
// The state of a valve in the optimizer
#[derive(Clone, Debug, PartialEq, Hash)]
enum ValveState {
    Closed,        // closed since beginning
    Open(usize),   // valve was opened at time given by usize
}
*/

struct ValveInfo {
    id: usize,
    flow_rate: usize,
    neighbors: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Hash)]
enum Action {
    Nop,            // do nothing
    Open,           // open the valve we are at
    Move(usize),    // move to specified valve
}

// The state of the whole puzzle
#[derive(Hash)]
struct State {
    time: usize,                        // elapsed time from start
    position: usize,                    // which valve we are near
    flowed: usize,                      // what's already flowed
    valve_open: Vec<bool>,
}

struct Path {
    step: Vec<Action>,
}

struct Optimizer<'a> {
    valve_info: &'a Vec<ValveInfo>,
    in_progress: PriorityQueue<Box<State>, usize>,
    best_value: HashMap<Box<State>, usize>,
}

pub struct Day16 {
    valve_ids: HashMap<String, usize>,
    valves: HashMap<usize, ValveInfo>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal  // TODO
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// This says PartialEq provides a total ordering.
impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 
impl State {
    fn value(&self) -> usize {
        // Return number of units flowed.
        self.flowed
    }

    fn potential_value(&self, valve_info: &HashMap<usize, ValveInfo>) -> usize {
        // TODO: current value plus value if all remaining valves were opened in next time step.
        let mut unopened_rate = 0;
        let remaining_minutes = 30 - self.time - 1;
        for n in 0..valve_info.len() {
            if !self.valve_open[n] {
                // n is a valve that could be opened.
                unopened_rate += valve_info.get(&n).unwrap().flow_rate;
            }
        }
        let max_unrealized = remaining_minutes * unopened_rate;

        max_unrealized + self.value()
    }

    fn next_states(&self, valve_info: &HashMap<usize, ValveInfo>) -> Vec<State> {
        // Generate all possible next states from this state.
        // At each time step we can open the local valve, if it's closed or
        // we can move to any adjacent valve.  Either way, time will advance one step.
        // (Once time reaches the limit, no further states can be reached.)
        // If opening the local valve, our position stays the same and the local
        // valve is given the current time as its open
        let mut next_states: Vec<State> = Vec::new();

        // are we at a valve we can open?
        match &self.valve_states[self.position] {
            ValveState::Closed => {
                // Create the option where we open the valve
                let mut new_valve_states = self.valve_states.clone();
                new_valve_states[self.position] = ValveState::Open(self.time);
                next_states.push(State {
                    time: self.time+1, 
                    position: self.position, 
                    valve_states: new_valve_states
                });
            }
            _ => ()
        }

        // Try moving to a neighboring valve
        for neighbor in &valve_info[&self.position].neighbors {
            next_states.push(State {
                time: self.time+1,
                position: *neighbor,
                valve_states: self.valve_states.clone(),
            });
        }

        next_states
    }
}

impl<'a> Optimizer<'a> {
    pub fn new(valves: &'a HashMap<usize, ValveInfo>, initial: &'a State) -> Optimizer<'a> {
        // 
        let mut in_progress: PriorityQueue<&'a State, usize> = PriorityQueue::new();
        let priority = initial.potential_value(valves);

        in_progress.push(initial, priority);

        Optimizer {valve_info: valves, in_progress: in_progress}
    }

    pub fn optimize(&'a mut self) -> usize {
        let mut max_value = 0;

        // while in_progress is not empty
        while self.in_progress.len() > 0 {
            // remove highest priority solution in progress
            let (state, _priority) = self.in_progress.pop().unwrap();

            // if it's current value is greater than max, increase max
            let value = state.value();
            max_value = max(max_value, value);

            // confirm that this is still the best way we've seen to get to this state.
            if self.best_value.get(state) > value {
                continue;
            }

            // For each next state from this one, evaluate it and add it back
            // to the priority queue if it's worth exploring.
            for next in state.next_states(&self.valve_info) {

            }
            // generate all its next states
            // for each next state
                // if this state isn't already bested by some other state
                // (this state is superior if at this position, time, it has either
                // a higher current value or a higher potential value)
                    // evaluate priority of this state
                    // insert this state into in_progress queue
        }

        max_value
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

    fn get_start(&self) -> State {
        let position_id = *self.valve_ids.get("AA").unwrap();
        let mut valve_states: Vec<ValveState> = Vec::new();
        for n in 0..self.valve_ids.len() {
            valve_states.push(ValveState::Closed);
        }

        State {
            time: 0,
            position: position_id,
            valve_states,
        }
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

    #[test]
    fn test_get_start() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        assert_eq!(initial.time, 0);
        assert_eq!(Some(&initial.position), d.valve_ids.get("AA"));
        assert_eq!(initial.valve_states.len(), 10);
        for n in 0..initial.valve_states.len() {
            assert_eq!(initial.valve_states[n], ValveState::Closed);
        }
    }

    #[test]
    fn test_value_functions() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        assert_eq!(initial.value(&d.valves), 0);
        assert_eq!(initial.potential_value(&d.valves), 2349);
    }

    #[test]
    fn test_next_states() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        let nexts = initial.next_states(&d.valves);

        assert_eq!(nexts.len(), 4);

        // First one is where we open the valve
        assert_eq!(nexts[0].valve_states[0], ValveState::Open(0));
        assert_eq!(nexts[0].position, 0);

        // Others are all moves to a new location
        assert_eq!(nexts[1].valve_states[0], ValveState::Closed);
        assert_eq!(nexts[1].position, *d.valve_ids.get("DD").unwrap());

        assert_eq!(nexts[2].valve_states[0], ValveState::Closed);
        assert_eq!(nexts[2].position, *d.valve_ids.get("II").unwrap());

        assert_eq!(nexts[3].valve_states[0], ValveState::Closed);
        assert_eq!(nexts[3].position, *d.valve_ids.get("BB").unwrap());
    }
    
    #[test]
    fn test_next_next_states() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        let nexts = initial.next_states(&d.valves);
        let nn = nexts[0].next_states(&d.valves);

        assert_eq!(nn.len(), 3);

        // All are all moves to a new location, with valve AA open
        assert_eq!(nn[0].time, 2);
        assert_eq!(nn[0].valve_states[0], ValveState::Open(0));
        assert_eq!(nn[0].position, *d.valve_ids.get("DD").unwrap());

        assert_eq!(nn[1].valve_states[0], ValveState::Open(0));
        assert_eq!(nn[1].position, *d.valve_ids.get("II").unwrap());

        assert_eq!(nn[2].valve_states[0], ValveState::Open(0));
        assert_eq!(nn[2].position, *d.valve_ids.get("BB").unwrap());
    }

    #[test]
    fn test_optimizer() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        let mut optimizer: Optimizer = Optimizer::new(&d.valves, &initial);

        let result = optimizer.optimize();
    }
}
