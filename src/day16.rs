use crate::day::{Day, Answer};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use priority_queue::PriorityQueue;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::hash::Hash;

use lazy_static::lazy_static;
use regex::Regex;

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
    distance: HashMap<(usize, usize), usize>,  // distance[(a,b)] = Number of moves from a to b.
}

// This represents a solution in-progress.  It stores the actions taken at each time step by each agent.
#[derive(Hash, PartialEq, Eq, Clone)]
struct Solution {
    opens: Vec<Vec<usize>>,   // opens[agent][n] is the nth valve opened by agent.
    flow_captured : usize,
    max_uncaptured : usize,
}

// Note: The Solver will keep a best potential score for each state observed.  
// If a state is re-visited with a lower potential score, that move will be pruned.

// Solutions will be explored by the Solver by the following process:
//   Take the next solution-in-progress from the priority queue.
//   If the solution-in-progress is complete, 
//     declare it as the optimal solution.
//   Else
//     Generate all valid next-step solutions.
//     For each next step.
//       Discard the potential solution if its max potential is lower than best actual
//       Store the extended solution using the max potential score as its priority.

struct Solver<'a> {
    problem: &'a Problem<'a>,
}

impl<'a> Problem<'a> {
    pub fn new(period: usize, two_agents: bool, valves: &'a HashMap<usize, ValveInfo>, start_position: usize) -> Problem {

        // Compute distances from any valve to any valve.
        let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
        for from in valves.keys() {
            // println!("Finding distances from {}", from);
            let mut visited: HashSet<usize> = HashSet::new();
            let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();  // FIFO
            to_visit.push_back((*from, 0));
            while !to_visit.is_empty() {
                let (to, d) = to_visit.pop_front().unwrap();
                if !visited.contains(&to) {
                    // From <from> we reached <to> for the first time.
                    // println!("    {} to {} is {}", *from, to, d);
                    distance.insert((*from, to), d);
                    visited.insert(to);

                    // Push all potential next steps
                    for next_to in &valves[&to].neighbors {
                        if !visited.contains(&next_to) {
                            to_visit.push_back((*next_to, d+1));
                        }
                    }
                }
            }
        }

        Problem {
            period: period,
            two_agents: two_agents,
            valves: valves,
            start_position: start_position,
            distance: distance,
        }
    }

    // Get the "empty" solution, representing the start state of the puzzle.
    pub fn get_start(&self) -> Solution {
        Solution::new(&self)
    }

    pub fn ttg(&self, path: &Vec<usize>) -> usize {
        let mut ttg = self.period;
        let mut position = self.start_position;
        for p in path {
            let move_dist = self.distance[&(position, *p)];
            position = *p;
            if ttg > move_dist {
                ttg = ttg - move_dist - 1;
            }
            else {
                ttg = 0;
            }
        }

        ttg
    }
}

impl Solution {

    fn new(problem: &Problem) -> Solution {
        // create empty action vectors for all actors
        let mut opens: Vec<Vec<usize>> = Vec::new();

        // First agent's list of opens is empty.
        opens.push(Vec::new());

        if problem.two_agents {
            // Second agent's list of opens is empty.
            opens.push(Vec::new());
        }

        // Construct a vector of all the flow rates available
        let mut flows: Vec<usize> = Vec::new();
        for (_valve_id, valve_info) in problem.valves {
            if valve_info.flow_rate > 0 {
                flows.push(valve_info.flow_rate);
            }
        }
        flows.sort();
        flows.reverse();

        // simulate opening all remaining valves from largest to smallest
        // in the most effective way possible.
        let mut max_uncaptured = 0;
        let mut ttg = problem.period;
        let mut i = 0;
        while ttg >= 2 && i < flows.len() {
            // First agent opens the biggest available valve
            let rate = flows[i];
            i += 1;
            max_uncaptured += (ttg-2)*rate;

            if problem.two_agents && i < flows.len() {
                // Second agent opens the next biggest available valve
                let rate = flows[i];
                i += 1;
                max_uncaptured += (ttg-2)*rate;
            }

            // two time steps pass as we open the valve then move at least one
            // position to a next valve.
            if ttg >= 2 {
                ttg -= 2;
            }
            else {
                ttg = 0;
            }
        }

        Solution { opens, flow_captured: 0, max_uncaptured }
    }
    
    // A solution is complete when its action sequence is as long as self.period allows.
    fn is_complete(&self, problem: &Problem) -> bool {
        for sequence in &self.opens {
            let ttg = problem.ttg(sequence);
            if ttg > 0 {
                return false;
            }
        }

        // none of the sequence lengths were shorter than period
        // so this solution is complete.
        true
    }

    fn get_next_steps(&self, problem: &Problem) -> Vec<Solution> {
        let mut nexts = Vec::new();

        // Compute the list of valves that are open and non-zero flow rate.
        let mut closed_valves: Vec<usize> = Vec::new();
        for (valve_id, valve_info) in problem.valves {
            let mut skip = false;
            // skip this valve_id if it isn't a real valve with a flow rate
            if valve_info.flow_rate == 0 { 
                skip = true;
            }
            else {
                // skip this valve_id if some agent already opened it.
                for agent_id in 0..self.opens.len() {
                    if self.opens[agent_id].contains(valve_id) { 
                        skip = true;
                        break;
                    }
                }
            }

            if !skip {
                closed_valves.push(*valve_id);
            }
        }

        // Figure out which agent to "move".  (The one with most ttg or lowest index)
        let mut agent_id = 0;
        let mut agent_ttg = problem.ttg(&self.opens[agent_id]);
        for other_id in 1..self.opens.len() {
            let other_ttg = problem.ttg(&self.opens[other_id]);
            if other_ttg > agent_ttg {
                agent_id = other_id;
                agent_ttg = other_ttg;
            }
        }

        if agent_ttg > 0 {
            // Have the selected agent go next to each of the open valves.
            for next_valve in closed_valves {
                let mut next_solution = self.clone();
                next_solution.update(agent_id, next_valve, problem);

                nexts.push(next_solution);
            }
        }

        nexts
    }

    fn update(&mut self, agent_id: usize, valve_id: usize, problem: &Problem) {
        // Append a new valve id to the agent's opens vector.
        self.opens[agent_id].push(valve_id);
        // println!("Opening valve {:?}", self.opens[agent_id]);
        let ttg = problem.ttg(&self.opens[agent_id]);
        if ttg > 0 {
            // println!("Capturing flow {} for ttg {}.", problem.valves[&valve_id].flow_rate, ttg);
            self.flow_captured += (ttg) * problem.valves[&valve_id].flow_rate;
        }

        // Construct a vector of all the flow rates available
        let mut flows: Vec<usize> = Vec::new();
        for (_valve_id, valve_info) in problem.valves {
            if valve_info.flow_rate > 0 {
                flows.push(valve_info.flow_rate);
            }
        }
        flows.sort();
        flows.reverse();
        let mut next_flow = 0;

        let mut max_uncaptured = 0;

        let mut agent_ttg: Vec<usize> = Vec::new();
        for agent_id in 0..self.opens.len() {
            agent_ttg.push(problem.ttg(&self.opens[agent_id]));
        }
        for sim_time in 0..problem.period {
            let sim_ttg = problem.period - sim_time;

            for agent_id in 0..self.opens.len() {
                if sim_ttg == agent_ttg[agent_id] && next_flow < flows.len() {
                    // This agent could take the highest available flow rate
                    // then be occupied for two time steps
                    max_uncaptured += flows[next_flow]*(sim_ttg);
                    next_flow += 1;
                    if agent_ttg[agent_id] >= 2 {
                        agent_ttg[agent_id] -= 2;
                    }
                    else {
                        agent_ttg[agent_id] = 0;
                    }
                }
            }
        }

        self.max_uncaptured = max_uncaptured;
        
    }
}

// TODO: Create Trait to encapsulate Problem
// TODO: Make Generic to Problem Trait.
impl<'a> Solver<'a> {

    pub fn solve(&self) -> Option<Rc<Solution>> {
        let mut in_progress: PriorityQueue<Rc<Solution>, usize> = PriorityQueue::new();

        // Seed the in_progress queue and best_score map with the initial state
        let start = Rc::new(self.problem.get_start());
        in_progress.push(start.clone(), 1);  // priority doesn't matter for initial push.

        let mut best_solution = start.clone();

        // Loop through in_progress queue until we get a solution or it goes empty
        // (When we get a solution, it will be the one with the highest potential score)
        while let Some((soln, _priority)) = in_progress.pop() {

            // Evaluate this solution's potential
            let potential = soln.flow_captured + soln.max_uncaptured;

            // Report in_progress queue size, ttg, flowed, potential
            // println!("Depth: {}, captured: {}, Potential: {}, Best: {}", 
            //     in_progress.len(),
            //     soln.flow_captured, 
            //     potential,
            //     best_solution.flow_captured);
            // println!("    Soln: {:?}", soln.opens[0]);

            // Reject this solution if we already have a better one.
            if potential <= best_solution.flow_captured { continue; }

            // Is this solution better than the best so far?
            if soln.flow_captured > best_solution.flow_captured {
                // println!("New best: {:?}", soln.flow_captured);
                best_solution = soln.clone();

                // TODO? Delete entries from in_progress that have less potential than this new find.
            }

            // If this solution isn't complete
            if !soln.is_complete(self.problem) {
                // For each next step
                for next in soln.get_next_steps(self.problem) {
                    // Reject if we already have a better solution
                    let potential = next.flow_captured + next.max_uncaptured;
                    if potential < best_solution.flow_captured { continue; }
                        
                    // Push this solution
                    let priority = potential;
                    in_progress.push(Rc::new(next), priority);
                }
            }
        }

        Some(best_solution)
        
    }
}


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

        let soln= solver.solve();
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

        let soln= solver.solve();
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
        assert_eq!(start.opens.len(), 1);
        assert_eq!(start.opens[0].len(), 0);
        assert_eq!(start.flow_captured, 0);
        let expected = 28*22 + 26*21 + 24*20 + 22*13 + 20*3 + 18*2;
        assert_eq!(start.max_uncaptured, expected);
        let start_position = d.valve_ids.get("AA").unwrap();
        assert_eq!(*start_position, 0);

        let problem2 = Problem::new(26, true, &d.valves, *start_position);
        assert_eq!(problem2.period, 26);
        assert_eq!(problem2.two_agents, true);
        assert_eq!(problem2.valves.len(), 10);

        let start2 = problem2.get_start();
        assert_eq!(start2.opens.len(), 2);
        assert_eq!(start2.opens[0].len(), 0);
        assert_eq!(start2.opens[1].len(), 0);
        assert_eq!(start2.flow_captured, 0);
        let expected = 24*22 + 24*21 + 22*20 + 22*13 + 20*3 + 20*2;
        assert_eq!(start2.max_uncaptured, expected);
        let start_position = d.valve_ids.get("AA").unwrap();
        assert_eq!(*start_position, 0);
    }

    #[test]
    fn test_create_solver() {
        let d = Day16::load("examples/day16_example1.txt");
        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(30, false, &d.valves, *start_position);
        let _solver = Solver { problem: &problem };
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
        let solution = solver.solve().unwrap();

        println!("Search found {:?}", solution.opens[0]);
        println!("Flow Captured: {:?}", solution.flow_captured);

        assert_eq!(solution.flow_captured, 1651);
    }

    
    #[test]
    fn test_search2() {
        let d = Day16::load("examples/day16_example1.txt");

        let start_position = d.valve_ids.get("AA").unwrap();
        let problem = Problem::new(26, true, &d.valves, *start_position);

        let solver = Solver { problem: &problem };
        let solution = solver.solve().unwrap();

        println!("Search found {:?}, {:?}", solution.opens[0], solution.opens[1]);
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
