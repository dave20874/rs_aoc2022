use crate::day::{Day, Answer};
use std::collections::{HashMap};
use std::sync::Arc;
use priority_queue::PriorityQueue;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};
use itertools::iproduct;

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;

// ValveInfo represents the information read from the daily input file.
#[derive(Hash)]
struct ValveInfo {
    id: usize,
    name: String,
    flow_rate: usize,
    neighbors: Vec<usize>,
}

// Problem represents the overall problem to be solved.  It consists of the ValveInfo above plus time and num agents.
struct Problem<'a> {
    period: usize,  // how long we can take
    num_agents: usize, // how many agents can act
    valves: &'a HashMap<usize, ValveInfo>,  // what valves exist and how they are connected.
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
// This enum represents the action an agent can take in a given time step.
enum Action {
    Nop,
    Open(usize),
    MoveTo(usize),
}

// This represents a solution in-progress.  It stores the actions taken at each time step by each agent.
// Time elapsed is implied by the length of the action[N] vector.
#[derive(Hash, PartialEq, Eq, Clone)]
struct Solution {
    action: Vec<Vec<Action>>, // action[N][t] is the action taken by agent N at time t
}


// The state of the problem at a given point in time.
#[derive(Hash)]
#[derive(Eq, PartialEq)]
struct State {
    position: Vec<usize>,              // The position of each agent
    valve_state: Vec<bool>,            // whether each valve is open (true) or closed
}

// Note: The Solver will keep a best potential score for each state observed.  
// If a state is re-visited with a lower potential score, that move will be pruned.

// Solutions will be explored by the Solver by the following process:
//   Take the next solution-in-progress from the priority queue.
//   If the solution-in-progress is complete, 
//     declare it as the optimal solution.
//   Else
//     For each potential next steps.
//       Generate state and potential score for each next step.
//       If the generated state is already recorded with a higher potential score, discard it.
//       Store the extended extended solution using the potential score as its priority.
//     

struct Solver<'a> {
    problem: &'a Problem<'a>,
}

impl<'a> Problem<'a> {
    // Get the "empty" solution, representing the start state of the puzzle.
    pub fn get_start(&self) -> Solution {
        let mut action = Vec::new();
        for agent_id in 0..self.num_agents {
            action.push(Vec::new());
        }
        Solution { action }
    }
}

impl Solution {
    pub fn get_state_score(&self, problem: &Problem) -> (State, isize) {
        // start with all valves closed, all agents at start position
        let mut valve_state = vec!(false, problem.valves.len());  
        let mut position = vec!(START_POS, problem.num_agents);
        let mut ttg = problem.period;
        let mut flow_released = 0;

        // Perform all the operations in the solution to update valves, positions, ttg
        for t in 0..self.action[0].len() {
            for agent in 0..problem.num_agents {
                match self.action[agent][t] {
                    Action::Nop => {
                        // Do nothing
                    }
                    Action::MoveTo(pos) => {
                        // Move agent to new location
                        position[agent] = pos;
                    }
                    Action::Open(valve) => {
                        // Open a valve, capturing it's flow to the end
                        valve_state[valve] = true;
                        flow_released += problem.valves[valve].flow * (ttg - 1)
                    }
                }
            }

            // Update time to go
            ttg -= 1;
        }

        // Evaluate score based on remaining closed valves.
        let mut closed_flows = Vec::new();
        for valve in 0..problem.valves.len() {
            if !valve_state[valve] {
                closed_flows.append(problem.valves[valve].flow);
            }
        }
        closed_flows.sort().reverse();

        // closed_flows is a sorted vec of valves that could be opened.
        // Now assume, optimistically, that each agent will open the biggest valves
        // on each of their subsequent turns.  This overestimates the potential flow,
        // of course, but that's what we need to keep the algorithm stable.
        let mut index = 0;
        let mut potential_flow = 0;
        while ttg > 0 {
            for agent in 0..problem.num_agents {
                if index < closed_flows.len() {
                    potential_flow = closed_flows[index] * (ttg-1);
                    index += 1;
                }
            }
            ttg -= 1;
        }

        let state = State {valve_state, position };

        ( state, flow_released+potential_flow )
    }

    pub fn is_complete(&self, problem: &Problem) -> bool {
        self.action.len() == problem.period
    }

    pub fn get_next_steps(&self, problem: &Problem) -> Vec<Solution> {
        let mut nexts = Vec::new();

        let mut options: Vec<Vec<Action>> = Vec::new();
        for agent in 0..problem.num_agents {
            let mut agent_options = Vec::new();
            // Can this agent open a valve?

            options[agent] = agent_options;
        }

        // TODO

        nexts
    }
}

impl<'a> Solver<'a> {
    pub fn new(problem: &'a Problem) -> Solver<'a> {
        Solver { problem }
    }

    pub fn solve(&self) -> Option<Solution> {
        let mut in_progress: PriorityQueue<Box<Solution>, isize> = PriorityQueue::new();
        let mut best_score: HashMap<State, isize> = HashMap::new();

        // Seed the in_progress queue and best_score map with the initial state
        let start = Box::new(self.problem.get_start());
        let (state, score) = start.get_state_score();
        best_score.insert(state, score);
        in_progress.push(start, score);

        // Loop through in_progress queue until we get a solution or it goes empty
        // (When we get a solution, it will be the one with the highest potential score)
        while let Some((soln, score)) = in_progress.pop() {
            // If this solution is complete, we're done.
            if soln.is_complete() {
                return Some(*soln.clone());
            }

            // generate all next steps from this solution
            for next in soln.get_next_steps() {

                let (state, score) = next.get_state_score();
                let mut prune = false;
                match best_score.get(&state) {
                    Some(prev_score) => {
                        if *prev_score > score {
                            // TODO: this match is inelegant - figure out how to fix.
                            prune = true;
                        }
                    }
                    _ => ()
                }

                if !prune {
                    best_score.insert(state, score);
                    in_progress.push(Box::new(next), score);
                }
            }
        }

        // No solution was found.
        None
    }
}

// ==============================================================
/*

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
*/

pub struct Day16 {
    valve_ids: HashMap<String, usize>,
    valves: HashMap<usize, ValveInfo>,
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
}

impl<'a> Day for Day16 {
    fn part1(&self) -> Answer {
        let problem = Problem {period: 30, num_agents: 1, valves: &self.valves};
        let solver = Solver { problem: &problem };

        let (final_state, final_score) = solver.solve().unwrap().get_state_score();
        Answer::Number(final_score as usize)
    }

    fn part2(&self) -> Answer {
        let problem = Problem {period: 30, num_agents: 2, valves: &self.valves};
        let solver = Solver { problem: &problem };

        let (final_state, final_score) = solver.solve().unwrap().get_state_score();
        Answer::Number(final_score as usize)
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
    fn test_create_problem() {
        let d = Day16::load("examples/day16_example1.txt");
        let problem = Problem {period: 30, num_agents: 1, valves: &d.valves};
        assert_eq!(problem.period, 30);
        assert_eq!(problem.num_agents, 1);
        assert_eq!(problem.valves.len(), 10);

        let start = problem.get_start();
        assert_eq!(start.action.len(), 1);
        assert_eq!(start.action[0].len(), 0);

        let problem2 = Problem {period: 30, num_agents: 2, valves: &d.valves};
        assert_eq!(problem2.period, 30);
        assert_eq!(problem2.num_agents, 2);
        assert_eq!(problem2.valves.len(), 10);

        let start2 = problem2.get_start();
        assert_eq!(start2.action.len(), 2);
        assert_eq!(start2.action[0].len(), 0);
        assert_eq!(start2.action[1].len(), 0);
    }

    #[test]
    fn test_create_solver() {
        let d = Day16::load("examples/day16_example1.txt");
        let problem = Problem {period: 30, num_agents: 1, valves: &d.valves};
        let solver = Solver { problem: &problem };
    }

    #[test]
    fn test_state_score1() {
        let d = Day16::load("examples/day16_example1.txt");
        let problem = Problem {period: 30, num_agents: 1, valves: &d.valves};

        let start = problem.get_start();
        let (state, score) = start.get_state_score();
        assert_eq!(score, 99);
        assert_eq!(state.position.len(), 1);
        assert_eq!(state.position[0], 0);  // TODO: get id of initial position
        assert_eq!(state.valve_state.len(), 10);
        for n in 0..10 {
            assert_eq!(state.valve_state[n], false);
        }
    }
/* 
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
*/

/*
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
*/

}
