#![allow(dead_code, unused_imports)]

use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Display};
use std::fs;

use std::path::Path;

use regex::Regex;

use read_input;

fn parse_lines<I>(lines: I) -> Vec<(String, i64)>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
    I::Item: Debug,
{
    let re = Regex::new(r"([a-z]+) (\d+)").unwrap();
    lines
        .into_iter()
        .map(|l| {
            let l = l.borrow();

            let matches = re.captures(l).unwrap();
            let dir = matches.get(1).unwrap().as_str().to_string();
            let steps: i64 = matches.get(2).unwrap().as_str().parse().unwrap();
            (dir, steps)
        })
        .collect()
}

fn calc_position(lines: &Vec<(String, i64)>) -> i64 {
    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;

    for (dir, steps) in lines {
        match dir.as_str() {
            "forward" => horizontal_position += steps,
            "up" => depth -= steps,
            "down" => depth += steps,
            o => panic!("unexpected direction {}", o),
        }
    }
    horizontal_position * depth
}

fn calc_position2(lines: &Vec<(String, i64)>) -> i64 {
    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim = 0;

    for (dir, steps) in lines {
        match dir.as_str() {
            "forward" => {
                horizontal_position += steps;
                depth += aim * steps;
            }
            "up" => aim -= steps,
            "down" => aim += steps,
            o => panic!("unexpected direction {}", o),
        }
    }
    horizontal_position * depth
}

fn main() {
    let example_lines: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let example_lines = parse_lines(example_lines.lines());

    // Part 1
    assert_eq!(calc_position(&example_lines), 150);
    let lines = read_input::read_input_lines!();
    let lines = parse_lines(lines);
    println!("Part 1: {}", calc_position(&lines));

    // Part 2
    assert_eq!(calc_position2(&example_lines), 900);
    println!("Part 2: {}", calc_position2(&lines));
}
