#![allow(dead_code)]

use std::borrow::Borrow;
use std::fmt::Debug;

use std::path::Path;
use std::str::Chars;

use read_input;

fn parse_lines<I>(lines: I) -> Vec<Chars<'static>>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
    I::Item: Debug,
{
    lines
        .into_iter()
        .map(|l| {
            let l = l.borrow();
            l.to_string().chars()
        })
        .collect()
}

fn main() {
    let example_lines: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let example_lines = parse_lines(example_lines.lines());
    println!("{:?}", example_lines);

    // Part 1
    // assert_eq!(calc_position(&example_lines), 150);
    let lines = read_input::read_input_lines!();
    let lines = parse_lines(lines);
    // println!("Part 1: {}", calc_position(&lines));

    // Part 2
    // assert_eq!(calc_position2(&example_lines), 900);
    // println!("Part 2: {}", calc_position2(&lines));
}
