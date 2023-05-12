use crate::day::{Day, Answer};
use crate::astar::{AStarState, AStarSearch};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};
use itertools::iproduct;

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
    ttg: usize,                        // elapsed time from start
    agents: usize,
    position1: usize,                    // which valve we are near
    position2: usize,
    last_position1: usize,               // where we were last
    last_position2: usize,
    flowed: usize,                      // what will flow based on all open valves.
    valve_open: Vec<bool>,
    valve_info: &'a HashMap<usize, ValveInfo>,
    sequence: Vec<String>,
}

#[derive(Hash)]
struct StateKey {
    ttg: usize,
    position1: usize,
    position2: usize,
    valve_open: Vec<bool>,
}

#[derive(Debug)]
enum Action {
    Nop,
    Open(usize),
    MoveTo(usize),
}

impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
         .field("ttg", &self.ttg)
         .field("agents", &self.agents)
         .field("position1", &self.position1)
         .field("position2", &self.position2)
         .field("flowed", &self.flowed)
         .field("valve_open", &self.valve_open)
         .field("sequence", &self.sequence)
         .finish()
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ttg == other.ttg &&
        self.position1 == other.position1 &&
        self.position2 == other.position2 &&
        self.last_position1 == other.last_position1 &&
        self.last_position2 == other.last_position2 &&
        self.flowed == other.flowed &&
        self.valve_open == other.valve_open
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ttg.hash(state);
        self.position1.hash(state); 
        self.position2.hash(state);
        self.flowed.hash(state);
        self.valve_open.hash(state);
    }
}

pub struct Day16 {
    valve_ids: HashMap<String, usize>,
    valves: HashMap<usize, ValveInfo>,
}

// 
impl<'a> AStarState<StateKey> for State<'a> {
    fn is_final(&self) -> bool {
        self.ttg == 0
    }

    fn cost(&self) -> isize {
        // Return number of units flowed.
        self.flowed as isize
    }

    fn get_key(&self) -> StateKey {
        StateKey { ttg: self.ttg, position1: self.position1, position2: self.position2, valve_open: self.valve_open }
    }

    fn get_key_metric(&self) -> usize {
        self.flowed
    }

    fn completion_estimate(&self) -> isize {
        if self.is_final() {
            // println!("Estimate for final state is 0.");
            return 0;
        }

        let mut flows = (0..self.valve_info.len()).map(|n| if !self.valve_open[n] { self.valve_info[&n].flow_rate} else { 0 }).collect::<Vec<usize>>();
        flows.sort();
        flows.reverse();
        // println!("    sorted flows: {:?}", flows);

        let mut potential_flow = 0;
        let mut index = 0;
        for t in 0..self.ttg {
            for n in 0..self.agents {
            // account for agent1
                if index < flows.len() && (t < self.ttg) {
                    potential_flow += (self.ttg-1-t) * flows[index];
                    index += 1;
                }
            }
        }

        // println!("Completion estimate: {}", potential_flow);
        potential_flow as isize
    }

    fn next_states(&self) -> Vec<Box<State<'a>>> {

        // This is going to take a bit of rework.
        // With N agents, I need to produce all possible actions for each agent then
        // cross those, then compute next states for each combination.

        // Generate all possible next states from this state.
        // At each time step we can open the local valve, if it's closed or
        // we can move to any adjacent valve.  Either way, time will advance one step.
        // (Once time reaches the limit, no further states can be reached.)
        // If opening the local valve, our position stays the same and the local
        // valve is given the current time as its open
        let mut next_states: Vec<Box<State>> = Vec::new();

        let mut moves1: Vec<Action> = Vec::new();
        // If agent1 can open the valve here, that's a possible move
        let here = self.position1;
        if !self.valve_open[here] && self.valve_info[&here].flow_rate > 0 {
            moves1.push(Action::Open(here));
        }

        // Moving to any neighbor except where we just came from is a possible move.
        for neighbor in self.valve_info[&here].neighbors.iter() {
            if *neighbor != self.last_position1 {
                moves1.push(Action::MoveTo(*neighbor));
            }
        }

        let mut moves2: Vec<Action> = Vec::new();
        if self.agents > 1 {
            // If agent2 can open the valve here, that's a possible move
            let here = self.position2;
            if !self.valve_open[here] && self.valve_info[&here].flow_rate > 0 {
                moves2.push(Action::Open(here));
            }

            // Moving to any neighbor except where we just came from is a possible move.
            for neighbor in self.valve_info[&here].neighbors.iter() {
                if *neighbor != self.last_position2 {
                    moves2.push(Action::MoveTo(*neighbor));
                }
            }
        }
        else {
            moves2.push(Action::Nop);
        }

        // For each combination of moves, get resulting next state.
        // println!("Moves: {:?}, {:?}", moves[0], moves[1]);

        for (action1, action2) in iproduct!(moves1.iter(), moves2.iter()) {
            // println!("    Combo: {:?} {:?}", action1, action2);

            // Set up baseline next state.
            let mut new_valve_open = self.valve_open.clone();
            let mut seq = self.sequence.clone();
            let mut new_flow = 0;
            let mut new_position1 = self.position1;
            let mut new_position2 = self.position2;

            // Change state according to action1.
            match action1 {
                Action::Nop => { /* do nothing */ }
                Action::Open(position) => {
                    seq.push(format!("1 Opens valve {}", self.valve_info[position].name));
                    if !new_valve_open[*position] {
                        new_valve_open[*position] = true;
                        new_flow += (self.ttg-1)*self.valve_info[position].flow_rate;
                    }
                }
                Action::MoveTo(position) => {
                    seq.push(format!("1 Moves to {}", self.valve_info[position].name));
                    new_position1 = *position;
                }
            }

            // Change state according to action2.
            match action2 {
                Action::Nop => { /* do nothing */ }
                Action::Open(position) => {
                    seq.push(format!("2 Opens valve {}", self.valve_info[position].name));
                    if !new_valve_open[*position] {
                        new_valve_open[*position] = true;
                        new_flow += (self.ttg-1)*self.valve_info[position].flow_rate;
                    }
                }
                Action::MoveTo(position) => {
                    seq.push(format!("2 Moves to {}", self.valve_info[position].name));
                    new_position2 = *position;
                }
            }

            let mut state = State {
                ttg: self.ttg-1, 
                agents: self.agents,
                flowed: self.flowed+new_flow,
                position1: new_position1,
                position2: new_position2,
                last_position1: self.position1,
                last_position2: self.position2,
                valve_open: new_valve_open,
                valve_info: self.valve_info,
                sequence: seq,
            };
            next_states.push(Box::new(state));
        };

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

    fn get_start_p1(&self) -> State {
        let position_id = *self.valve_ids.get("AA").unwrap();
        let valve_open: Vec<bool> = vec![false; self.valve_ids.len()];
        let mut seq = Vec::new();
        seq.push(format!("Start at {}.", self.valves[&position_id].name));

        State {
            ttg: 30,
            agents: 1,
            flowed: 0,
            position1: position_id,
            last_position1: position_id,
            position2: position_id,
            last_position2: position_id,
            valve_open,
            valve_info: &self.valves,
            sequence: seq,
        }
    }

    fn get_start_p2(&self) -> State {
        let position_id = *self.valve_ids.get("AA").unwrap();
        let valve_open: Vec<bool> = vec![false; self.valve_ids.len()];
        let mut seq = Vec::new();
        seq.push(format!("Start at {}/{}.", 
            self.valves[&position_id].name, 
            self.valves[&position_id].name));

        State {
            ttg: 26,
            agents: 2,
            flowed: 0,
            position1: position_id,
            last_position1: position_id,
            position2: position_id,
            last_position2: position_id,
            valve_open,
            valve_info: &self.valves,
            sequence: seq,
        }
    }
}

impl<'a> Day for Day16 {
    fn part1(&self) -> Answer {
        let initial = self.get_start_p1();

        let mut searcher: AStarSearch<State, StateKey> = AStarSearch::new(false, false);
        searcher.set_start(initial);

        let final_state = searcher.search().unwrap();

        Answer::Number(final_state.flowed)
    }

    fn part2(&self) -> Answer {
        let initial = self.get_start_p2();

        let mut searcher: AStarSearch<State, StateKey> = AStarSearch::new(false, true);
        searcher.set_start(initial);

        let final_state = searcher.search().unwrap();

        Answer::Number(final_state.flowed)
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
        let initial = d.get_start_p1();
        assert_eq!(initial.ttg, 30);
        assert_eq!(Some(&initial.position1), d.valve_ids.get("AA"));
        assert_eq!(initial.valve_open.len(), 10);
        for n in 0..initial.valve_open.len() {
            assert_eq!(initial.valve_open[n], false);
        }
    }

    #[test]
    fn test_value_functions() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start_p1();
        assert_eq!(initial.cost(), 0);
        assert_eq!(initial.completion_estimate(), 2227);
    }

    #[test]
    fn test_next_states() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start_p1();
        let nexts = initial.next_states();

        assert_eq!(nexts.len(), 3);

        // all first moves are to a new location
        assert_eq!(nexts[0].valve_open[0], false);
        assert_eq!(nexts[0].position1, *d.valve_ids.get("DD").unwrap());

        assert_eq!(nexts[1].valve_open[0], false);
        assert_eq!(nexts[1].position1, *d.valve_ids.get("II").unwrap());

        assert_eq!(nexts[2].valve_open[0], false);
        assert_eq!(nexts[2].position1, *d.valve_ids.get("BB").unwrap());
    }
    
    #[test]
    fn test_next_next_states() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start_p1();
        let nexts = initial.next_states();
        let nn = nexts[0].next_states();

        assert_eq!(nn.len(), 3);

        // First option is to open the valve
        assert_eq!(nn[0].ttg, 28);
        let pos_dd = *d.valve_ids.get("DD").unwrap();
        assert_eq!(nn[0].valve_open[pos_dd], true);
        assert_eq!(nn[0].position1, pos_dd);

        // All are all moves to a new location, with valve AA open
        assert_eq!(nn[1].ttg, 28);
        assert_eq!(nn[1].valve_open[0], false);
        assert_eq!(nn[1].position1, *d.valve_ids.get("CC").unwrap());

        assert_eq!(nn[2].ttg, 28);
        assert_eq!(nn[2].valve_open[0], false);
        assert_eq!(nn[2].position1, *d.valve_ids.get("EE").unwrap());
    }

    #[test]
    fn test_next_next_states2() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start_p1();
        let nexts = initial.next_states();
        let nn = nexts[1].next_states();
        let pos_ii = *d.valve_ids.get("II").unwrap();
        
        // next states from closed valve at II.

        assert_eq!(nn.len(), 1);

        // Only move from II is to JJ
        assert_eq!(nn[0].ttg, 28);
        assert_eq!(nn[0].valve_open[pos_ii], false);
        assert_eq!(nn[0].position1, *d.valve_ids.get("JJ").unwrap());
    }

    #[test]
    fn test_search() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start_p1();

        let mut searcher: AStarSearch<State, StateKey> = AStarSearch::new(false, false);
        searcher.set_start(initial);
        // println!("Starting search.");

        let final_state = searcher.search().unwrap();
        println!("Search found {:?}", final_state);
        println!("Sequence: {:?}", final_state.sequence);

        assert_eq!(final_state.flowed, 1651);
    }

    #[test]
    fn test_search2() {
        let d = Day16::load("examples/day16_example1.txt");
        let initial = d.get_start_p2();

        let mut searcher: AStarSearch<State, StateKey> = AStarSearch::new(false, false);
        searcher.set_start(initial);
        // println!("Starting search.");

        let final_state = searcher.search().unwrap();
        println!("Search found {:?}", final_state);
        println!("Sequence: {:?}", final_state.sequence);

        assert_eq!(final_state.flowed, 1707);
    }

    #[test]
    fn test_part1() {
        let d = Day16::load("examples/day16_example1.txt");

        assert_eq!(d.part1(), Answer::Number(1651));
    }


    #[test]
    fn test_part2() {
        let d = Day16::load("examples/day16_example1.txt");

        assert_eq!(d.part2(), Answer::Number(1707));
    }


}
