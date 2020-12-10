use crate::Solution;
use arraymap::ArrayMap;
use nom::types::CompleteStr;
use nom::{alt, do_parse, many0, named, preceded, r, tag};
use std::collections::HashSet;
use std::error::Error;

pub(crate) const DAY12: Solution = Solution {
    part1: |input| Ok(run_simulation(input, 20)?.to_string()),
    part2: |input| Ok(run_simulation(input, 50_000_000_000)?.to_string()),
};

fn run_simulation(input: &str, generations: i64) -> Result<i64, Box<dyn Error + '_>> {
    let mut lines = input.lines();
    let mut state = get_initial_state(lines.next().ok_or("Empty input")?)?;
    if lines.next() != Some("") {
        return Err("Expected empty line".into());
    }
    let rules = get_rules(lines)?;
    let mut beginning = 0;
    let mut previous_sum = 0;
    let mut previous_delta = 0;
    let mut delta_count = 0;
    for i in 0..generations {
        let (trimmed_elements, trimmed_state) = trim_state(&state);
        beginning += trimmed_elements as i64 - 2;
        state = (-2..trimmed_state.len() as isize + 2)
            .map(|i| {
                rules.contains(&ArrayMap::map(&[-2, -1, 0, 1, 2], |m| {
                    trimmed_state
                        .get((i + m) as usize)
                        .cloned()
                        .unwrap_or(false)
                }))
            })
            .collect();
        let sum = get_state_sum(beginning, &state);
        let delta = sum - previous_sum;
        if delta == previous_delta {
            delta_count += 1;
            if delta_count == 100 {
                return Ok(get_state_sum(beginning, &state) + (generations - i - 1) * delta);
            }
        }
        previous_sum = sum;
        previous_delta = delta;
    }
    Ok(get_state_sum(beginning, &state))
}

fn trim_state(mut input: &[bool]) -> (usize, &[bool]) {
    let mut counter = 0;
    while let Some((&false, rest)) = input.split_first() {
        counter += 1;
        input = rest;
    }
    while let Some((&false, rest)) = input.split_last() {
        input = rest;
    }
    (counter, input)
}

fn get_initial_state(input: &str) -> Result<Vec<bool>, Box<dyn Error + '_>> {
    match initial_state(CompleteStr(input))? {
        (CompleteStr(""), line) => Ok(line),
        _ => Err("Found text after a line".into()),
    }
}

named!(
    initial_state(CompleteStr<'_>) -> Vec<bool>,
    preceded!(tag!("initial state: "), many0!(symbol))
);

fn get_rules<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<HashSet<[bool; 5]>, Box<dyn Error + 'a>> {
    let mut output = HashSet::new();
    for line in lines {
        match rule(CompleteStr(line))? {
            (CompleteStr(""), ([false, false, false, false, false], true)) => {
                return Err("Sequence of five dots cannot map to octothorpes".into());
            }
            (CompleteStr(""), (input, true)) => {
                output.insert(input);
            }
            (CompleteStr(""), (_, false)) => {}
            _ => return Err("Found text after a line".into()),
        }
    }
    Ok(output)
}

named!(
    rule(CompleteStr<'_>) -> ([bool; 5], bool),
    do_parse!(
        a: symbol
            >> b: symbol
            >> c: symbol
            >> d: symbol
            >> e: symbol
            >> tag!(" => ")
            >> r: symbol
            >> ([a, b, c, d, e], r)
    )
);

named!(
    symbol(CompleteStr<'_>) -> bool,
    alt!(
        tag!("#") => { |_| true } |
        tag!(".") => { |_| false }
    )
);

fn get_state_sum(beginning: i64, state: &[bool]) -> i64 {
    state
        .iter()
        .enumerate()
        .filter(|&(_, &pot)| pot)
        .map(|(i, _)| i as i64 + beginning)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY12.part1,
        example: lines!(
            "initial state: #..#.#..##......###...###"
            ""
            "...## => #"
            "..#.. => #"
            ".#... => #"
            ".#.#. => #"
            ".#.## => #"
            ".##.. => #"
            ".#### => #"
            "#.#.# => #"
            "#.### => #"
            "##.#. => #"
            "##.## => #"
            "###.. => #"
            "###.# => #"
            "####. => #"
        ) => 325,
        input: 1623,
    );
    test!(
        DAY12.part2,
        input: 1_600_000_000_401,
    );
}
