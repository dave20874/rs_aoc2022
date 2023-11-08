use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;
use priority_queue::PriorityQueue;

#[derive(Hash, std::cmp::PartialEq, std::cmp::Eq, Clone, Debug)]
enum Action {
    StartOre,
    StartClay,
    StartObsidian,
    StartGeode,
}

const PART1_ACTIONS: [Action; 4] = [Action::StartOre, Action::StartClay, Action::StartObsidian, Action::StartGeode];

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct ActionList {
    action: Vec<(usize, Action, usize)>  // time, action, count
}


impl ActionList {
    pub fn new() -> ActionList {
        ActionList { action: Vec::new() }
    }

    pub fn extend(&self, t: usize, action: Action) -> ActionList {
        // Make a new action list cloned from this one.
        let mut newlist: ActionList = self.clone();

        // If the existing list ends with an action of the same type at the same time,
        // just add one to that.
        let len = newlist.action.len();
        // println!("Extending {:?} with {:?} at time {:?}", newlist, action, t);

        if (len >= 1) &&
           (newlist.action[len-1].0 == t) &&
           (newlist.action[len-1].1 == action) {
            // Just increment the last element
            newlist.action[len-1].2 += 1;
            // println!("  Incremented last element.");
        }
        else {
            // Append a new element to the list
            newlist.action.push( (t, action, 1));
            // println!("  Pushed new element.");
        }

        newlist
    }
}

struct Sim<'a> {
    blueprint: &'a Blueprint,

    next_ore_bot: Option<usize>,
    next_clay_bot: Option<usize>,
    next_obsidian_bot: Option<usize>,
    next_geode_bot: Option<usize>,
}

impl<'a> Sim<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Sim {
        Sim {
            blueprint: blueprint,
            next_ore_bot: None,
            next_clay_bot: None,
            next_obsidian_bot: None,
            next_geode_bot: None,
        }
    }

    fn update_affordability(&mut self, 
            t: usize, 
            ore_bots_ordered: usize,
            ore: usize, 
            clay_bots_ordered: usize, 
            clay: usize, 
            obsidian_bots_ordered: usize, 
            obsidian: usize,
            geode_bots_ordered: usize) {

        // If there are non-zero orders in this time step already, we won't consider 
        // any possible orders from earlier time steps.
        // (In fact, if any were possible, that would indicate suboptimal operations?)
        if ore_bots_ordered > 0 || clay_bots_ordered > 0 || obsidian_bots_ordered > 0 || geode_bots_ordered > 0 {
            self.next_ore_bot = None;
            self.next_clay_bot = None;
            self.next_obsidian_bot = None;
            self.next_geode_bot = None;
        }

        // Check whether the various options are possible at this point.
        if clay_bots_ordered > 0 || obsidian_bots_ordered > 0 || geode_bots_ordered > 0 {
            // Respect the hierarchy.
            // once we've started ordering higher order bots in a time step, 
            // don't add more lower ones.
            self.next_ore_bot = None;
        }
        else if ore < self.blueprint.ore_cost_ore {
            // We don't have enough ore for another ore bot at this time
            self.next_ore_bot = None;
        }
        else if self.next_ore_bot == None {
            // Ore bots just went from unaffordable to affordable
            self.next_ore_bot = Some(t);
        }

        if obsidian_bots_ordered > 0 || geode_bots_ordered > 0 {
            // Respect the hierarchy.
            // once we've started ordering higher order bots in a time step, 
            // don't add more lower ones.
            self.next_clay_bot = None;
        }
        else if ore < self.blueprint.clay_cost_ore {
            // We don't have enough ore for another clay bot at this time
            self.next_clay_bot = None;
        }
        else if self.next_clay_bot == None {
            // Clay bots just went from unaffordable to affordable
            self.next_clay_bot = Some(t);
        }

        if geode_bots_ordered > 0 {
            // Respect the hierarchy.
            // once we've started ordering higher order bots in a time step, 
            // don't add more lower ones.
            self.next_obsidian_bot = None;
        }
        else if (ore < self.blueprint.obsidian_cost_ore) ||
                (clay < self.blueprint.obsidian_cost_clay) {
            // We don't have enough material for another obsidian bot at this time
            self.next_obsidian_bot = None;
        }
        else if self.next_obsidian_bot == None {
            // Obsidian bots just went from unaffordable to affordable
            self.next_obsidian_bot = Some(t);
        }

        if (ore < self.blueprint.geode_cost_ore) ||
                (obsidian < self.blueprint.geode_cost_obsidian) {
            // We don't have enough material for another geode bot at this time
            self.next_geode_bot = None;
        }
        else if self.next_geode_bot == None {
            // Geode bots just went from unaffordable to affordable
            self.next_geode_bot = Some(t);
        }        
    }

    

    // run the actionlist and return the number of geodes produced.
    // returns (geodes, score)
    pub fn run(&mut self, action_list: &ActionList) -> (usize, usize, usize, usize) {

        let mut ore = 0;
        let mut clay = 0;
        let mut obsidian = 0;
        let mut geodes = 0;

        let mut ore_bots = 1;
        let mut clay_bots = 0;
        let mut obsidian_bots = 0;
        let mut geode_bots = 0;

        let mut ore_bots_ordered = 0;
        let mut clay_bots_ordered = 0;
        let mut obsidian_bots_ordered = 0;
        let mut geode_bots_ordered = 0;

        let mut t = 0;
        let mut action_index = 0;

        while t < TIME_PART1 {
            let mut acted = false;
            if action_index < action_list.action.len() {
                let (action_time, action, quantity) = &action_list.action[action_index];
                if t == *action_time {
                    // Do an action.  Consume resources, order machines for next time step.
                    match action {
                        Action::StartOre => {
                            ore -= self.blueprint.ore_cost_ore * quantity;
                            ore_bots_ordered += quantity;
                        }
                        Action::StartClay => { 
                            ore -= self.blueprint.clay_cost_ore * quantity;
                            clay_bots_ordered += quantity;
                            // self.next_ore_bot = None
                        }
                        Action::StartObsidian => { 
                            ore -= self.blueprint.obsidian_cost_ore * quantity;
                            clay -= self.blueprint.obsidian_cost_clay * quantity;
                            obsidian_bots_ordered += quantity;
                            // self.next_ore_bot = None;
                            // self.next_clay_bot = None;
                        }
                        Action::StartGeode => { 
                            ore -= self.blueprint.geode_cost_ore * quantity;
                            obsidian -= self.blueprint.geode_cost_obsidian * quantity;
                            geode_bots_ordered += quantity;
                            // self.next_ore_bot = None;
                            // self.next_clay_bot = None;
                            // self.next_obsidian_bot = None;
                        }
                    }

                    acted = true;
                    action_index += 1;
                }
            }

            if !acted {
                // No more actions for this time tick.
                self.update_affordability(t, 
                    ore_bots_ordered, ore, 
                    clay_bots_ordered, clay, 
                    obsidian_bots_ordered, obsidian, 
                    geode_bots_ordered);

                // Let machines generate new resources.
                ore += ore_bots;
                clay += clay_bots;
                obsidian += obsidian_bots;
                geodes += geode_bots;

                // Newly ordered machines can come online now.
                ore_bots += ore_bots_ordered;
                clay_bots += clay_bots_ordered;
                obsidian_bots += obsidian_bots_ordered;
                geode_bots += geode_bots_ordered;

                ore_bots_ordered = 0;
                clay_bots_ordered = 0;
                obsidian_bots_ordered = 0;
                geode_bots_ordered = 0;

                t += 1;
            }
        }

        (geodes, obsidian, clay, ore)
    }

    // Return the earliest time when the given action could be performed without
    // interfering with the other actions already done.  (or None if not possible.)
    pub fn next_action_time(&self, action: &Action) -> Option<usize> {
        match action {
            Action::StartClay => self.next_clay_bot,
            Action::StartOre => self.next_ore_bot,
            Action::StartObsidian => self.next_obsidian_bot,
            Action::StartGeode => self.next_geode_bot,
        }
    }

}

#[derive(Debug)]
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

    fn score(&self, result: (usize, usize, usize, usize)) -> usize {
        let clay_value = 1 + self.clay_cost_ore;
        let obsidian_value = 1 + (self.obsidian_cost_clay * clay_value + self.obsidian_cost_ore);
        let geode_value = 1 + (self.geode_cost_obsidian * obsidian_value + self.geode_cost_ore);

        result.3 + result.2*clay_value + result.1*obsidian_value + result.0*geode_value
    }

    fn quality_level(&self) -> usize {
        let mut max_geodes = 0;
        let mut max_score = 0;
        let mut best_result: [(usize, usize, usize, usize); 25] = [(0,0,0,0);25];

        println!("Assessing QL for {:?}", self);

        let mut in_progress: PriorityQueue<Rc<ActionList>, usize> = PriorityQueue::new();
        let empty_solution = ActionList::new();
        in_progress.push(Rc::new(empty_solution), 0);
        

        let mut _evals = 0;
        while in_progress.len() > 0 {
            // get candidate solution
            let (candidate, score) = in_progress.pop().unwrap();
            if score > max_score {
                max_score = score;
            }
            println!("max: {}, score: {}, max_score: {} in_progress: {} sequences.", max_geodes, score, max_score, in_progress.len());
            println!("{:?}", candidate);
            
            _evals += 1;
            // assert!(evals < 200);
            
            // println!("Evaluating candidate: {:#?}", candidate);

            // Run the candidate solution through simulation
            let mut sim = Sim::new(self);
            let result = sim.run(&candidate);
            //let _score = self.score(result);
            //println!("    Sim completed.  {:?} Geodes, score: {}", 
            //         geodes, score);
            if result.0 > max_geodes {
                max_geodes = result.0;
                //println!("New best: {:?} Geodes via {:?}", geodes, candidate);
            }

            for new_action in PART1_ACTIONS {
                if let Some(t) = sim.next_action_time(&new_action) {
                    // extend the candidate with one of the next possible actions.
                    // println!("  Extending with {:?} at {:?}", new_action, t);
                    let new_seq = candidate.extend(t, new_action);
                    // println!("  {:?}", new_seq);
                    let mut new_sim = Sim::new(self);
                    let result = new_sim.run(&new_seq);
                    let score = self.score(result);
                    // println!("Got score {} from ore:{}, clay:{}, obsidian:{}, geodes:{}",
                    //         score, result.3, result.2, result.1, result.0);

                    if result >= best_result[t] {
                        // This result is the best to time t, keep exploring this chain.
                        in_progress.push(Rc::new(new_seq), score);

                        // update best scores
                        for tt in t..25 {
                            if result > best_result[tt] {
                                best_result[tt] = result;
                            }
                        }
                    }
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
            sum_quality += blueprint.quality_level();
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
        // assert_eq!(d.blueprints[0].quality_level(), 9);
        assert_eq!(d.blueprints[1].quality_level(), 24);
    }
}
