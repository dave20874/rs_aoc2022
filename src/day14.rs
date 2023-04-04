use std::collections::HashSet;
use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

struct CavernState {
    stone: HashSet<(isize, isize)>,  // locations of stones
    sand: HashSet<(isize, isize)>,   // locations of sand
    lowest_y: isize,
}

impl CavernState {
    pub fn init(lines: &Vec<Vec<(isize, isize)>>) -> CavernState {
        let mut stone = HashSet::new();
        let sand = HashSet::new();
        let mut lowest_y = 0;

        for line in lines {
            for n in 0..line.len()-1 {
                let (x1, y1) = line[n];
                let (x2, y2) = line[n+1];
                let delta_x: isize = if x2>x1 {1} else if x2<x1 {-1} else {0};
                let delta_y: isize = if y2>y1 {1} else if y2<y1 {-1} else {0};

                let mut x = x1;
                let mut y = y1;

                if y > lowest_y {
                    lowest_y = y;
                }
                // Insert stones in cells denoted by this line segment.
                while (x,y) != (x2, y2) {
                    stone.insert((x, y));
                    x += delta_x;
                    y += delta_y;
                }
                stone.insert((x, y));

                if y > lowest_y {
                    lowest_y = y;
                }
            }
        }

        CavernState {stone, sand, lowest_y}
    }

    // inserts a grain of sand, returns false if no more sand can be inserted
    fn insert_sand(&mut self, until_blocked: bool) -> bool {
        // initial position of new sand grain
        let mut x: isize = 500;
        let mut y: isize = 0;
        let mut can_fall = true;

        while can_fall {
            let straight = (x, y + 1);
            let left = (x - 1, y + 1);
            let right = (x + 1, y + 1);

            if y >= self.lowest_y+1 {
                // stop when level lowest+1 is reached.
                can_fall = false;
            } else if !self.stone.contains(&straight) && !self.sand.contains(&straight) {
                // fall straight down
                y += 1;
            } else if !self.stone.contains(&left) && !self.sand.contains(&left) {
                // fall down and left
                y += 1;
                x -= 1;
            } else if !self.stone.contains(&right) && !self.sand.contains(&right) {
                // fall down and left
                y += 1;
                x += 1;
            } else {
                can_fall = false;
            }
        }

        // Add sand where it came to rest
        self.sand.insert((x, y));

        // Return false if no more can be inserted
        if until_blocked {
            y != 0
        }
        else {
            y <= self.lowest_y
        }
    }

    fn insert_sand_loop(&mut self, until_blocked: bool) -> usize {
        let mut inserts = 0;

        while self.insert_sand(until_blocked) {
            inserts += 1;
        }
        if until_blocked {
            // when inserting until blocked, that last one counts.
            inserts += 1;
        }

        inserts
    }
}

pub struct Day14 {
    // Vector of vector of X,Y coord.
    lines: Vec<Vec<(isize, isize)>>,
}

impl Day14 {
    pub fn load(filename: &str) -> Day14 {
        let mut lines: Vec<Vec<(isize, isize)>> = Vec::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+),([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut coords: Vec<(isize, isize)> = Vec::new();
            let l = &line.unwrap();

            for cap in LINE_RE.captures_iter(l) {
                let x = cap[1].parse::<isize>().unwrap();
                let y = cap[2].parse::<isize>().unwrap();
                coords.push( (x, y) );
            }
            lines.push(coords);
        }

        Day14 { lines }
    }

    fn get_cavern(&self) -> CavernState {
        CavernState::init(&self.lines)
    }
}

impl Day for Day14 {
    fn part1(&self) -> Answer {
        let mut cavern = self.get_cavern();
        Answer::Number(cavern.insert_sand_loop(false))
    }

    fn part2(&self) -> Answer {
        let mut cavern = self.get_cavern();
        Answer::Number(cavern.insert_sand_loop(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day14::load("examples/day14_example1.txt");
        assert_eq!(d.lines.len(), 2);
    }

    #[test]
    fn test_cavern() {
        let d = Day14::load("examples/day14_example1.txt");
        let cavern = d.get_cavern();
        assert_eq!(cavern.stone.len(), 20);
    }

    #[test]
    fn test_insert_sand_part1() {
        let d = Day14::load("examples/day14_example1.txt");
        let mut cavern = d.get_cavern();
        assert_eq!(cavern.insert_sand_loop(false), 24);
    }

    #[test]
    fn test_insert_sand_part2() {
        let d = Day14::load("examples/day14_example1.txt");
        let mut cavern = d.get_cavern();
        assert_eq!(cavern.insert_sand_loop(true), 93);
    }

    #[test]
    fn test_part1() {
        let d = Day14::load("examples/day14_example1.txt");
        assert_eq!(d.part1(), Answer::Number(24));
    }

    #[test]
    fn test_part2() {
        let d = Day14::load("examples/day14_example1.txt");
        assert_eq!(d.part2(), Answer::Number(93));
    }
}
