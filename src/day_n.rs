use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct DayN {
    tbd: Vec<usize>,
}

impl DayN {
    pub fn load(filename: &str) -> DayN {
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

        DayN { tbd: tbd }
    }
}

impl Day for DayN {
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
        let d = DayN::load("examples/dayN_example1.txt");
        assert_eq!(d._tbd.len(), 10);
    }
}
