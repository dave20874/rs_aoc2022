use crate::day::{Day, Answer};
use crate::astar::{AStarState, AStarSearch};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;

#[derive(Hash)]
struct ValveInfo {
    id: usize,
    name: String,
    flow_rate: usize,
    neighbors: Vec<usize>,
}

// The state of the whole puzzle
struct State<'a> {
    time: usize,                        // elapsed time from start
    position: usize,                    // which valve we are near
    last_position: usize,               // where we were last
    flowed: usize,                      // what will flow based on all open valves.
    valve_open: Vec<bool>,
    valve_info: &'a HashMap<usize, ValveInfo>,
    sequence: Vec<String>,
}

impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
         .field("time", &self.time)
         .field("position", &self.position)
         .field("flowed", &self.flowed)
         .field("valve_open", &self.valve_open)
         .finish()
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time &&
        self.position == other.position &&
        self.flowed == other.flowed &&
        self.valve_open == other.valve_open
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.position.hash(state);
        self.flowed.hash(state);
        self.valve_open.hash(state);
    }
}

pub struct Day16 {
    valve_ids: HashMap<String, usize>,
    valves: HashMap<usize, ValveInfo>,
}

// 
impl<'a> AStarState for State<'a> {
    fn is_final(&self) -> bool {
        self.time == 30
    }

    fn cost(&self) -> isize {
        // Return number of units flowed.
        self.flowed as isize
    }

    fn completion_estimate(&self) -> isize {
        // current value plus value if all remaining valves were opened in next time step.
        let mut unopened_rate = 0;

        let remaining_minutes = 
            if self.time >= 30 { 0 }
            else { 30 - self.time };
        for n in 0..self.valve_info.len() {
            if !self.valve_open[n] {
                // n is a valve that could be opened.
                unopened_rate += self.valve_info.get(&n).unwrap().flow_rate;
            }
        }
        let max_unrealized = remaining_minutes * unopened_rate;

        (max_unrealized) as isize
    }

    fn next_states(&self) -> Vec<Box<State<'a>>> {
        // Generate all possible next states from this state.
        // At each time step we can open the local valve, if it's closed or
        // we can move to any adjacent valve.  Either way, time will advance one step.
        // (Once time reaches the limit, no further states can be reached.)
        // If opening the local valve, our position stays the same and the local
        // valve is given the current time as its open
        let mut next_states: Vec<Box<State>> = Vec::new();

        // are we at a valve we can open?
        if !self.valve_open[self.position] {
            // Create the option where we open the valve
            let mut new_valve_open = self.valve_open.clone();
            new_valve_open[self.position] = true;
            let mut seq = self.sequence.clone();
            seq.push(format!("Open valve {}", self.valve_info[&self.position].name));
            let new_flow = (30 - self.time - 1) * self.valve_info[&self.position].flow_rate;
            let state = State {
                time: self.time+1, 
                flowed: self.flowed+new_flow,
                position: self.position, 
                last_position: self.position,
                valve_open: new_valve_open,
                valve_info: self.valve_info,
                sequence: seq,
            };
            next_states.push(Box::new(state));
        }

        // Try moving to a neighboring valve
        for neighbor in &self.valve_info[&self.position].neighbors {
            if *neighbor != self.last_position {
                let mut seq = self.sequence.clone();
                seq.push(format!("Move to {}", self.valve_info[neighbor].name));
                let state = State {
                    time: self.time+1,
                    position: *neighbor,
                    last_position: self.position,
                    flowed: self.flowed,
                    valve_open: self.valve_open.clone(),
                    valve_info: self.valve_info,
                    sequence: seq,
                };
                next_states.push(Box::new(state));
            }
        }

        next_states
    }
}

impl Day16 {
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
        let valve_info = ValveInfo { id: valve_id, name: name.to_string(), flow_rate, neighbors: neighbor_ids };

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
        let valve_open: Vec<bool> = vec![false; self.valve_ids.len()];
        let mut seq = Vec::new();
        seq.push(format!("Start at {}.", self.valves[&position_id].name));

        State {
            time: 0,
            flowed: 0,
            position: position_id,
            last_position: position_id,
            valve_open,
            valve_info: &self.valves,
            sequence: seq,
        }
    }
}

impl<'a> Day for Day16 {
    fn part1(&self) -> Answer {
        let initial = self.get_start();

        let mut searcher: AStarSearch<State> = AStarSearch::new(false);
        searcher.set_start(initial);

        let final_state = searcher.search().unwrap();

        Answer::Number(final_state.flowed)
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
        assert_eq!(initial.valve_open.len(), 10);
        for n in 0..initial.valve_open.len() {
            assert_eq!(initial.valve_open[n], false);
        }
    }

    #[test]
    fn test_value_functions() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        assert_eq!(initial.cost(), 0);
        assert_eq!(initial.completion_estimate(), 2349);
    }

    #[test]
    fn test_next_states() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        let nexts = initial.next_states();

        assert_eq!(nexts.len(), 4);

        // First one is where we open the valve
        assert_eq!(nexts[0].valve_open[0], true);
        assert_eq!(nexts[0].position, 0);

        // Others are all moves to a new location
        assert_eq!(nexts[1].valve_open[0], false);
        assert_eq!(nexts[1].position, *d.valve_ids.get("DD").unwrap());

        assert_eq!(nexts[2].valve_open[0], false);
        assert_eq!(nexts[2].position, *d.valve_ids.get("II").unwrap());

        assert_eq!(nexts[3].valve_open[0], false);
        assert_eq!(nexts[3].position, *d.valve_ids.get("BB").unwrap());
    }
    
    #[test]
    fn test_next_next_states() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        let nexts = initial.next_states();
        let nn = nexts[0].next_states();

        assert_eq!(nn.len(), 3);

        // All are all moves to a new location, with valve AA open
        assert_eq!(nn[0].time, 2);
        assert_eq!(nn[0].valve_open[0], true);
        assert_eq!(nn[0].position, *d.valve_ids.get("DD").unwrap());

        assert_eq!(nn[1].valve_open[0], true);
        assert_eq!(nn[1].position, *d.valve_ids.get("II").unwrap());

        assert_eq!(nn[2].valve_open[0], true);
        assert_eq!(nn[2].position, *d.valve_ids.get("BB").unwrap());
    }

    #[test]
    fn test_next_next_states2() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();
        let nexts = initial.next_states();
        let nn = nexts[1].next_states();  // next states from closed valve at DD.

        assert_eq!(nn.len(), 4);

        // First one is where we open the valve
        let valve_id_dd = *d.valve_ids.get("DD").unwrap();
        assert_eq!(nn[0].valve_open[valve_id_dd], true);
        assert_eq!(nn[0].position, valve_id_dd);

        // All are all moves to a new location, with valve DD cstill losed
        assert_eq!(nn[1].time, 2);
        assert_eq!(nn[1].valve_open[valve_id_dd], false);
        assert_eq!(nn[1].position, *d.valve_ids.get("CC").unwrap());

        assert_eq!(nn[2].valve_open[valve_id_dd], false);
        assert_eq!(nn[2].position, *d.valve_ids.get("AA").unwrap());

        assert_eq!(nn[3].valve_open[valve_id_dd], false);
        assert_eq!(nn[3].position, *d.valve_ids.get("EE").unwrap());
    }

    #[test]
    fn test_search() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start();

        let mut searcher: AStarSearch<State> = AStarSearch::new(false);
        searcher.set_start(initial);
        println!("Starting search.");

        let final_state = searcher.search().unwrap();
        println!("Search found {:?}", final_state);
        println!("Sequence: {:?}", final_state.sequence);

        // I think the connections data structure is wrong somehow.

        assert_eq!(final_state.flowed, 1651);
    }

    #[test]
    fn test_part1() {
        let d = Day16::load("examples/day16_example1.txt");

        assert_eq!(d.part1(), Answer::Number(1651));
    }


}
