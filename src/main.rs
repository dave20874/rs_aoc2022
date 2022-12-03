#[macro_use]

mod day;
mod day1;
mod day2;

use day::Day;
use day1::Day1;
use day2::Day2;


fn do_day(n: usize, day: &dyn Day) {
    match day.part1() {
        Ok(val) => println!("day {}, part 1: {}", n, val),
        Err(msg) => println!("day {}, part 1: {}", n, msg),
    }
    match day.part2() {
        Ok(val) => println!("day {}, part 2: {}", n, val),
        Err(msg) => println!("day {}, part 2: {}", n, msg),
    }
}

fn main() {
    println!("Advent of Code 2022!");
    println!("See adventofcode.com/2022 for background.");
    println!("");

    let day1 = Day1::load("data_aoc2022/day1_input.txt");
    let day2: Day2 = Day2::load("data_aoc2022/day2_input.txt");

    let days: Vec<&dyn Day> = vec![
        &day1, 
        &day2,
    ];

    let selected_day: Option<usize> = None;
    match selected_day {
        None => {
            // No day selected, do them all
            for (n, day) in days.iter().enumerate() {
                do_day(n + 1, *day);
            }
        }
        Some(n) => {
            do_day(n - 1, days[n - 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test results will be different for each participant.
    #[test]
    fn test_day1() {
        let d = Day1::load("data_aoc2022/day1_input.txt");
        assert_eq!(d.part1(), Ok(71780));
        assert_eq!(d.part2(), Ok(212489));
    }

    #[test]
    fn test_day2() {
        let d = Day2::load("data_aoc2022/day2_input.txt");
        assert_eq!(d.part1(), Ok(1));
        assert_eq!(d.part2(), Ok(2));
    }
}

