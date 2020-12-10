use crate::Solution;
use nom::types::CompleteStr;
use nom::{do_parse, many0, map_res, named, tag, take_while};
use std::collections::hash_map::{Entry, HashMap};
use std::collections::HashSet;
use std::error::Error;

pub(crate) const DAY16: Solution = Solution {
    part1: |input| {
        let input = get_input(input)?;
        Ok(input
            .tests
            .iter()
            .filter(|t| t.possible_opcodes().count() >= 3)
            .count()
            .to_string())
    },
    part2: |input| {
        let input = get_input(input)?;
        let opcode_map = get_opcode_map(&input);
        let mut registers = [0; 4];
        for line in input.program {
            registers[line[3]] = opcode_map[&line[0]](registers, line[1], line[2])
        }
        Ok(registers[0].to_string())
    },
};

#[derive(Debug)]
struct Input {
    tests: Vec<Test>,
    program: Vec<Line>,
}

type Registers = [usize; 4];
type Line = [usize; 4];

#[derive(Debug)]
struct Test {
    before: Registers,
    line: Line,
    after: Registers,
}

fn get_input(text: &str) -> Result<Input, Box<dyn Error + '_>> {
    let (rest, result) = input(CompleteStr(text))?;
    if rest.is_empty() {
        Ok(result)
    } else {
        Err("Found text after input".into())
    }
}

named!(
    input(CompleteStr<'_>) -> Input,
    do_parse!(
        tests: many0!(test) >> tag!("\n\n") >> program: many0!(line) >> (Input { tests, program })
    )
);

named!(
    test(CompleteStr<'_>) -> Test,
    do_parse!(
        tag!("Before:")
            >> before: registers
            >> line: line
            >> tag!("After: ")
            >> after: registers
            >> tag!("\n")
            >> (Test {
                before,
                line,
                after
            })
    )
);

named!(
    registers(CompleteStr<'_>) -> Registers,
    do_parse!(
        tag!(" [")
            >> first: integer
            >> tag!(", ")
            >> second: integer
            >> tag!(", ")
            >> third: integer
            >> tag!(", ")
            >> fourth: integer
            >> tag!("]\n")
            >> ([first, second, third, fourth])
    )
);

named!(
    line(CompleteStr<'_>) -> Line,
    do_parse!(
        first: integer
            >> tag!(" ")
            >> second: integer
            >> tag!(" ")
            >> third: integer
            >> tag!(" ")
            >> fourth: integer
            >> tag!("\n")
            >> ([first, second, third, fourth])
    )
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> usize,
    map_res!(take_while!(|c| char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

const OPCODES: &[Opcode] = &[
    // addr
    |r, a, b| r[a] + r[b],
    // addi
    |r, a, b| r[a] + b,
    // mulr
    |r, a, b| r[a] * r[b],
    // muli
    |r, a, b| r[a] * b,
    // banr
    |r, a, b| r[a] & r[b],
    // bani
    |r, a, b| r[a] & b,
    // borr
    |r, a, b| r[a] | r[b],
    // bori
    |r, a, b| r[a] | b,
    // setr
    |r, a, _| r[a],
    // seti
    |_, a, _| a,
    // gtir
    |r, a, b| (a > r[b]).into(),
    // gtri
    |r, a, b| (r[a] > b).into(),
    // gtrr
    |r, a, b| (r[a] > r[b]).into(),
    // eqir
    |r, a, b| (a == r[b]).into(),
    // eqri
    |r, a, b| (r[a] == b).into(),
    // eqrr
    |r, a, b| (r[a] == r[b]).into(),
];

type Opcode = fn(Registers, usize, usize) -> usize;

impl Test {
    fn possible_opcodes(&self) -> impl Iterator<Item = usize> {
        let Test {
            before,
            line: [_, a, b, c],
            after,
        } = *self;
        let after = after[c];
        OPCODES
            .iter()
            .enumerate()
            .filter(move |(_, opcode)| opcode(before, a, b) == after)
            .map(|(i, _)| i)
    }
}

fn get_opcode_map(input: &Input) -> HashMap<usize, Opcode> {
    let mut opcode_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for test in &input.tests {
        let opcode = test.line[0];
        let possible_opcodes: HashSet<_> = test.possible_opcodes().collect();
        match opcode_map.entry(opcode) {
            Entry::Occupied(mut occupied) => {
                let occupied = occupied.get_mut();
                *occupied = &possible_opcodes & occupied;
            }
            Entry::Vacant(vacant) => {
                vacant.insert(possible_opcodes);
            }
        }
    }
    loop {
        let mut already_recognized = HashSet::new();
        let mut should_break = true;
        for entry in opcode_map.values() {
            if entry.len() == 1 {
                already_recognized.insert(*entry.iter().next().unwrap());
            } else {
                should_break = false;
            }
        }
        if should_break {
            break;
        }
        for entry in opcode_map.values_mut() {
            if entry.len() != 1 {
                for recognized in &already_recognized {
                    entry.remove(recognized);
                }
            }
        }
    }
    opcode_map
        .into_iter()
        .map(|(k, v)| {
            assert_eq!(v.len(), 1);
            (k, OPCODES[v.into_iter().next().unwrap()])
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::test;

    test!(
        DAY16.part1,
        fn test_identical_opcodes() {
            use crate::day16::Test;
            assert_eq!(
                Test {
                    before: [3, 2, 1, 1],
                    line: [9, 2, 1, 2],
                    after: [3, 2, 2, 1]
                }.possible_opcodes().count(),
                3
            );
        }
        input: 544,
    );
    test!(
        DAY16.part2,
        input: 600,
    );
}
