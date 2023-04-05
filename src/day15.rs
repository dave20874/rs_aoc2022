use std::collections::HashSet;
use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

struct SensorBeacon {
    sx: isize,
    sy: isize,
    bx: isize,
    by: isize,
}

pub struct Day15 {
    sensor_beacons: Vec<SensorBeacon>,
}

impl Day15 {
    pub fn load(filename: &str) -> Day15 {
        let mut sensor_beacons: Vec<SensorBeacon> = Vec::new();
        lazy_static! {
            static ref SENSOR_BEACON_RE: Regex =
                Regex::new("Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = SENSOR_BEACON_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let sx = caps[1].parse::<isize>().unwrap();
                    let sy = caps[2].parse::<isize>().unwrap();
                    let bx = caps[3].parse::<isize>().unwrap();
                    let by = caps[4].parse::<isize>().unwrap();
                    let sb = SensorBeacon { sx, sy, bx, by };
                    sensor_beacons.push(sb);
                }
                None => {
                    // println!("Nope: '{}'", l);
                }
            }
        }

        Day15 { sensor_beacons }
    }

    pub fn not_on_line(&self, y: isize) -> usize {
        // for reference, we need a hashmap of all beacons on the line of interest
        let mut beacons_on_line: HashSet<isize> = HashSet::new();
        for sb in &self.sensor_beacons {
            if sb.by == y {
                beacons_on_line.insert(sb.bx);
            }
        }

        // line of interest has an entry for every x value where (x,y) is inside coverage
        // area of some sensor/beacon pair.
        let mut line_of_interest: HashSet<isize> = HashSet::new();
        for sb in &self.sensor_beacons {
            // get manhattan distance for this sensor/beacon pair.
            let dist = (sb.sx-sb.bx).abs() + (sb.sy-sb.by).abs();
            // extra_dist is < 0 if this sensor/beacon pair doesn't intersect y
            let extra_dist = dist - (y - sb.sy).abs();
            if extra_dist >= 0 {
                for dx in 0..=extra_dist {
                    let x = sb.sx - dx;
                    if !beacons_on_line.contains(&x) {
                        line_of_interest.insert(x);
                    }
                    let x = sb.sx + dx;
                    if !beacons_on_line.contains(&x) {
                        line_of_interest.insert(x);
                    }
                }
            }
        }

        // count how many points we identified
        line_of_interest.len()
    }

    fn is_covered(&self, x: isize, y: isize) -> bool {
        // determine if x, y is covered by some sensor/beacon pair
        // For each pair, see if (x,y) is at a further hamming distance from the sensor than its beacon.
        let mut covered = false;
        for sb in &self.sensor_beacons {
            let covered_dist = (sb.sx-sb.bx).abs() + (sb.sy-sb.by).abs();
            let test_dist = (sb.sx-x).abs() + (sb.sy-y).abs();
            if covered_dist >= test_dist {
                covered = true;
                break;
            }
        }

        covered
    }

    fn find_uncovered(&self, min: isize, max: isize) -> Option<(isize, isize)> {
        let mut found_x = 0;
        let mut found_y = 0;
        let mut found = false;

        // search beyond the perimeter of each sensor/beacon pair's covered area.
        'outer: for sb in &self.sensor_beacons {
            let hamming_d = (sb.sx-sb.bx).abs() + (sb.sy-sb.by).abs();
            for dx in 0..=hamming_d+1 {
                let dy = hamming_d+1-dx;
                for (coord_x, coord_y) in [(sb.sx+dx, sb.sy+dy), (sb.sx+dx, sb.sy-dy),
                                                      (sb.sx-dx, sb.sy+dy), (sb.sx-dx, sb.sy-dy)] {
                    if coord_x < min || coord_y < min || coord_x > max || coord_y > max {
                        continue;
                    }
                    if !self.is_covered(coord_x, coord_y) {
                        // found an uncovered cell
                        found_x = coord_x;
                        found_y = coord_y;
                        found = true;
                        break 'outer;
                    }
                }
            }
        }

        if found {
            // println!("found {}, {}!", found_x, found_y);
            Some((found_x, found_y))
        }
        else {
            None
        }
    }
}

impl Day for Day15 {
    fn part1(&self) -> Answer {
        Answer::Number(self.not_on_line(2000000))
    }

    fn part2(&self) -> Answer {
        match self.find_uncovered(0, 4000000) {
            Some((x, y)) => {
                let tuning_frequency = x*4000000+y;
                Answer::Number(tuning_frequency as usize)
            }
            None => {
                Answer::Number(0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day15::load("examples/day15_example1.txt");
        assert_eq!(d.sensor_beacons.len(), 14);
    }

    #[test]
    fn test_not_on_line() {
        let d = Day15::load("examples/day15_example1.txt");
        let eliminated = d.not_on_line(10);
        assert_eq!(eliminated, 26);
    }

    #[test]
    fn test_part1() {
        let d = Day15::load("examples/day15_example1.txt");
        assert_eq!(d.part1(), Answer::Number(0));
    }

    #[test]
    fn test_is_covered() {
        let d = Day15::load("examples/day15_example1.txt");
        assert!(d.is_covered(1, 1));
        assert!(!d.is_covered(14, 11));
    }

    #[test]
    fn test_find_uncovered() {
        let d = Day15::load("examples/day15_example1.txt");
        let uncovered = d.find_uncovered(0, 20);
        assert_eq!(uncovered, Some((14, 11)) );
    }
}
