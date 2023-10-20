use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;
use priority_queue::PriorityQueue;

#[derive(Hash, std::cmp::PartialEq, std::cmp::Eq, Clone)]
enum Action {
    Start_Ore,
    Start_Clay,
    Start_Obsidian,
    Start_Geode,
}

const PART1_ACTIONS: [Action; 3] = [Action::Start_Clay, Action::Start_Obsidian, Action::Start_Geode];

struct ActionList {
    action: Vec<(t, Action, count)>
}

impl ActionList {
    pub fn new() -> ActionList {
        ActionList { action: vec::new() }
    }

    pub fn extend(&self, t: usize, action: Action) -> Option<ActionList> {
        // Make a new action list cloned from this one.

        // Check that the time isn't less than the last time already listed.
        // Check that if the time is equal to the last time listed, the new
        // action type is the same or a greater type.

        // Add the new action at the end
        // If the last action is the same type, add one to its count.
        // Otherwise, append a new entry to the vector.
    }
}

struct SimState {
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,

    ore_bots_ordered: usize,
    clay_bots_ordered: usize,
    obsidian_bots_ordered: usize,
    geode_bots_ordered: usize,
}

impl SimState {
    pub fn new() -> SimState {
        SimState { 
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,

            ore_bots_ordered: 0,
            clay_bots_ordered: 0,
            obsidian_bots_ordered: 0,
            geode_bots_ordered: 0,
        }
    }
}

struct Sim {
    blueprint: &Blueprint,
    // TODO
}

impl Sim {
    pub fn new(blueprint: &Blueprint) -> Sim {
        Sim {blueprint: blueprint}
    }

    pub fn reset(&self) {

    }

    pub fn run(&self) {
        // TODO
    }

    pub fn next_action_time(&self, action: &Action);
}

#[derive(Hash, std::cmp::PartialEq, std::cmp::Eq, Clone)]
struct Solution {
    // Each element is a time and action
    steps: Vec<(usize, Action)>,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {time: 0, steps: Vec::new(), geodes: 3 }
    }

    // Create a new solution that extends another one by one action.
    fn extend(&self, action: &Action, time: usize) -> Solution {
        // TODO
    }

    /*
    fn action_time(&self, blueprint: Blueprint, action: &Action) -> Option<usize> {
        let mut retval = None;

        // Simulate the current sequence.
        // At or after the allowed time, if there are resources to perform the action
        // that's the time we want to return.
        let mut state = SimState::new();
        let mut time = 0;
        let mut action_index = 0;
        let mut ordered_ore_bots = 0;
        let mut ordered_clay_bots = 0;
        let mut ordered_obsidian_bots = 0;
        let mut ordered_geode_bots = 0;
        let mut outranked = false;
        let mut affordable = false;

        while (time < TIME_PART1) && 
              (action_index < self.steps.len()) {}

            let (t_step, a_step) = self.steps[action_index];

            // If we are at the time for this action, consume resources
            // to order bots
            if time == t_step {
                match a_step {
                    Action::Start_Ore(count) => {
                        state.ore -= count * blueprint.ore_cost_ore;
                        ordered_ore_bots += count;
                    }
                    Action::Start_Clay(count) => {
                        state.ore -= count * blueprint.clay_cost_ore;
                        ordered_clay_bots += count;
                    }
                    Action::Start_Obsidian(count) => {
                        state.ore -= count * blueprint.obsidian_cost_ore;
                        state.clay -= count * blueprint.obsidian_cost_clay;
                        ordered_clay_bots += count;
                    }
                    Action::Start_Geode(count) => {
                        state.ore -= count * blueprint.geode_cost_ore;
                        state.obsidian -= count * blueprint.geode_cost_obsidian;
                        ordered_geode_bots += count;
                    }
                }
                // None
                action_index += 1;
            }
            
            // Advance to the next time step
            if time < t_step {
                state.ore += state.ore_bots;
                state.clay += state.clay_bots;
                state.obsidian += state.obsidian_bots;
                state.geodes += state.geode_bots;

                state.ore_bots += ordered_ore_bots;
                ordered_ore_bots = 0;
                state.clay_bots += ordered_clay_bots;
                ordered_clay_bots = 0;
                state.obsidian_bots += ordered_obsidian_bots;
                ordered_obsidian_bots = 0;
                state.geode_bots += ordered_geode_bots;
                ordered_geode_bots = 0;

                time += 1;
            }
        }

        // If the last action was a type greater than the one we're trying to place, 
        if self.steps[self.steps.len()-1].1 > action {
            // Step forward to the next point in time.
            state.ore += state.ore_bots;
            state.clay += state.clay_bots;
            state.obsidian += state.obsidian_bots;
            state.geodes += state.geode_bots;

            state.ore_bots += ordered_ore_bots;
            ordered_ore_bots = 0;
            state.clay_bots += ordered_clay_bots;
            ordered_clay_bots = 0;
            state.obsidian_bots += ordered_obsidian_bots;
            ordered_obsidian_bots = 0;
            state.geode_bots += ordered_geode_bots;
            ordered_geode_bots = 0;

            time += 1;
        }

        // Now, can we afford this action?  Step forward until we can
        let affordable = 
        while (time < TIME_PART1) && () {

        }

        None
        // TODO
    }
    */

    /*
    // Try to schedule an action at the end of this solution.
    // The return value consists of a flag and an Optional solution.
    // If the flag is true, the current solution was found to be suboptimal
    // and should be discarded.  The other component will be None in this case.
    // The Option<Solution> will be None of the provided action can't be sheduled
    // in the time frame.  If the action is feasible, the return value will be Some<s>
    // where s is the current solution extended with the newly scheduled action.
    fn try_action(&self, blueprint: Blueprint, action: &Action, time: usize) -> (bool, Option<Rc<Solution>>) {
        // Set action time to None
        let mut action_time: Option<usize> = None;
        let mut states: [SimState; TIME_PART1+1];

        // Step through time, updating resources
        let mut action_id: usize = 0;
        for t in 1..TIME_PART1 {
            // copy resources left from previous time step.
            states[t] = states[t-1];

            let mut new_ore_bots = 0;
            let mut new_clay_bots = 0;
            let mut new_obsidian_bots = 0;
            let mut new_geode_bots = 0;

            // Start builds scheduled for now
            while (action_id < self.steps.len()) && (self.steps[action_id].0 == t) {
                match self.steps[action_id].1 {
                    Action::Start_Ore(n) => {
                        states[t].ore -= blueprint.ore_cost_ore*n;
                        new_ore_bots += n;
                    }
                    Action::Start_Clay(n) => {
                        states[t].ore -= blueprint.clay_cost_ore*n;
                        new_clay_bots += n;
                    }
                    Action::Start_Obsidian(n) => {
                        states[t].ore -= blueprint.obsidian_cost_ore*n;
                        states[t].clay -= blueprint.obsidian_cost_clay*n;
                        new_obsidian_bots += n;
                    }
                    Action::Start_Geode(n) => {
                        states[t].ore -= blueprint.geode_cost_ore*n;
                        states[t].obsidian -= blueprint.geode_cost_obsidian*n;
                        new_geode_bots += n;
                    }
                }
                action_id += 1;
            }

            // If we can afford the new action at this time...
            //    And we couldn't afford it a moment ago,
            //        Set action time to now
            //    Else
            //        (Action time is already set to an earlier time)
            // Else
            //    Set action time to none.

            // Let the bots do their thing, collecting ore.
            states[t].ore += states[t].ore_bots;
            states[t].clay += states[t].clay_bots;
            states[t].obsidian += states[t].obsidian_bots;
            states[t].geodes += states[t].geode_bots;

            // Add the newly built bots to employ next time
            states[t].ore_bots += new_ore_bots;
            states[t].clay_bots += new_clay_bots;
            states[t].obsidian_bots += new_obsidian_bots;
            states[t].geode_bots += new_geode_bots;
        }     

        // If action time is None, return None as the resulting solution.
        // If action time is Some(t), t < latest action already performed, reject as suboptimal
        // If action time is Some(t), t >= latest action already performed, append (t, action)
        // If action was to start a geode bot, add geodes to the new solution's total.

        // TODO
        (false, None)
    }
    */

}

struct Blueprint {
    id: usize,
    ore_cost_ore: usize,
    clay_cost_ore: usize,
    obsidian_cost_ore: usize,
    obsidian_cost_clay: usize,
    geode_cost_ore: usize,
    geode_cost_obsidian: usize,
}

impl Blueprint {
    fn from_str(s: &str) -> Blueprint {
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
        }

        let caps = LINE_RE.captures(s).unwrap();
        Blueprint {
            id: caps[1].parse::<usize>().unwrap(),
            ore_cost_ore: caps[2].parse::<usize>().unwrap(),
            clay_cost_ore: caps[3].parse::<usize>().unwrap(),
            obsidian_cost_ore: caps[4].parse::<usize>().unwrap(),
            obsidian_cost_clay: caps[5].parse::<usize>().unwrap(),
            geode_cost_ore: caps[6].parse::<usize>().unwrap(),
            geode_cost_obsidian: caps[7].parse::<usize>().unwrap()
        }
    }

    fn quality_level(&self, time_limit: usize) -> usize {
        let mut max_geodes = 0;

        let mut in_progress: PriorityQueue<Rc<Solution>, usize> = PriorityQueue::new();
        let empty_solution = Solution::new();
        in_progress.push(Rc::new(empty_solution), 0);
        let mut sim = Sim::new(self);

        while in_progress.len() > 0 {
            // get candidate solution
            let candidate = in_progress.pop().unwrap().0;

            // reset the simulator
            sim.reset();

            // Run the candidate solution through it.
            let geodes = sim.run(candidate);
            if geodes > max_geodes {
                max_geodes = geodes;
            }

            for new_action in Action::Actions {
                if let Some(t) = sim.next_action_time(new_action) {
                    // extend the candidate with Action::Start_Clay(1) at time t
                    in_progress.push(candidate.extend(new_action, t), geodes);
                }
            }
        }

        max_geodes * self.id
    }
}



pub struct Day19 {
    blueprints: Vec<Blueprint>,
}

impl Day19 {
    pub fn load(filename: &str) -> Day19 {
        let mut blueprints: Vec<Blueprint> = Vec::new();
        
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            blueprints.push(Blueprint::from_str(l));
        }

        Day19 { blueprints }
    }
}

const TIME_PART1: usize = 24;

impl Day for Day19 {
    fn part1(&self) -> Answer {
        let mut sum_quality = 0;
        for blueprint in &self.blueprints {
            sum_quality += blueprint.quality_level(TIME_PART1);
        }

        Answer::Number(sum_quality)
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
        let d = Day19::load("examples/day19_example1.txt");
        assert_eq!(d.blueprints.len(), 2);
    }

    #[test]
    fn test_load_input() {
        let d = Day19::load("data_aoc2022/day19_input.txt");
        assert_eq!(d.blueprints.len(), 30);
    }

    #[test]
    fn test_max_geodes() {
        let d = Day19::load("examples/day19_example1.txt");
        assert_eq!(d.blueprints[0].quality_level(TIME_PART1), 9);
        assert_eq!(d.blueprints[1].quality_level(TIME_PART1), 24);
    }
}
