use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day1 {
    // elves[n] is a vector of calorie values, one per item carried by that elf.
    elves: Vec<Vec<usize>>,
}

impl Day1 {
    pub fn load(filename: &str) -> Day1 {
        let mut elves: Vec<Vec<usize>> = Vec::new();
        let mut items: Vec<usize> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let calories = l.parse::<usize>();
            match calories {
                Err(_) => {
                    // blank line : previous items vector is done
                    elves.push(items);
                    items = Vec::new();
                }
                Ok(value) => {
                    // we have a new item
                    items.push(value);
                }
            }
        }

        // push last list of items if non-empty
        if items.len() > 0 {
            elves.push(items);
        }

        Day1 { elves }
    }

    fn max_cals(&self) -> usize {
        let mut max = 0;
        for elf in &self.elves {
            let mut count = 0;
            for calories in elf {
                count += calories;
            }

            if count > max {
                max = count;
            }
        }

        max
    }

    fn max3_cals(&self) -> usize {
        let mut totals: Vec<usize> = Vec::new();

        for elf in &self.elves {
            let mut total: usize = 0;
            for item in elf {
                total += item;
            }
            totals.push(total);
        }
        totals.sort();
        totals.reverse();

        let mut sum3 = 0;

        sum3 += totals[0];
        sum3 += totals[1];
        sum3 += totals[2];

        sum3
    }
}

impl Day for Day1 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.max_cals())
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.max3_cals())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day1::load("examples/day1_example1.txt");
        assert_eq!(d.elves.len(), 5);
    }

    #[test]
    fn test_max() {
        let d = Day1::load("examples/day1_example1.txt");
        assert_eq!(d.max_cals(), 24000);
    }

    #[test]
    fn test_max3() {
        let d = Day1::load("examples/day1_example1.txt");
        assert_eq!(d.max3_cals(), 45000);
    }
}
