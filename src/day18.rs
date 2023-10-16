use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub struct Day18 {
    cubes: HashSet<(usize, usize, usize)>,
}

impl Day18 {
    pub fn load(filename: &str) -> Day18 {
        let mut cubes: HashSet<(usize, usize, usize)> = HashSet::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+),([0-9]+),([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let x: usize = caps[1].parse::<usize>().unwrap();
                    let y: usize = caps[2].parse::<usize>().unwrap();
                    let z: usize = caps[3].parse::<usize>().unwrap();
                    cubes.insert( (x, y, z) );
                }
                None => {}
            }
        }

        Day18 { cubes }
    }
}

impl Day for Day18 {
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
        let d = Day18::load("examples/day18_example1.txt");
        assert_eq!(d.cubes.len(), 13);
    }
}
