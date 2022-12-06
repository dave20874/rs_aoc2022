use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day3 {
    rucksacks: Vec<String>,
}

impl Day3 {
    pub fn load(filename: &str) -> Day3 {
        let mut rucksacks: Vec<String> = Vec::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([a-zA-Z]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let contents = caps[1].to_string();
                    rucksacks.push(contents);
                }
                None => {}
            }
        }

        Day3 { rucksacks }
    }

    fn priority(c: char) -> usize {
        let p = if (c >= 'a') && (c <= 'z') {
            (c as u32) - ('a' as u32) + 1
        }
        else if (c >= 'A') && (c <= 'Z') {
            (c as u32) - ('A' as u32) + 1 + 26
        }
        else {
            0
        };

        p as usize


    }

    fn priority_sum(&self) -> usize {
        let mut sum = 0;

        for contents in self.rucksacks.iter() {
            let len = contents.len();
            let compartment1 = &contents[0 .. len/2];
            let compartment2 = &contents[len/2 .. len];
            let mut misplaced: char = 'a';
            for c in compartment1.chars() {
                if compartment2.contains(c) {
                    misplaced = c;
                    break;
                }
            }

            let p = Day3::priority(misplaced);
            // println!("In Priority sum, {}", contents);
            // println!("    '{}' contributes {}.", misplaced, p);
            sum += p;
        }

        sum
    }

    fn badge_priority(sack1: &str, sack2: &str, sack3: &str) -> usize {
        let mut p = 0;
        for c in sack1.chars() {
            if sack2.contains(c) && sack3.contains(c) {
                let badge = c;
                p = Day3::priority(badge);
                break;
            }
        }

        p
    }

    fn badge_sum(&self) -> usize {
        let mut sum = 0;

        for group in 0 .. self.rucksacks.len()/3 {
            sum += Day3::badge_priority(
                &self.rucksacks[group*3],
                &self.rucksacks[group*3+1],
                &self.rucksacks[group*3+2]);
        }

        sum
    }
}

impl Day for Day3 {
    fn part1(&self) -> Answer {
        Answer::Number(self.priority_sum())
    }

    fn part2(&self) -> Answer {
        Answer::Number(self.badge_sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day3::load("examples/day3_example1.txt");
        assert_eq!(d.rucksacks.len(), 6);
    }

    #[test]
    fn test_priority_sum() {
        let d = Day3::load("examples/day3_example1.txt");
        assert_eq!(d.priority_sum(), 157);
    }

    #[test]
    fn test_badge_sum() {
        let d = Day3::load("examples/day3_example1.txt");
        assert_eq!(d.badge_sum(), 70);
    }
}
