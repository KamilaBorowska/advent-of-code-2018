use failure::err_msg;
use std::io::{self, Read, Write};
use structopt::StructOpt;

mod day1;
mod day2;
#[cfg(test)]
mod testmacros;

struct Solution {
    part1: fn(&str) -> Result<String, failure::Error>,
    part2: fn(&str) -> Result<String, failure::Error>,
}

const SOLUTIONS: &[Solution] = &[day1::DAY1, day2::DAY2];

#[derive(StructOpt)]
struct Options {
    /// Day for which a solution should be ran
    day: u8,
}

fn main() -> Result<(), failure::Error> {
    let opt = Options::from_args();
    let solution = SOLUTIONS
        .get(opt.day as usize - 1)
        .ok_or_else(|| err_msg("Day number out of range"))?;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    writeln!(io::stdout(), "Part 1: {}", (solution.part1)(&input)?)?;
    writeln!(io::stdout(), "Part 2: {}", (solution.part2)(&input)?)?;
    Ok(())
}
