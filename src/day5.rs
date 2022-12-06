use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub struct Day5 {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Day5 {
    pub fn load(filename: &str) -> Day5 {

        lazy_static! {
            static ref CRATE_RE: Regex =
                Regex::new("\\[[A-Z]\\]").unwrap();
            static ref STACK_RE: Regex =
                Regex::new(" 1   ").unwrap();
            static ref MOVE_RE: Regex =
                Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        }

        let mut crate_lines: Vec<String> = Vec::new();
        let mut moves: Vec<Move> = Vec::new();
        let mut stacks: Vec<Vec<char>> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            // Process crate line by storing it for now.
            let caps = CRATE_RE.captures(&l);
            match caps {
                Some(_caps) => {
                    crate_lines.push(l.to_string());
                }
                None => {}
            }

            // Process stacks line, when it is seen process those crates.
            let caps = STACK_RE.captures(&l);
            match caps {
                Some(_caps) => {
                    // Determine number of stacks from line length.
                    let num_stacks = (l.len()+1)/4;

                    // Create the stacks
                    for _ in 0..num_stacks {
                        stacks.push(Vec::new());
                    }

                    // Process each line of crates
                    while crate_lines.len() > 0 {
                        let crate_line = crate_lines.pop().unwrap();
                        for stack_no in 0..num_stacks {
                            let crate_id = crate_line.chars().nth(stack_no*4+1).unwrap();
                            if crate_id != ' ' {
                                stacks[stack_no].push(crate_id);
                            }
                        }
                    }
                }
                None => ()
            }

            // Process move line
            let caps = MOVE_RE.captures(&l);
            match caps {
                Some(cap) => {
                    let count = cap[1].parse::<usize>().unwrap();
                    let from = cap[2].parse::<usize>().unwrap();
                    let to = cap[3].parse::<usize>().unwrap();

                    moves.push(Move {count, from, to});
                }
                None => ()
            }
        }

        Day5 { stacks, moves }
    }

    fn process_moves(&self, part1: bool) -> String {
        // Create mutable stacks to work with
        let mut work_stacks: Vec<Vec<char>> = Vec::new();
        let mut temp_stack: Vec<char> = Vec::new();

        // Initialize those stacks with contents from puzzle input
        for stack in &self.stacks {
            let mut work_stack = Vec::new();

            for c in stack {
                work_stack.push(*c);
            }
            work_stacks.push(work_stack);
        }

        // Execute the list of moves
        for m in &self.moves {
            if part1 {
                // move one crate at a time
                for _ in 0..m.count {
                    let c = work_stacks[m.from - 1].pop().unwrap();
                    work_stacks[m.to - 1].push(c);
                }
            }
            else {
                // move whole sub-stacks of crates
                for _ in 0..m.count {
                    let c = work_stacks[m.from - 1].pop().unwrap();
                    temp_stack.push(c);
                }
                for _ in 0..m.count {
                    let c = temp_stack.pop().unwrap();
                    work_stacks[m.to-1].push(c);
                }
            }
        }

        // Construct a string from the tops of the stacks
        let mut s = String::new();
        for work_stack in work_stacks {
            s.push(*work_stack.last().unwrap());
        }

        s
    }
}

impl Day for Day5 {
    fn part1(&self) -> Answer {
        // Ok(1)
        Answer::Message(self.process_moves(true))
    }

    fn part2(&self) -> Answer {
        // Ok(2)
        Answer::Message(self.process_moves(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day5::load("examples/day5_example1.txt");
        assert_eq!(d.stacks.len(), 3);
        assert_eq!(d.stacks[0].len(), 2);
        assert_eq!(d.stacks[1].len(), 3);
        assert_eq!(d.stacks[2].len(), 1);
        assert_eq!(d.moves.len(), 4);
    }

    #[test]
    fn test_process_moves() {
        let d = Day5::load("examples/day5_example1.txt");
        let s = d.process_moves(true);
        assert_eq!(s, "CMZ");
    }

    #[test]
    fn test_process_moves2() {
        let d = Day5::load("examples/day5_example1.txt");
        let s = d.process_moves(false);
        assert_eq!(s, "MCD");
    }
}
