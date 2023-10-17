use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

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

impl Day for Day19 {
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
        let d = Day19::load("examples/day19_example1.txt");
        assert_eq!(d.blueprints.len(), 2);
    }

    #[test]
    fn test_load_input() {
        let d = Day19::load("data_aoc2022/day19_input.txt");
        assert_eq!(d.blueprints.len(), 30);
    }
}
