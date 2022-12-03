use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

enum Rps {
    ROCK,
    PAPER,
    SCISSORS,
}

pub struct Day2 {
    plays : Vec<(Rps, Rps)>,
}

impl Day2 {
    pub fn load(filename: &str) -> Day2 {
        let mut plays: Vec<(Rps, Rps)> = Vec::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([ABC]) ([XYZ])").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let other = match &caps[1] {
                        "A" => { Rps::ROCK }
                        "B" => { Rps::PAPER }
                        "C" => { Rps::SCISSORS }
                        _ => { panic!("Invalid letter for opponent's play."); }
                    };
                    let me = match &caps[2] {
                        "X" => {Rps::ROCK}
                        "Y" => {Rps::PAPER}
                        "Z" => {Rps::SCISSORS}
                        _ => { panic!("Invalid letter for my play."); }
                    };
                    plays.push( (other, me) );
                }
                _ => ()
            }
        }

        Day2 { plays }
    }

    fn score_match(&self) -> usize {
        let mut score = 0;
        for (opponent, me) in self.plays.iter() {

        }

        score
    }

}

impl Day for Day2 {
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
        let d = Day2::load("data_aoc2022/day2_example1.txt");
        assert_eq!(d.plays.len(), 3);
    }
}
