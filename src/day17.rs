use crate::day::{Day, Answer};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
enum Rock {
    Rock1,
    Rock2,
    Rock3,
    Rock4,
    Rock5,
}



lazy_static! {
    // Rock 1 : 
    // ####
    static ref ROCK1_OCCUPANCY: HashSet<(isize, isize)> = {
        let s = HashSet::from_iter(vec! { 
            (0, 0), (1, 0), (2, 0), (3, 0) 
        });

        s
    };

    // Rock 2 :
    // .#.
    // ###
    // .#.
    static ref ROCK2_OCCUPANCY: HashSet<(isize, isize)> = {
        let s = HashSet::from_iter(vec! { 
            (1, 0), (0, 1), (1, 1), (2, 1), (1, 2) 
        });
        s
    };

    // Rock 3 :
    // ..#
    // ..#
    // ###
    static ref ROCK3_OCCUPANCY: HashSet<(isize, isize)> = {
        let s = HashSet::from_iter(vec! { 
            (0, 0), (1, 0), (2, 0), (2, 1), (2, 2)
        });

        s
    };

    // Rock 4 :
    // #
    // #
    // #
    // #
    static ref ROCK4_OCCUPANCY: HashSet<(isize, isize)> = {
        let s = HashSet::from_iter(vec! { 
            (0, 0), (0, 1), (0, 2), (0, 3) 
        });

        s
    };

    // Rock 5 :
    // ##
    // ##
    static ref ROCK5_OCCUPANCY: HashSet<(isize, isize)> = {
        let s = HashSet::from_iter(vec! { 
            (0, 0), (1, 0), (0, 1), (1, 1) 
        });

        s
    };
}

const ROCK1_HEIGHT: usize = 1;
const ROCK2_HEIGHT: usize = 3;
const ROCK3_HEIGHT: usize = 3;
const ROCK4_HEIGHT: usize = 4;
const ROCK5_HEIGHT: usize = 2;

impl Rock {
    fn get_occupancy(&self) -> &HashSet<(isize, isize)> {
        match &self {
            Rock::Rock1 => { &ROCK1_OCCUPANCY },
            Rock::Rock2 => { &ROCK2_OCCUPANCY },
            Rock::Rock3 => { &ROCK3_OCCUPANCY },
            Rock::Rock4 => { &ROCK4_OCCUPANCY },
            Rock::Rock5 => { &ROCK5_OCCUPANCY },
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
}

struct Chamber {
    occupied: HashSet<(isize, isize)>,
    height: usize,
}

impl Chamber {
    fn new() -> Chamber {
        let occupied: HashSet<(isize, isize)> = HashSet::new();
        
        Chamber { occupied, height: 0 }
    }
}

struct Sim {
    time: usize,
    wind_index: usize,
    wind_pattern: String,
    next_rock: Rock,

    chamber: Chamber,
}

impl Sim {
    fn new(wind_pattern: &str) -> Sim {
        let chamber = Chamber::new();

        Sim { 
            time: 0, 
            wind_index: 0,
            wind_pattern: wind_pattern.to_string(), 
            next_rock: Rock::Rock1, 
            chamber
        }
    }

    fn drop_rock(&mut self) {
        // start rock with left edge two spaces from wall, three steps above top of chamber.
        let mut position: (isize, isize) = (2, self.chamber.height+3);
        let mut blocked = false;

        while !blocked {
            // get new position from wind shift
            let mut new_position = position;
            if self.wind_pattern.get_c(self.wind_index) == '<' { 
                new_position.0 -= 1;
            }
            else {
                new_position.0 += 1;
            };
            self.time += 1;
            self.wind_index += 1;
            if self.wind_index >= self.wind_pattern.len() {
                self.wind_index = 0;
            }

            // check for interference from wall or other blocks
            let mut interfered = false;
            for (x, y) in self.next_rock.get_occupancy() {
                let component_pos = (new_position.0+x, new_position.1+y);
                if (component.x < 0) || 
                    (component.x >= 7) ||
                    (self.chamber.contains(component_pos)) {
                        interfered = true;
                }
            }
            
            // If no interference, adopt the shifted position, otherwise don't shift.
            if interfered {
                new_position = position;
            }

            // get new position from dropping one level
            new_position.1 -= 1;

            // check for interference from bottom or other blocks
            let mut interfered = false;
            for (x, y) in self.next_rock.get_occupancy() {
                let component_pos = (new_position.0+x, new_position.1+y);
                if (component.y < 0) || 
                    (self.chamber.contains(component_pos)) {
                        interfered = true;
                }
            }
            if interfered {
                // Undo the drop
                new_position = position;

                // Solidify in this position

                blocked = true;
            }
            else {
                // adopt the dropped position and repeat this loop
                position = new_position;
            }
        }

        // Solidify the block in its rest position
        // TODO

        // Update the recorded height of the chamber
        // TODO
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
        Answer::Number(1)
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
        assert_eq!(Rock::Rock1.get_occupancy().len(), 4);
        assert_eq!(Rock::Rock1.get_height(), 1);

        assert_eq!(Rock::Rock2.get_occupancy().len(), 5);
        assert_eq!(Rock::Rock2.get_height(), 3);

        assert_eq!(Rock::Rock3.get_occupancy().len(), 5);
        assert_eq!(Rock::Rock3.get_height(), 3);

        assert_eq!(Rock::Rock4.get_occupancy().len(), 4);
        assert_eq!(Rock::Rock4.get_height(), 4);

        assert_eq!(Rock::Rock5.get_occupancy().len(), 4);
        assert_eq!(Rock::Rock5.get_height(), 2);
    }

    #[test]
    fn test_new_sim() {
        let sim = Sim::new("<<>><<>>");

        assert_eq!(0, sim.chamber.occupied.len());
        assert_eq!(0, sim.chamber.height);
        assert_eq!(Rock::Rock1, sim.next_rock);
        assert_eq!(8, sim.wind_pattern.len());
        assert_eq!(0, sim.time);
    }

    #[test]
    fn test_drop_rock1() {
        let d = Day17::load("examples/day17_example1.txt");
        let sim = Sim::new(&d.winds);

        sim.drop_rock();

        assert_eq!(sim.chamber.height, 1);
    }
}
