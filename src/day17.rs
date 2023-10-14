use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Rock {
    Rock1,
    Rock2,
    Rock3,
    Rock4,
    Rock5,
}

// Rock shapes are represented as U32, with the four bytes representing four potential layers of height.
// In each byte, the MSB is unused (as the chamber is only 7 units wide.)
// Rock shapes are defined here with two open spaces to the left of the left-most rock edge so that
// these are in the rock's initial position when falling.
// Rock 1:
// ####
const ROCK1_OCCUPANCY: u32 = 0b_00000000_00000000_00000000_00011110;

// Rock 2:
// .#.
// ###
// .#.
const ROCK2_OCCUPANCY: u32 = 0b_00000000_00001000_00011100_00001000;

// Rock 3:
// ..#
// ..#
// ###
const ROCK3_OCCUPANCY: u32 = 0b_00000000_00000100_00000100_00011100;

// Rock 4 :
// #
// #
// #
// #
const ROCK4_OCCUPANCY: u32 = 0b_00010000_00010000_00010000_00010000;

// Rock 5 :
// ##
// ##
const ROCK5_OCCUPANCY: u32 = 0b_00000000_00000000_00011000_00011000;

// Test for a rock against leftmost edge
const CHAMBER_LEFTMOST: u32  = 0b_01000000_01000000_01000000_01000000;
const CHAMBER_RIGHTMOST: u32 = 0b_00000001_00000001_00000001_00000001;

const ROCK1_HEIGHT: usize = 1;
const ROCK2_HEIGHT: usize = 3;
const ROCK3_HEIGHT: usize = 3;
const ROCK4_HEIGHT: usize = 4;
const ROCK5_HEIGHT: usize = 2;

impl Rock {
    fn get_occupancy(&self) -> u32 {
        match &self {
            Rock::Rock1 => { ROCK1_OCCUPANCY },
            Rock::Rock2 => { ROCK2_OCCUPANCY },
            Rock::Rock3 => { ROCK3_OCCUPANCY },
            Rock::Rock4 => { ROCK4_OCCUPANCY },
            Rock::Rock5 => { ROCK5_OCCUPANCY },
        }
    }

    fn get_height(&self) -> usize {
        match &self {
            Rock::Rock1 => ROCK1_HEIGHT,
            Rock::Rock2 => ROCK2_HEIGHT,
            Rock::Rock3 => ROCK3_HEIGHT,
            Rock::Rock4 => ROCK4_HEIGHT,
            Rock::Rock5 => ROCK5_HEIGHT,
        }
    }

    fn next(&self) -> Rock {
        match &self {
            Rock::Rock1 => Rock::Rock2,
            Rock::Rock2 => Rock::Rock3,
            Rock::Rock3 => Rock::Rock4,
            Rock::Rock4 => Rock::Rock5,
            Rock::Rock5 => Rock::Rock1,
        }
    }
}

struct Chamber {
    occupied: Vec<u8>,
    height: usize,
}

impl Chamber {
    fn new() -> Chamber {
        let occupied: Vec<u8> = Vec::new();
        
        Chamber { occupied, height: 0 }
    }
}

struct Sim {
    time: usize,
    wind_index: usize,
    wind_vec: Vec<bool>,  // true is wind to the left.
    next_rock: Rock,

    chamber: Chamber,
}

impl Sim {
    fn new(wind_pattern: &str) -> Sim {
        let chamber = Chamber::new();
        let mut wind_vec: Vec<bool> = Vec::new();

        for c in wind_pattern.chars() {
            if c == '<' {
                wind_vec.push(true);
            }
            else {
                wind_vec.push(false);
            }
        }

        Sim { 
            time: 0, 
            wind_index: 0,
            wind_vec, 
            next_rock: Rock::Rock1, 
            chamber
        }
    }

    fn drop_rock(&mut self) {
        // start rock with left edge two spaces from wall, three steps above top of chamber.
        let mut rock = self.next_rock.get_occupancy();
        let mut obstacles: u32 = 0;
        let mut level = self.chamber.height + 3;

        // println!("Dropping {:?}", self.next_rock);

        let mut blocked = false;
        while !blocked {
            // shift rock left or right, if possible
            if self.wind_vec[self.wind_index] {
                if (rock & CHAMBER_LEFTMOST) == 0 {
                    if (rock << 1) & obstacles == 0 {
                        rock = rock << 1;
                    }
                }
            }
            else {
                if (rock & CHAMBER_RIGHTMOST) == 0 {
                    if (rock >> 1) & obstacles == 0 {
                        rock = rock >> 1;
                    }
                }
            }

            // update time
            self.time += 1;
            self.wind_index += 1;
            if self.wind_index >= self.wind_vec.len() {
                self.wind_index = 0;
            }

            // println!("{:?}", position);

            // Drop down one level if possible.
            // println!("Moving down");
            if level > 0 {
                let new_level = level-1;
                obstacles <<= 8;
                if new_level < self.chamber.occupied.len() {
                    obstacles |= self.chamber.occupied[new_level] as u32;
                }

                if rock & obstacles != 0 {
                    // The rock hit an obstacle moving down.
                    blocked = true;
                }
                else {
                    // no interference, adopt the new level
                    level = new_level;
                }
            }
            else {
                // hit the bottom of the chamber
                blocked = true;
            }
        }

        // Solidify the block in its rest position
        // println!("solidifying at {:?}", position)
        let mut rock_cross_section = rock;
        for h in level..level+4 {
            let plane: u8 = (rock_cross_section & 0xFF) as u8;
            if h >= self.chamber.occupied.len() {
                self.chamber.occupied.push(plane);
            }
            else {
                self.chamber.occupied[h] |= plane;
            }
            rock_cross_section >>= 8;
        }

        // Update the recorded height of the chamber
        if level + self.next_rock.get_height() > self.chamber.height {
            self.chamber.height = level + self.next_rock.get_height();
        }

        self.next_rock = self.next_rock.next();
    }
}

pub struct Day17 {
    winds: String,
}

impl Day17 {
    pub fn load(filename: &str) -> Day17 {
        let mut winds: String = String::new();

        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        reader.read_line(&mut winds).unwrap();

        Day17 { winds: winds.trim().to_string() }
    }
}

impl Day for Day17 {
    fn part1(&self) -> Answer {
        let mut sim = Sim::new(&self.winds);

        for _count in 0..2022 {
            sim.drop_rock();
        }

        Answer::Number(sim.chamber.height)
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
        let d = Day17::load("examples/day17_example1.txt");
        assert_eq!(d.winds.len(), 40);
    }

    #[test]
    fn test_rocks() {
        assert_eq!(Rock::Rock1.get_occupancy(), ROCK1_OCCUPANCY);
        assert_eq!(Rock::Rock1.get_height(), 1);

        assert_eq!(Rock::Rock2.get_occupancy(), ROCK2_OCCUPANCY);
        assert_eq!(Rock::Rock2.get_height(), 3);

        assert_eq!(Rock::Rock3.get_occupancy(), ROCK3_OCCUPANCY);
        assert_eq!(Rock::Rock3.get_height(), 3);

        assert_eq!(Rock::Rock4.get_occupancy(), ROCK4_OCCUPANCY);
        assert_eq!(Rock::Rock4.get_height(), 4);

        assert_eq!(Rock::Rock5.get_occupancy(), ROCK5_OCCUPANCY);
        assert_eq!(Rock::Rock5.get_height(), 2);
    }

    #[test]
    fn test_new_sim() {
        let sim = Sim::new("<<>><<>>");

        assert_eq!(0, sim.chamber.occupied.len());
        assert_eq!(0, sim.chamber.height);
        assert_eq!(Rock::Rock1, sim.next_rock);
        assert_eq!(8, sim.wind_vec.len());
        assert_eq!(0, sim.time);
    }

    #[test]
    fn test_drop_rock1() {
        let d = Day17::load("examples/day17_example1.txt");
        let mut sim = Sim::new(&d.winds);

        sim.drop_rock();

        assert_eq!(sim.chamber.height, 1);
        assert_eq!(sim.time, 4);
    }

    #[test]
    fn test_drop_rock2() {
        let d = Day17::load("examples/day17_example1.txt");
        let mut sim = Sim::new(&d.winds);

        sim.drop_rock();
        sim.drop_rock();

        assert_eq!(sim.chamber.height, 4);
        assert_eq!(sim.time, 8);
    }

    #[test]
    fn test_drop_rocks_10() {
        let d = Day17::load("examples/day17_example1.txt");
        let mut sim = Sim::new(&d.winds);

        for _count in 0..10 {
            sim.drop_rock();
        }

        assert_eq!(sim.chamber.height, 17);
    }

    #[test]
    fn test_drop_rocks_2022() {
        let d = Day17::load("examples/day17_example1.txt");
        let mut sim = Sim::new(&d.winds);

        for _count in 0..2022 {
            sim.drop_rock();
        }

        assert_eq!(sim.chamber.height, 3068);
    }


    #[test]
    fn test_drop_rocks_1b() {
        let d = Day17::load("examples/day17_example1.txt");
        let mut sim = Sim::new(&d.winds);

        for _count in 0..1000000000 {
            sim.drop_rock();
        }

        assert_eq!(sim.chamber.height, 3068);
    }

/*
    #[test]
    fn test_input_len() {
        let d = Day17::load("data_aoc2022/day17_input.txt");

        assert_eq!(d.winds.len(), 10);
    }

    #[test]
    fn test_periodicity() {
        let d = Day17::load("data_aoc2022/day17_input.txt");
        let period = d.winds.len() * 5;
        let mut sim = Sim::new(&d.winds);

        let mut h = 0;
        for periods in 0..100 {
            for _n in 0..period {
                sim.drop_rock();
            }
            let prev_h = h;
            h = sim.chamber.height; 
            println!("After {:?} periods, height is {:?}, change: {:?}", periods, h, h-prev_h);
        }

        assert!(true);
    }
    */
}
