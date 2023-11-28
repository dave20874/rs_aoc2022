use crate::day::{Day, Answer};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;
use priority_queue::PriorityQueue;

const MAX_P2_BOTS: usize = 3;

#[derive(Debug)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const MATERIALS: [Material; 4] = [Material::Ore, Material::Clay, Material::Obsidian, Material::Geode];

/*
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
*/


#[derive(Hash, std::cmp::PartialEq, std::cmp::Eq, Debug, Clone, Copy)]
struct ProductionPlan {
    // How many bots to make of each type
    // The Sim class will figure out the best time to make each one.

    // p1_bots need to be satisfied before any higher bots are made.
    p1_bots: (usize, usize, usize, usize),

    // p2_bots can be satisfied after production of higher bots starts.
    p2_bots: (usize, usize, usize, usize),
}

impl ProductionPlan {
    fn new() -> ProductionPlan {
        ProductionPlan { p1_bots: (0, 0, 0, 0), p2_bots: (0, 0, 0, 0) }
    }

    fn to_result(&self) -> Result {
        Result { resource: (
            self.p1_bots.0 + self.p2_bots.0, 
            self.p1_bots.1 + self.p2_bots.1,
            self.p1_bots.2 + self.p2_bots.2,
            self.p1_bots.3 + self.p2_bots.3)}
    }

    // Create a new plan based on this one but with an additional bot ordered.
    fn add(&self, bot_type: &Material, p1: bool) -> ProductionPlan {
        let mut p1_bots = self.p1_bots;
        let mut p2_bots = self.p2_bots;

        if p1 {
            match bot_type {
                Material::Ore => p1_bots.3 += 1,
                Material::Clay => p1_bots.2 += 1,
                Material::Obsidian => p1_bots.1 += 1,
                Material::Geode => p1_bots.0 += 1,
            }
        }
        else {
            match bot_type {
                Material::Ore => p2_bots.3 += 1,
                Material::Clay => p2_bots.2 += 1,
                Material::Obsidian => p2_bots.1 += 1,
                Material::Geode => p2_bots.0 += 1,
            }
        }

        // Limit P2 bots to a max of 1.
        if p2_bots.0 > MAX_P2_BOTS {
            p1_bots.0 += p2_bots.0-MAX_P2_BOTS;
            p2_bots.0 = MAX_P2_BOTS;
        }
        if p2_bots.1 > MAX_P2_BOTS {
            p1_bots.1 += p2_bots.1-MAX_P2_BOTS;
            p2_bots.1 = MAX_P2_BOTS;
        }
        if p2_bots.2 > MAX_P2_BOTS {
            p1_bots.2 += p2_bots.2-MAX_P2_BOTS;
            p2_bots.2 = MAX_P2_BOTS;
        }
        if p2_bots.3 > MAX_P2_BOTS {
            p1_bots.3 += p2_bots.3-MAX_P2_BOTS;
            p2_bots.3 = MAX_P2_BOTS;
        }

        ProductionPlan { p1_bots, p2_bots }
    }

    // Subtract a bot from those that need to be ordered
    fn remove(&mut self, bot_type:&Material) {
        match bot_type {
            Material::Ore => {
                if self.p1_bots.3 > 0 {
                    self.p1_bots.3 -= 1;
                }
                else if self.p2_bots.3 > 0 {
                    self.p2_bots.3 -= 1;
                }
                else {
                    // Can't remove when nothing is there!
                    panic!();
                }
            }
            Material::Clay => {
                if self.p1_bots.2 > 0 {
                    self.p1_bots.2 -= 1;
                }
                else if self.p2_bots.2 > 0 {
                    self.p2_bots.2 -= 1;
                }
                else {
                    // Can't remove when nothing is there!
                    panic!();
                }
            }
            Material::Obsidian => {
                if self.p1_bots.1 > 0 {
                    self.p1_bots.1 -= 1;
                }
                else if self.p2_bots.1 > 0 {
                    self.p2_bots.1 -= 1;
                }
                else {
                    // Can't remove when nothing is there!
                    panic!();
                }
            }
            Material::Geode => {
                if self.p1_bots.0 > 0 {
                    self.p1_bots.0 -= 1;
                }
                else if self.p2_bots.0 > 0 {
                    self.p2_bots.0 -= 1;
                }
                else {
                    // Can't remove when nothing is there!
                    panic!();
                }
            }
        }
    }

    fn p1_ore(&self) -> usize {
        self.p1_bots.3
    }

    fn ore(&self) -> usize {
        self.p1_bots.3 + self.p2_bots.3
    }

    fn p1_clay(&self) -> usize {
        self.p1_bots.2
    }

    fn clay(&self) -> usize {
        self.p1_bots.2 + self.p2_bots.2
    }

    fn p1_obsidian(&self) -> usize {
        self.p1_bots.1
    }

    fn obsidian(&self) -> usize {
        self.p1_bots.1 + self.p2_bots.1
    }

    /* fn p1_geode(&self) -> usize {
        self.p1_bots.0
    }
    */

    fn geode(&self) -> usize {
        self.p1_bots.0 + self.p2_bots.0
    }
}


#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
struct Result {
    // geodes, obsidian, clay, ore
    resource: (usize, usize, usize, usize),
}

impl Result {
    fn new() -> Result {
        Result { resource: (0, 0, 0, 0) }
    }

    fn add(&mut self, for_material: &Material) {

        match for_material {
            Material::Ore => self.resource.3 += 1,
            Material::Clay => self.resource.2 += 1,
            Material::Obsidian => self.resource.1 += 1,
            Material::Geode => self.resource.0 += 1,
        }
    }

    fn clear(&mut self) {
        self.resource.0 = 0;
        self.resource.1 = 0;
        self.resource.2 = 0;
        self.resource.3 = 0;
    }

    fn ore(&self) -> usize {
        self.resource.3
    }

    fn clay(&self) -> usize {
        self.resource.2
    }

    fn obsidian(&self) -> usize {
        self.resource.1
    }

    fn geode(&self) -> usize {
        self.resource.0
    }

    fn produce(&mut self, other: &Result) {
        self.resource.0 += other.resource.0;
        self.resource.1 += other.resource.1;
        self.resource.2 += other.resource.2;
        self.resource.3 += other.resource.3;
    }

    fn use_resource(&mut self, resource: &Material, amount: usize) {
        match resource {
            Material::Ore => { self.resource.3 -= amount; }
            Material::Clay => { self.resource.2 -= amount; }
            Material::Obsidian => { self.resource.1 -= amount; }
            Material::Geode => { self.resource.0 -= amount; }
        }
    }
}

struct Sim<'a> {
    blueprint: &'a Blueprint,
}

impl<'a> Sim<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Sim {
        Sim {
            blueprint: blueprint,
        }
    }

    /*
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
*/

    // run the actionlist and return the number of geodes produced.
    // returns (geodes, score)
    pub fn run(&mut self, plan: &ProductionPlan, verbose: bool) -> Result {
        let mut to_order = *plan;

        let mut material: Result = Result::new();
        let mut bots: Result = Result::new();
        bots.add(&Material::Ore);
        let mut bots_ordered = Result::new();

        for _t in 0..TIME_LIMIT {
            // Bot ordering phase            
            bots_ordered.clear();
            if verbose {
                println!("time: {}", _t+1);
            }

            let mut changed = true;
            while changed {
                changed = false;

                // Should we order a geode bot?
                // Yes, if we have the resources, it's in the plan and there aren't any higher 
                // priority plan components blocking it.
                if self.blueprint.can_afford(&Material::Geode, &material) && 
                    (to_order.geode() > 0)  &&
                    (to_order.p1_obsidian() == 0) &&
                    (to_order.clay() == 0) &&
                    (to_order.ore() == 0)  {
                        // Add a geode bot to the order
                        bots_ordered.add(&Material::Geode);
                        to_order.remove(&Material::Geode);
                        material.use_resource(&Material::Obsidian, self.blueprint.geode_cost_obsidian);
                        material.use_resource(&Material::Ore, self.blueprint.geode_cost_ore);
                        
                        changed = true;
                }
                
                // Should we order an obsidian bot?
                else if self.blueprint.can_afford(&Material::Obsidian, &material) && 
                    (to_order.obsidian() > 0) &&        
                    (to_order.p1_clay() == 0) &&
                    (to_order.ore() == 0) {
                        // Add an obsidian bot to the order
                        bots_ordered.add(&Material::Obsidian);
                        to_order.remove(&Material::Obsidian);
                        
                        material.use_resource(&Material::Clay, self.blueprint.obsidian_cost_clay);
                        material.use_resource(&Material::Ore, self.blueprint.obsidian_cost_ore);
                        

                        changed = true;
                }

                // Should we order a clay bot?
                else if self.blueprint.can_afford(&Material::Clay, &material) && 
                    (to_order.clay() > 0) &&        
                    (to_order.p1_ore() == 0) {
                        // Add a clay bot to the order
                        bots_ordered.add(&Material::Clay);
                        to_order.remove(&Material::Clay);
                        material.use_resource(&Material::Ore, self.blueprint.clay_cost_ore);
                        

                        changed = true;
                }

                // Should we order an ore bot?
                else if self.blueprint.can_afford(&Material::Ore, &material) && 
                    (to_order.ore() > 0)  {
                        // Add an ore bot to the order
                        bots_ordered.add(&Material::Ore);
                        to_order.remove(&Material::Ore);
                        
                        material.use_resource(&Material::Ore, self.blueprint.ore_cost_ore);
                        

                        changed = true;
                }
            }
            if verbose {
                println!("    Bots ordered {:?}", to_order);
            }

            // Production phase
            material.produce(&bots);
            if verbose {
                println!("    Materials {:?}", material);
            }

            // Bot delivery phase
            bots.produce(&bots_ordered);
            if verbose {
                println!("    Bots now: {:?}", bots);
                println!();
            }
        }

        material
    }

/*
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
    */

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
            geode_cost_obsidian: caps[7].parse::<usize>().unwrap(),
        }
    }

    /*
    fn score(&self, result: (usize, usize, usize, usize)) -> usize {
        let clay_value = 1 + self.clay_cost_ore;
        let obsidian_value = 1 + (self.obsidian_cost_clay * clay_value + self.obsidian_cost_ore);
        let geode_value = 1 + (self.geode_cost_obsidian * obsidian_value + self.geode_cost_ore);

        result.3 + result.2*clay_value + result.1*obsidian_value + result.0*geode_value
    }
    */

    fn can_afford(&self, material: &Material, result: &Result) -> bool {
        match material {
            Material::Ore => {
                result.ore() >= self.ore_cost_ore
            }
            Material::Clay => {
                result.ore() >= self.clay_cost_ore
            }
            Material::Obsidian => {
                result.ore() >= self.obsidian_cost_ore &&
                result.clay() >= self.obsidian_cost_clay
            }
            Material::Geode => {
                result.ore() >= self.geode_cost_ore &&
                result.obsidian() >= self.geode_cost_obsidian
            }
        }
    }

    fn quality_level(&self) -> usize {
        let mut sim = Sim::new(self);

        println!("Assessing QL for {:?}", self);

        let mut plans: HashSet<ProductionPlan> = HashSet::new();
        let mut in_progress: PriorityQueue<Rc<(ProductionPlan, Result)>, Result> = PriorityQueue::new();
        let empty_plan = ProductionPlan::new();
        let base_result = sim.run(&empty_plan, false);
        let mut best_result = base_result;
        plans.insert(empty_plan);
        in_progress.push(Rc::new((empty_plan, base_result)), empty_plan.to_result());

        let mut verbose;

        let mut _evals = 0;
        while in_progress.len() > 0 {
            verbose = false;
            _evals += 1;
            // println!("Evals: {}, Max: {}", _evals, best_result.resource.0);

            // Pop next candidate and its results
            let (popped, _priority) = in_progress.pop().unwrap();
            let plan = popped.0;
            let base_result = popped.1;



            // println!("popped plan: {:?}, result: {:?}", plan, base_result);

            let mut improved = false;

            // Look at ways to extend this plan
            for material in MATERIALS {
                verbose = false;

                // If we have resources to make another bot for <material>,
                // explore that option.
                // (Unless it's already registered to be explored or it doesn't improve on the base plan.)
                if self.can_afford(&material, &base_result) {
                    for p1 in &[true] {
                        let new_plan = plan.add(&material, *p1);
                        if !plans.contains(&new_plan) {
                            // This is an unexplored plan
                      
                            let bot_totals = new_plan.to_result();
                            if (bot_totals.resource.3 == 0) &&
                                (bot_totals.resource.2 == 4) &&
                                (bot_totals.resource.1 == 2) &&
                                (bot_totals.resource.0 == 2) {
                                    // This should be the winner
                                    verbose = true;
                            }
                            else {
                                verbose = false;
                            }
                            if verbose {
                                println!("THIS SHOULD BE A WINNER");
                                println!("plan: {:?}", new_plan);
                            }
                            
                            let new_result = sim.run(&new_plan, verbose);
                            if verbose {
                                println!("Simulate unexplored plan: {:?}", new_plan);
                                println!("    result: {:?}", new_result);
                            }
                            if new_result > base_result {
                                // The new action improves on the plan
                                // Add it to the list of options to explore.
                                // let priority: usize = 0;  // TODO-DW establish priority mechanism.
                                in_progress.push(Rc::new((new_plan, new_result)), new_plan.to_result());
                                plans.insert(new_plan);
                                improved = true;
                                if verbose {
                                    println!("Improved by adding {:?}", &material);
                                }
                            }
                            else {
                                if verbose {
                                    println!("No improvement from this plan.");
                                }
                            }
                        }
                        else {
                            if verbose {
                                println!("Pruned exploration on duplicate plan.");
                            }
                        }
                    }
                }
            }

            if !improved {
                // This plan, base_result couldn't be improved.  Is it a global best?
                if verbose {
                    println!("Local max: {:?}, {:?}", plan, base_result);
                }   
                if base_result > best_result {
                    best_result = base_result;
                }
            }
            
        }

        // best result for time 24 has the geode count.
        let max_geodes = best_result.geode();

        println!("Most geodes: {}", max_geodes);

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

const TIME_LIMIT: usize = 24;

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
        assert_eq!(d.blueprints[0].quality_level(), 9);
        // assert_eq!(d.blueprints[1].quality_level(), 24);
    }
}
