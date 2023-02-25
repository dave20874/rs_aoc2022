extern crate core;

#[macro_use]

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

use day::{Day, Answer};
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;

fn do_day(n: usize, day: &dyn Day) {
    match day.part1() {
        Answer::Number(val) => println!("day {}, part 1: {}", n, val),
        Answer::Message(s) => println!("day {}, part 1: \n{}", n, s),
    }
    match day.part2() {
        Answer::Number(val) => println!("day {}, part 2: {}", n, val),
        Answer::Message(s) => println!("day {}, part 2: \n{}", n, s),
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
    let day6 = Day6::load("data_aoc2022/day6_input.txt");
    let day7 = Day7::load("data_aoc2022/day7_input.txt");
    let day8 = Day8::load("data_aoc2022/day8_input.txt");
    let day9 = Day9::load("data_aoc2022/day9_input.txt");
    let day10 = Day10::load("data_aoc2022/day10_input.txt");
    let day11 = Day11::load("data_aoc2022/day11_input.txt");
    let day12 = Day12::load("data_aoc2022/day12_input.txt");
    let day13 = Day13::load("data_aoc2022/day13_input.txt");

    let days: Vec<&dyn Day> = vec![
        &day1, 
        &day2,
        &day3,
        &day4,
        &day5,
        &day6,
        &day7,
        &day8,
        &day9,
        &day10,
        &day11,
        &day12,
        &day13,
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

    #[test]
    fn test_day6() {
        let d = Day6::load("data_aoc2022/day6_input.txt");
        assert_eq!(d.part1(), Answer::Number(1287));
        assert_eq!(d.part2(), Answer::Number(3716));
    }

    #[test]
    fn test_day7() {
        let d = Day7::load("data_aoc2022/day7_input.txt");
        assert_eq!(d.part1(), Answer::Number(1667443));
        assert_eq!(d.part2(), Answer::Number(8998590));
    }

    #[test]
    fn test_day8() {
        let d = Day8::load("data_aoc2022/day8_input.txt");
        assert_eq!(d.part1(), Answer::Number(1787));
        assert_eq!(d.part2(), Answer::Number(440640));
    }

    #[test]
    fn test_day9() {
        let d = Day9::load("data_aoc2022/day9_input.txt");
        assert_eq!(d.part1(), Answer::Number(6090));
        assert_eq!(d.part2(), Answer::Number(2566));
    }

    #[test]
    fn test_day10() {
        let d = Day10::load("data_aoc2022/day10_input.txt");
        let s =
            "####.###...##..###..####.###...##....##.\n\
             #....#..#.#..#.#..#.#....#..#.#..#....#.\n\
             ###..#..#.#....#..#.###..#..#.#.......#.\n\
             #....###..#....###..#....###..#.......#.\n\
             #....#.#..#..#.#.#..#....#....#..#.#..#.\n\
             ####.#..#..##..#..#.####.#.....##...##..\n".to_string();
        assert_eq!(d.part1(), Answer::Number(11720));
        assert_eq!(d.part2(), Answer::Message(s));
    }

    #[test]
    fn test_day11() {
        let d = Day11::load("data_aoc2022/day11_input.txt");
        assert_eq!(d.part1(), Answer::Number(62491));
        assert_eq!(d.part2(), Answer::Number(17408399184));
    }

    #[test]
    fn test_day12() {
        let d = Day12::load("data_aoc2022/day12_input.txt");
        assert_eq!(d.part1(), Answer::Number(534));
        assert_eq!(d.part2(), Answer::Number(525));
    }

    #[test]
    fn test_day13() {
        let d = Day13::load("data_aoc2022/day13_input.txt");
        assert_eq!(d.part1(), Answer::Number(1));
        assert_eq!(d.part2(), Answer::Number(2));
    }
}

