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

enum Xyz {
    X,
    Y,
    Z,
}

pub struct Day2 {
    plays : Vec<(Rps, Xyz)>,
}

impl Day2 {
    pub fn load(filename: &str) -> Day2 {
        let mut plays: Vec<(Rps, Xyz)> = Vec::new();
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
                        "X" => {Xyz::X}
                        "Y" => {Xyz::Y}
                        "Z" => {Xyz::Z}
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
            let my_play = match me {
                Xyz::X => Rps::ROCK,
                Xyz::Y => Rps::PAPER,
                Xyz::Z => Rps::SCISSORS,
            };
            let play_points = match my_play {
                Rps::ROCK => 1,
                Rps::PAPER => 2,
                Rps::SCISSORS => 3,
            };
            let win_lose_points = match (opponent, my_play) {
                // Win
                (Rps::ROCK, Rps::PAPER) => 6,
                (Rps::PAPER, Rps::SCISSORS) => 6,
                (Rps::SCISSORS, Rps::ROCK) => 6,

                // Draw
                (Rps::ROCK, Rps::ROCK) => 3,
                (Rps::PAPER, Rps::PAPER) => 3,
                (Rps::SCISSORS, Rps::SCISSORS) => 3,

                // Lose
                _ => 0,
            };

            score += win_lose_points + play_points
        }

        score
    }

    fn score_match2(&self) -> usize {
        let mut score = 0;
        for (opponent, me) in self.plays.iter() {
            let my_play = match (me, opponent) {
                // Play to lose
                (Xyz::X, Rps::ROCK) => Rps::SCISSORS,
                (Xyz::X, Rps::PAPER) => Rps::ROCK,
                (Xyz::X, Rps::SCISSORS) => Rps::PAPER,

                // Play to draw
                (Xyz::Y, Rps::ROCK) => Rps::ROCK,
                (Xyz::Y, Rps::PAPER) => Rps::PAPER,
                (Xyz::Y, Rps::SCISSORS) => Rps::SCISSORS,

                // Play to win
                (Xyz::Z, Rps::ROCK) => Rps::PAPER,
                (Xyz::Z, Rps::PAPER) => Rps::SCISSORS,
                (Xyz::Z, Rps::SCISSORS) => Rps::ROCK,
            };
            let play_points = match my_play {
                Rps::ROCK => 1,
                Rps::PAPER => 2,
                Rps::SCISSORS => 3,
            };
            let win_lose_points = match (opponent, my_play) {
                // Win
                (Rps::ROCK, Rps::PAPER) => 6,
                (Rps::PAPER, Rps::SCISSORS) => 6,
                (Rps::SCISSORS, Rps::ROCK) => 6,

                // Draw
                (Rps::ROCK, Rps::ROCK) => 3,
                (Rps::PAPER, Rps::PAPER) => 3,
                (Rps::SCISSORS, Rps::SCISSORS) => 3,

                // Lose
                _ => 0,
            };

            score += win_lose_points + play_points
        }

        score
    }
}

impl Day for Day2 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.score_match())
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.score_match2())
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

    #[test]
    fn test_score() {
        let d = Day2::load("data_aoc2022/day2_example1.txt");
        assert_eq!(d.score_match(), 15);
    }

    #[test]
    fn test_score2() {
        let d = Day2::load("data_aoc2022/day2_example1.txt");
        assert_eq!(d.score_match2(), 12);
    }
}
