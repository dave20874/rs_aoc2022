use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day4 {
    _tbd: Vec<usize>,
}

impl Day4 {
    pub fn load(filename: &str) -> Day4 {
        let mut tbd: Vec<usize> = Vec::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let n = caps[1].parse::<usize>().unwrap();
                    tbd.push(n);
                }
                None => {}
            }
        }

        Day4 { _tbd: tbd }
    }
}

impl Day for Day4 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(1)
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day4::load("examples/day4_example1.txt");
        assert_eq!(d._tbd.len(), 10);
    }
}
