use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct AssignmentPair {
    low1: usize,
    high1: usize,
    low2: usize,
    high2: usize
}

pub struct Day4 {
    assignment_pairs: Vec<AssignmentPair>,
}

impl Day4 {
    pub fn load(filename: &str) -> Day4 {
        let mut assignment_pairs: Vec<AssignmentPair> = Vec::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let low1 = caps[1].parse::<usize>().unwrap();
                    let high1 = caps[2].parse::<usize>().unwrap();
                    let low2 = caps[3].parse::<usize>().unwrap();
                    let high2 = caps[4].parse::<usize>().unwrap();

                    assignment_pairs.push(AssignmentPair {low1, high1, low2, high2});
                }
                None => {}
            }
        }

        Day4 { assignment_pairs }
    }

    // return the number of pairs where one is fully contained in the other.
    fn fully_contained(&self) -> usize {
        let mut total = 0;

        for ap in &self.assignment_pairs {
            if ((ap.low1 <= ap.low2) && (ap.high1 >= ap.high2)) ||  // first fully contains second
               ((ap.low2 <= ap.low1) && (ap.high2 >= ap.high1)) {   // second fully contains first
                   total += 1;
               }    
        }

        total
    }

    // return the number of pairs where one is fully contained in the other.
    fn overlap(&self) -> usize {
        let mut total = 0;

        for ap in &self.assignment_pairs {
            if ((ap.low1 <= ap.low2) && (ap.high1 >= ap.low2)) ||  // first fully contains second
               ((ap.low1 <= ap.high2) && (ap.high1 >= ap.high2)) ||
               ((ap.low1 >= ap.low2) && (ap.high1 <= ap.high2)) ||
               ((ap.low2 >= ap.low1) && (ap.high2 <= ap.high1)) {
                    total += 1;
            }    
        }

        total
    }

}

impl Day for Day4 {
    fn part1(&self) -> Answer {
        Answer::Number(self.fully_contained())
    }

    fn part2(&self) -> Answer {
        Answer::Number(self.overlap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day4::load("examples/day4_example1.txt");
        assert_eq!(d.assignment_pairs.len(), 6);
    }

    #[test]
    fn test_fully_contained() {
        let d = Day4::load("examples/day4_example1.txt");
        assert_eq!(d.fully_contained(), 2);
    }

    #[test]
    fn test_overlap() {
        let d = Day4::load("examples/day4_example1.txt");
        assert_eq!(d.overlap(), 4);
    }
}
