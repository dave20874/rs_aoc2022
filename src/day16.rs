use crate::day::{Day, Answer};
use std::collections::HashMap;
// use std::collections::HashSet;
use std::rc::Rc;
use priority_queue::PriorityQueue;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::hash::Hash;

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
    two_agents: bool, // true if two agents are active
    valves: &'a HashMap<usize, ValveInfo>,  // what valves exist and how they are connected.
    start_position: usize,  // What position the agents start at.
    max_flow: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
// The action an agent can take in a given time step.
enum Action {
    Nop,
    Open(usize),
    MoveTo(usize),
}




// The state of the problem at a given point in time.
#[derive(Hash)]
#[derive(Eq, PartialEq)]
#[derive(Clone)]
struct State {
    position1: usize,            // The position of agent1
    position2: usize,            // The position of agent2
    valve_open: Vec<bool>,       // whether each valve is open (true) or closed
}

// This represents a solution in-progress.  It stores the actions taken at each time step by each agent.
#[derive(Hash, PartialEq, Eq, Clone)]
struct Solution {
    ttg: usize,
    flow_captured: usize,
    flowed: usize,
    actions1: Vec<Action>,   // action1[t] is the action taken by agent1 at time t
    actions2: Vec<Action>,   // action2[t] is the action taken by agent1 at time t
    state: State,           // The state of the solution after the actions above.
}

// Note: The Solver will keep a best potential score for each state observed.  
// If a state is re-visited with a lower potential score, that move will be pruned.

// Solutions will be explored by the Solver by the following process:
//   Take the next solution-in-progress from the priority queue.
//   If the solution-in-progress is complete, 
//     declare it as the optimal solution.
//   Else
//     For each potential next steps.
//       Generate extended solution that includes this next tstep.
//       Generate state and potential score for this extended solution.
//       Discard the potential solution if this state (position, valves) has been encountered with higher potential score.
//       If the generated state is already recorded with a higher potential score, discard it.
//       Store the extended solution using the potential score as its priority.
//     

struct Solver<'a> {
    problem: &'a Problem<'a>,
}

impl<'a> Problem<'a> {
    pub fn new(period: usize, two_agents: bool, valves: &'a HashMap<usize, ValveInfo>, start_position: usize) -> Problem {
        let mut max_flow = 0;

        for valve_id in valves.keys() {
            max_flow += period * valves[valve_id].flow_rate;
        }

        Problem {
            period: period,
            two_agents: two_agents,
            valves: valves,
            start_position: start_position,
            max_flow: max_flow,
        }
    }

    // Get the "empty" solution, representing the start state of the puzzle.
    pub fn get_start(&self) -> Solution {
        // create empty action vectors for all actors
        let actions1 = Vec::new();
        let actions2 = Vec::new();

        // create initial state of valves
        let state = State {
            position1: self.start_position, 
            position2: self.start_position, 
            valve_open: vec!(false; self.valves.len()),
        };

        Solution {
            ttg: self.period,
            flowed: 0,
            actions1, 
            actions2, 
            flow_captured: 0,
            state, 
        }
    }
}

impl Solution {
    
    // A solution is complete when its action sequence is as long as self.period allows.
    fn is_complete(&self) -> bool {
        self.ttg == 0
    }

    fn get_next_steps(&self, problem: &Problem) -> Vec<Solution> {
        let mut nexts = Vec::new();

        let mut options1: Vec<Action> = Vec::new();
        let position = self.state.position1;

        // Doing nothing is always an option
        // options1.push(Action::Nop);

        // Can this agent open a valve?
        if !self.state.valve_open[position] {
            // This agent can open a valve right here, so add that to it's options.
            options1.push(Action::Open(position));
        }

        // Agent could move to any adjacent position
        for neighbor in &problem.valves[&position].neighbors {
            options1.push(Action::MoveTo(*neighbor));
        }

        let mut options2: Vec<Action> = Vec::new();
        let position = self.state.position2;



        if problem.two_agents {
            // Can this agent open a valve?
            if !self.state.valve_open[position] {
                // This agent can open a valve right here, so add that to it's options.
                options2.push(Action::Open(position));
            }

            // Agent could move to any adjacent position
            for neighbor in &problem.valves[&position].neighbors {
                options2.push(Action::MoveTo(*neighbor));
            }
        }
        else {
            // Doing nothing is always an option
            options2.push(Action::Nop);
        }

        // For each combination of actions the actors could take ...
        // Generate the state produced by each actor taking it's action from this state.
        // Push the new solution candidate.
        for action1 in &options1 {
            for action2 in &options2 {
                let next_solution = self.do_actions(problem, &action1, &action2);
                nexts.push(next_solution);
            }
        }

        nexts
    }

    // Compute maximum unrealized score for this solution
    fn max_potential(&self, problem: &Problem, _ttg: usize) -> usize {
        // Throw the flow rates of all closed valves into a vector.
        let mut flows: Vec<usize> = Vec::new();
        for (id, info) in problem.valves {
            if !self.state.valve_open[*id] {
                flows.push(info.flow_rate);
            }
        }

        // Sort the flow rates.  (Highest values first)
        flows.sort();
        flows.reverse();

        // Simulate the best case scenario where each agent opens the highest
        // flow rate valve, capturing all the future flow, for the rest of the 
        // time to go.

        let mut index = 0;
        let mut potential_score = 0;
        let mut sim_ttg = self.ttg;
        while sim_ttg > 1 {
            // agent 1 : capture flows[index]
            if index < flows.len() {
                // print!(" + {}*{}", sim_ttg-1, flows[index]);
                potential_score += flows[index] * (sim_ttg-1);
                index += 1;
            }

            if problem.two_agents {
                // agent 2 : capture flows[index]
                if index < flows.len() {
                    // print!(" + {}*{}", sim_ttg-1, flows[index]);
                    potential_score += flows[index] * (sim_ttg-1);
                    index += 1;
                }
            }

            sim_ttg -= 2;
        }
        // println!(" = {}",  potential_score);
        // println!("returning {} + {} = {}", self.flow_captured, potential_score, self.flow_captured+potential_score);

        self.flow_captured + potential_score
    }

    fn do_actions(&self, problem: &Problem, action1: &Action, action2: &Action) -> Solution {
        let mut actions1 = self.actions1.clone();
        actions1.push(*action1);
        let mut actions2 = self.actions2.clone();
        actions2.push(*action2);
        let mut state = self.state.clone();
        let mut flow_captured = self.flow_captured;

        let mut new_flow: usize = 0;
        //x println!("Evaluating new flow");
        for valve_id in problem.valves.keys() {
            if self.state.valve_open[*valve_id] {
                //x println!("    Valve {} flows {}", *valve_id, problem.valves[valve_id].flow_rate);
                new_flow += problem.valves[valve_id].flow_rate;
            }
            else {
                //x println!("    Valve {} closed, flows 0.", *valve_id);
            }
        }
        //x  println!("New flow is {}.", new_flow);

        match action1 {
            Action::Nop => {
                // No Operation
            }
            Action::Open(valve_id) => {
                // Open a valve, capturing all of its future flow
                if !state.valve_open[*valve_id] {
                    flow_captured += (self.ttg-1) * problem.valves[valve_id].flow_rate;
                    state.valve_open[*valve_id] = true;
                }
            }
            Action::MoveTo(position) => {
                // Move agent 1 to a new position
                state.position1 = *position
            }
        }

        match action2 {
            Action::Nop => {
                // No Operation
            }
            Action::Open(valve_id) => {
                // Open a valve
                if !state.valve_open[*valve_id] {
                    flow_captured += (self.ttg-1) * problem.valves[valve_id].flow_rate;
                    state.valve_open[*valve_id] = true;
                }
            }
            Action::MoveTo(position) => {
                // Move agent 2 to a new position
                state.position2 = *position;
            }
        }

        let new_solution = Solution {
            ttg: self.ttg - 1,
            flowed: self.flowed + new_flow,
            actions1: actions1,
            actions2: actions2,
            flow_captured,
            state,           // The state of the solution after the actions above.
        };

        new_solution
    }


    /*
    pub fn get_state_score(&self, problem: &Problem) -> (State, isize) {
        // start with all valves closed, all agents at start position
        let mut valve_state = vec!(false, problem.valves.len());  
        let mut position = vec!(START_POS, problem.num_agents);
        let mut ttg = problem.period;
        let mut flow_released = 0;

        // Perform all the operations in the solution to update valves, positions, ttg
        for t in 0..self.actions[0].len() {
            for agent in 0..problem.num_agents {
                match self.actions[agent][t] {
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

        let state = State {valve_open: valve_state, position };

        ( state, flow_released+potential_flow )
    }
    */

    /* TODO: Move to Problem
    pub fn is_complete(&self, problem: &Problem) -> bool {
        self.actions.len() == problem.period
    }
    */

    /* TODO: Move to Problem
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
    */
}

// TODO: Create Trait to encapsulate Problem
// TODO: Make Generic to Problem Trait.
impl<'a> Solver<'a> {
    pub fn solve(&self) -> Option<Solution> {
        let mut in_progress: PriorityQueue<Box<Solution>, usize> = PriorityQueue::new();
        let mut score_for_state: HashMap<State, usize> = HashMap::new();

        // Seed the in_progress queue and best_score map with the initial state
        let start = Box::new(self.problem.get_start());
        let score = start.max_potential(self.problem, start.ttg);
        score_for_state.insert(start.state.clone(), score);
        in_progress.push(start, score);

        let mut best_so_far = 0;

        // Loop through in_progress queue until we get a solution or it goes empty
        // (When we get a solution, it will be the one with the highest potential score)
        while let Some((soln, _score)) = in_progress.pop() {
            // If this solution is complete, we're done.
            if soln.is_complete() {
                return Some(*soln.clone());
            }

            let potential = soln.max_potential(&self.problem, soln.ttg);

            println!("TTG: {}, Flowed: {}, Potential: {}", soln.ttg, soln.flow_captured, potential);

            // If this solution isn't the best we've seen for it's state, don't propagate it
            let best_from_state = score_for_state[&soln.state];
            if potential >= best_from_state {
                // generate all next steps from this solution
                for next in soln.get_next_steps(self.problem) {
                    let potential_score = next.max_potential(&self.problem, next.ttg);
                    let mut keep: bool = false;

                    if potential_score < best_so_far {
                        // We've already seen solutions better than this could be
                        println!("Ew.");
                        keep = false;
                    }
                    else if score_for_state.contains_key(&next.state) {
                        let prev_best = score_for_state[&next.state];
                        if prev_best < potential_score {
                            // This solution is better than what was already seen, keep it.
                            keep = true;
                        }
                    }
                    else {
                        // This solution brings us to a state for the first time.
                        keep = true;
                    }

                    if keep {
                        // Add this solution to the search list
                        
                        
                        if next.flow_captured > best_so_far {
                            best_so_far = next.flow_captured;
                        }
                        score_for_state.insert(next.state.clone(), potential_score);
                        in_progress.push(Box::new(next), potential_score);

                    }
                }
            }
            else {
                println!("Nope.");
            }
        }

        // No solution was found.
        None
    }

    pub fn solve2(&self) -> Option<Rc<Solution>> {
        let mut in_progress: PriorityQueue<Rc<Solution>, usize> = PriorityQueue::new();
        let mut visited: HashMap<State, usize> = HashMap::new();

        // Seed the in_progress queue and best_score map with the initial state
        let start = Rc::new(self.problem.get_start());
        // let priority = start.flowed;
        // let score = start.max_potential(self.problem, start.ttg);
        visited.insert(start.state.clone(), start.flow_captured);
        in_progress.push(start.clone(), 1);  // priority doesn't matter for initial push.

        let mut best_solution = start.clone();

        // Loop through in_progress queue until we get a solution or it goes empty
        // (When we get a solution, it will be the one with the highest potential score)
        while let Some((soln, _priority)) = in_progress.pop() {


            // Evaluate this solution's potential
            let potential = soln.max_potential(&self.problem, soln.ttg);

            // Report in_progress queue size, ttg, flowed, potential
            println!("Depth: {} TTG: {}, flowed: {}, captured: {}, Potential: {}, Best: {}", 
                in_progress.len(),
                soln.ttg, 
                soln.flowed,
                soln.flow_captured, 
                potential,
                best_solution.flow_captured);

            // Reject this solution if we already have a better one.
            if potential <= best_solution.flow_captured { continue; }

            // Reject this solution if we already have a better way to this state.
            let best_score: usize  = visited[&soln.state];
            if best_score > soln.flow_captured {
                println!("Found better.");
                continue; 
            }

            // Is this solution better than the best so far?
            if soln.flow_captured > best_solution.flow_captured {
                println!("New best: {:?}", soln.flow_captured);
                best_solution = soln.clone();
            }



            if best_solution.flowed == 1641 {
                println!("=========== Found Best ====================");
                // panic!("Did it?");
            }

            // If this solution isn't complete
            if !soln.is_complete() {
                // For each next step
                for next in soln.get_next_steps(self.problem) {
                    // Reject if we already have a better solution
                    let potential = next.max_potential(&self.problem, next.ttg);
                    if potential < best_solution.flow_captured { continue; }
                    
                    // Reject if we've seen this state with a better score
                    if visited.contains_key(&next.state) { 
                        let best_score: usize  = visited[&next.state];
                        if best_score >= next.flow_captured {
                            // println!("Not the best way.");
                            continue; 
                        }
                        else { 
                            visited.insert(next.state.clone(), soln.flow_captured);
                        }
                    }
                    else {
                        visited.insert(next.state.clone(), soln.flow_captured);
                    }
                        
                    // Push this solution (with actual flow as priority)
                    /// visited.insert(next.state.clone());
                    let elapsed = self.problem.period - next.ttg;
                    let priority = 20*next.flowed/elapsed + potential;  // 100 * next.flow_captured / elapsed;
                    in_progress.push(Rc::new(next), priority);
                }
            }
            else {
                // The first complete solution is the best?
                println!("==== COMPLETE =====================");
                break;
            }
        }

        Some(best_solution)
        
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
        let start_position = self.valve_ids.get("AA").unwrap();
        let problem = Problem::new(30, false, &self.valves, *start_position);
        
        let solver = Solver { problem: &problem };

        let soln= solver.solve2();
        match soln {
            Some(soln) => {
                Answer::Number(soln.flow_captured)
            }
            None => {
                Answer::None
            }
        }
    }

    fn part2(&self) -> Answer {
        let start_position = self.valve_ids.get("AA").unwrap();
        let problem = Problem::new(26, true, &self.valves, *start_position);
        let solver = Solver { problem: &problem };

        let soln= solver.solve2();
        match soln {
            Some(soln) => {
                Answer::Number(soln.flow_captured)
            }
            None => {
                Answer::None
            }
        }
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
        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(30, false, &d.valves, *start_position);
        assert_eq!(problem.period, 30);
        assert_eq!(problem.two_agents, false);
        assert_eq!(problem.valves.len(), 10);

        let start = problem.get_start();
        assert_eq!(start.actions1.len(), 0);
        assert_eq!(start.actions2.len(), 0);

        let problem2 = Problem::new(30, false, &d.valves, *start_position);
        assert_eq!(problem2.period, 30);
        assert_eq!(problem2.two_agents, true);
        assert_eq!(problem2.valves.len(), 10);

        let start2 = problem2.get_start();
        assert_eq!(start2.actions1.len(), 0);
        assert_eq!(start2.actions2.len(), 0);
    }

    #[test]
    fn test_create_solver() {
        let d = Day16::load("examples/day16_example1.txt");
        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(30, false, &d.valves, *start_position);
        let _solver = Solver { problem: &problem };
    }

    #[test]
    fn test_state_score1() {
        let d = Day16::load("examples/day16_example1.txt");
        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(30, false, &d.valves, *start_position);

        let start = problem.get_start();
        let max_potential = start.max_potential(&problem, start.ttg);
        let expected = 29*22 + 27*21 + 25*20 + 23*13 + 21*3 + 19*2;
        assert_eq!(max_potential, expected);
        let start_position = d.valve_ids.get("AA").unwrap();
        assert_eq!(*start_position, 0);
        assert_eq!(start.state.position1, *start_position);
        assert_eq!(start.state.valve_open.len(), 10);
        for n in 0..10 {
            assert_eq!(start.state.valve_open[n], false);
        }
    }

    #[test]
    fn test_state_score2() {
        let d = Day16::load("examples/day16_example1.txt");
        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(26, true, &d.valves, *start_position);

        let start = problem.get_start();
        let max_potential = start.max_potential(&problem, start.ttg);
        let expected = 25*22 + 25*21 + 23*20 + 23*13 + 21*3 + 21*2;
        assert_eq!(max_potential, expected);
        let start_position = d.valve_ids.get("AA").unwrap();
        assert_eq!(*start_position, 0);
        assert_eq!(start.state.position1, *start_position);
        assert_eq!(start.state.position2, *start_position);
        assert_eq!(start.state.valve_open.len(), 10);
        for n in 0..10 {
            assert_eq!(start.state.valve_open[n], false);
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

    #[test]
    fn test_search() {
        let d = Day16::load("examples/day16_example1.txt");

        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(30, false, &d.valves, *start_position);

        let solver = Solver { problem: &problem };
        let solution = solver.solve2().unwrap();

        println!("Search found {:?}", solution.actions1);
        println!("Flow Captured: {:?}", solution.flow_captured);

        assert_eq!(solution.flow_captured, 1651);
    }

    
    #[test]
    fn test_search2() {
        let d = Day16::load("examples/day16_example1.txt");

        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(26, true, &d.valves, *start_position);

        let solver = Solver { problem: &problem };
        let solution = solver.solve2().unwrap();

        println!("Search found {:?}, {:?}", solution.actions1, solution.actions2);
        println!("Flow Captured: {:?}", solution.flow_captured);

        assert_eq!(solution.flow_captured, 1707);
    }
/*
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
