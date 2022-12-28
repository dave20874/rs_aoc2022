use crate::day::{Day, Answer};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    dir: Dir,
    dist: usize,
}

struct Sim {
    // knot[0] is head, knot[num_knots-1] is tail.
    // positions are represented as (x, y) tuples
    num_knots: usize,
    knots: Vec<(isize, isize)>,
    // head and tail are represented as (x, y) tuple.
    // head: (isize, isize),
    // tail: (isize, isize),
    tail_visited: HashSet<(isize, isize)>,
}

impl Sim {
    fn new(num_knots: usize) -> Sim {

        let mut knots: Vec<(isize, isize)> = Vec::new();
        for _n in 0..num_knots {
            knots.push( (0, 0) );
        }

        let mut tail_visited = HashSet::new();
        tail_visited.insert(knots[num_knots-1]);

        Sim { num_knots, knots, tail_visited }
    }

    #[cfg(test)]
    fn with_head(num_knots: usize, head: (isize, isize)) -> Sim {

        let mut knots: Vec<(isize, isize)> = Vec::new();
        knots.push(head);
        for _n in 1..num_knots {
            knots.push( (0, 0) );
        }

        let mut tail_visited = HashSet::new();
        tail_visited.insert(knots[num_knots-1]);

        Sim { num_knots, knots, tail_visited }
    }

    fn delta_for_difference(difference: (isize, isize)) -> (isize, isize) {
        let mut delta = (0, 0);

        if (difference.0 == 0) || (difference.1 == 0) {
            // they are in a row or col.  Move horizontally or vertically to keep up.

            if difference.0 > 1 {
                delta.0 = 1;
            }
            else if difference.0 < -1 {
                delta.0 = -1;
            }
            else if difference.1 > 1 {
                delta.1 = 1;
            }
            else if difference.1 < -1 {
                delta.1 = -1;
            }
        }
        else {
            // they are not in same row or col.  Move diagonally.
            if difference.0 > 1 {
                delta.0 = 1;
                if difference.1 > 0 {
                    delta.1 = 1;
                }
                else {
                    delta.1 = -1;
                }
            }
            else if difference.0 < -1 {
                delta.0 = -1;
                if difference.1 > 0 {
                    delta.1 = 1;
                }
                else {
                    delta.1 = -1;
                }
            }
            else if difference.1 > 1 {
                delta.1 = 1;
                if difference.0 > 0 {
                    delta.0 = 1;
                }
                else {
                    delta.0 = -1;
                }
            }
            else if difference.1 < -1 {
                delta.1 = -1;
                if difference.0 > 0 {
                    delta.0 = 1;
                }
                else {
                    delta.0 = -1;
                }
            }
        }

        delta
    }

    // for debugging.  Don't complain when not in use.
    #[allow(dead_code)]
    fn show_rope(&self) {
        // get extents
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for i in 0..self.num_knots {
            let x = self.knots[i].0;
            let y = self.knots[i].1;
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        // print full extents
        for y in (min_y ..= max_y).rev() {
            for x in min_x ..= max_x {
                let mut printed = false;
                for i in 0..self.num_knots {
                    if self.knots[i] == (x, y) {
                        print!("{}", i);
                        printed = true;
                        break;
                    }
                }
                if !printed {
                    if (x,y) == (0, 0) {
                        print!("s");
                    }
                    else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!("---------------------------------------------------");
    }
    
    fn do_move(&mut self, dir: &Dir) {

        // Update the head knot
        let delta = match dir {
            Dir::Right => (1, 0),
            Dir::Left => (-1, 0),
            Dir::Up => (0, 1),
            Dir::Down => (0, -1)
        };
        self.knots[0].0 += delta.0;
        self.knots[0].1 += delta.1;

        // Update followers
        for knot in 1..self.num_knots {
            let difference = 
                (self.knots[knot-1].0 - self.knots[knot].0, 
                 self.knots[knot-1].1 - self.knots[knot].1);
            
            let delta = Sim::delta_for_difference(difference);
            self.knots[knot].0 += delta.0;
            self.knots[knot].1 += delta.1;
        }

        // Add tail's new coordinate to tail_visited
        self.tail_visited.insert(self.knots[self.num_knots-1]);

        // self.show_rope();
    }


    fn do_instruction(&mut self, instruction: &Instruction) {
        for _n in 0..instruction.dist {
            self.do_move(&instruction.dir);
        }

        // self.show_rope();
    }

    fn do_instructions(&mut self, instructions: &Vec<Instruction>) {
        for i in instructions {
            self.do_instruction(i);
        }
    }

    // fn get_head(&self) -> (isize, isize) {
    //     self.knots[0]
    // }
    #[cfg(test)]
    fn get_tail(&self) -> (isize, isize) {
        self.knots[self.num_knots-1]
    }

    fn get_num_tail_positions(&self) -> usize {
        self.tail_visited.len()
    }
}

pub struct Day9 {
    instructions: Vec<Instruction>,
}

impl Day9 {
    pub fn load(filename: &str) -> Day9 {
        let mut instructions: Vec<Instruction> = Vec::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([UDLR]) ([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let dir = match &caps[1] {
                        "U" => Dir::Up,
                        "D" => Dir::Down,
                        "L" => Dir::Left,
                        "R" => Dir::Right,
                        _ => panic!("Invalid direction encountered.")
                    };
                    let dist = caps[2].parse::<usize>().unwrap();
                    instructions.push(Instruction {dir, dist});
                }
                None => {}
            }

        }

        Day9 { instructions }
    }
}

impl Day for Day9 {
    fn part1(&self) -> Answer {
        let mut sim= Sim::new(2);
        sim.do_instructions(&self.instructions);

        Answer::Number(sim.get_num_tail_positions())
    }

    fn part2(&self) -> Answer {
        let mut sim = Sim::new(10);
        sim.do_instructions(&self.instructions);

        Answer::Number(sim.get_num_tail_positions())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day9::load("examples/day9_example1.txt");
        assert_eq!(d.instructions.len(), 8);
    }

    #[test]
    fn test_sim_move_right() {
        let mut sim = Sim::with_head(2, (1, 0));
        sim.do_instruction(&Instruction {dir: Dir::Right, dist: 1});
        assert_eq!(sim.get_tail(), (1, 0));
        assert_eq!(sim.get_num_tail_positions(), 2);
    }

    #[test]
    fn test_sim_move_down() {
        let mut sim = Sim::with_head(2, (0, -1));
        sim.do_instruction(&Instruction {dir: Dir::Down, dist: 1});
        assert_eq!(sim.get_tail(), (0, -1));
        assert_eq!(sim.get_num_tail_positions(), 2);
    }

    #[test]
    fn test_sim_move_up() {
        let mut sim = Sim::with_head(2, (1, 1));
        sim.do_instruction(&Instruction {dir: Dir::Up, dist: 1});
        assert_eq!(sim.get_tail(), (1, 1));
        assert_eq!(sim.get_num_tail_positions(), 2);
    }

    #[test]
    fn test_sim_move_right_diag() {
        let mut sim = Sim::with_head(2, (1, 1));
        sim.do_instruction(&Instruction {dir: Dir::Right, dist: 1});
        assert_eq!(sim.get_tail(), (1, 1));
        assert_eq!(sim.get_num_tail_positions(), 2);
    }

    #[test]
    fn test_example1() {
        let d = Day9::load("examples/day9_example1.txt");

        let mut sim = Sim::new(2);
        sim.do_instructions(&d.instructions);

        assert_eq!(sim.get_num_tail_positions(), 13);
    }

    #[test]
    fn test_example1_part1() {
        let d = Day9::load("examples/day9_example1.txt");

        assert_eq!(d.part1(), Answer::Number(13));
    }

    #[test]
    fn test_example1_part2() {
        let d = Day9::load("examples/day9_example1.txt");

        let mut sim = Sim::new(10);
        sim.do_instructions(&d.instructions);

        assert_eq!(sim.get_num_tail_positions(), 1);
    }

    #[test]
    fn test_example2_part2() {
        let d = Day9::load("examples/day9_example2.txt");

        let mut sim = Sim::new(10);
        sim.do_instructions(&d.instructions);

        assert_eq!(sim.get_num_tail_positions(), 36);
    }
}
