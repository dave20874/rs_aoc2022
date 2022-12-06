extern crate core;

#[macro_use]

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use day::{Day, Answer};
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;


fn do_day(n: usize, day: &dyn Day) {
    match day.part1() {
        Answer::Number(val) => println!("day {}, part 1: {}", n, val),
        Answer::Message(s) => println!("day {}, part 1: '{}'", n, s),
    }
    match day.part2() {
        Answer::Number(val) => println!("day {}, part 2: {}", n, val),
        Answer::Message(s) => println!("day {}, part 2: '{}'", n, s),
    }
}

fn main() {
    println!("Advent of Code 2022!");
    println!("See adventofcode.com/2022 for background.");
    println!("");

    let day1 = Day1::load("data_aoc2022/day1_input.txt");
    let day2 = Day2::load("data_aoc2022/day2_input.txt");
    let day3 = Day3::load("data_aoc2022/day3_input.txt");
    let day4 = Day4::load("data_aoc2022/day4_input.txt");
    let day5 = Day5::load("data_aoc2022/day5_input.txt");   

    let days: Vec<&dyn Day> = vec![
        &day1, 
        &day2,
        &day3,
        &day4,
        &day5,
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
        assert_eq!(d.part1(), Answer::Number(71780));
        assert_eq!(d.part2(), Answer::Number(212489));
    }

    #[test]
    fn test_day2() {
        let d = Day2::load("data_aoc2022/day2_input.txt");
        assert_eq!(d.part1(), Answer::Number(13565));
        assert_eq!(d.part2(), Answer::Number(12424));
    }

    #[test]
    fn test_day3() {
        let d = Day3::load("data_aoc2022/day3_input.txt");
        assert_eq!(d.part1(), Answer::Number(8153));
        assert_eq!(d.part2(), Answer::Number(2342));
    }

    #[test]
    fn test_day4() {
        let d = Day4::load("data_aoc2022/day4_input.txt");
        assert_eq!(d.part1(), Answer::Number(459));
        assert_eq!(d.part2(), Answer::Number(779));
    }

    #[test]
    fn test_day5() {
        let d = Day5::load("data_aoc2022/day5_input.txt");
        assert_eq!(d.part1(), Answer::Message("SHMSDGZVC".to_string()));
        assert_eq!(d.part2(), Answer::Message("VRZGHDFBQ".to_string()));
    }
}

