use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

enum Op {
    Noop,
    Addx(isize),
}

pub struct Day10 {
    ops: Vec<Op>,
}

impl Day10 {
    pub fn load(filename: &str) -> Day10 {
        let mut ops: Vec<Op> = Vec::new();
        lazy_static! {
            static ref ADDX_RE: Regex =
                Regex::new("addx (-?[0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            if l.contains("noop") {
                // noop
                ops.push(Op::Noop);
            }
            else {
                let caps = ADDX_RE.captures(&l);
                match caps {
                    Some(caps) => {
                        // addx
                        let arg = caps[1].parse::<isize>().unwrap();
                        ops.push(Op::Addx(arg));
                    }
                    None => {}
                }
            }
        }

        Day10 { ops }
    }

    fn run_ops(&self) -> (usize, String) {
        let mut cycle = 0;
        let mut countdown = 20;
        let mut signal_sum = 0;
        let mut x = 1;
        let mut image: String = String::new();
        let mut cursor_x = 0;

        for op in &self.ops {
            // Use op to determine cycles used and value after they are done
            let (cycles, update) = match op {
                Op::Noop => {
                    (1, x)
                }
                Op::Addx(value) => {
                    (2, x+value)
                }
            };

            // Run cycles
            for _ in 0..cycles {
                cycle += 1;
                countdown -= 1;
                if countdown == 0 {
                    signal_sum += cycle*x;
                    // println!("In cycle {}, X={}, sum={}", cycle, x, signal_sum);
                    countdown = 40;
                }

                // evaluate pixel
                if (cursor_x >= x-1) && (cursor_x <= x+1) {
                    // pixel is on
                    image.push('#');
                }
                else {
                    image.push('.');
                }

                // update cursor
                cursor_x += 1;
                if cursor_x >= 40 {
                    cursor_x = 0;
                    image.push('\n');
                }
            }
            x = update;
        }

        (signal_sum as usize, image)
    }
}

impl Day for Day10 {
    fn part1(&self) -> Answer {
        let (ss, _image) = self.run_ops();
        Answer::Number(ss)
    }

    fn part2(&self) -> Answer {
        let (_ss, image) = self.run_ops();
        Answer::Message(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load1() {
        let d = Day10::load("examples/day10_example1.txt");
        assert_eq!(d.ops.len(), 3);
    }

    #[test]
    fn test_load2() {
        let d = Day10::load("examples/day10_example2.txt");
        assert_eq!(d.ops.len(), 146);
    }

    #[test]
    fn test_example2() {
        let expected_image =
            "##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######.....\n";
        let d = Day10::load("examples/day10_example2.txt");
        let (ss, image) = d.run_ops();
        assert_eq!(ss, 13140);
        print!("{}", image);
        assert_eq!(image, expected_image);
    }

    #[test]
    fn test_part1() {
        let d = Day10::load("examples/day10_example2.txt");
        assert_eq!(d.part1(), Answer::Number(13140));
    }

    #[test]
    fn test_part2() {
        let expected_image =
            "##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######.....\n";
        let d = Day10::load("examples/day10_example2.txt");
        assert_eq!(d.part2(), Answer::Message(expected_image.to_string()));
    }
}
