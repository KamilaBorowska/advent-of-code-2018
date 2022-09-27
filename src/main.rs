use clap::Parser;
use std::error::Error;
use std::io::{self, Read, Write};

mod cpu;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
#[cfg(test)]
mod testmacros;

struct Solution {
    part1: fn(&str) -> Result<String, Box<dyn Error + '_>>,
    part2: fn(&str) -> Result<String, Box<dyn Error + '_>>,
}

const SOLUTIONS: &[Solution] = &[
    day1::DAY1,
    day2::DAY2,
    day3::DAY3,
    day4::DAY4,
    day5::DAY5,
    day6::DAY6,
    day7::DAY7,
    day8::DAY8,
    day9::DAY9,
    day10::DAY10,
    day11::DAY11,
    day12::DAY12,
    day13::DAY13,
    day14::DAY14,
    day15::DAY15,
    day16::DAY16,
    day17::DAY17,
    day18::DAY18,
    day19::DAY19,
    day20::DAY20,
    day21::DAY21,
    day22::DAY22,
    day23::DAY23,
    day24::DAY24,
    day25::DAY25,
];

#[derive(Parser)]
struct Options {
    /// Day for which a solution should be ran
    day: u8,
    /// Input, if not provided taken from stdin
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Options::parse();
    let solution = SOLUTIONS
        .get(usize::from(opt.day) - 1)
        .ok_or("Day number out of range")?;
    let input = match opt.input {
        Some(input) => input,
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };
    writeln!(
        io::stdout(),
        "Part 1: {}",
        (solution.part1)(&input).map_err(|e| e.to_string())?
    )?;
    writeln!(
        io::stdout(),
        "Part 2: {}",
        (solution.part2)(&input).map_err(|e| e.to_string())?
    )?;
    Ok(())
}
