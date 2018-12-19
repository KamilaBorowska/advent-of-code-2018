use crate::Solution;
use nom::types::CompleteStr;
use nom::{alt, call, do_parse, error_position, many0, map_res, named, tag, take_while};
use std::error::Error;

pub(crate) const DAY19: Solution = Solution {
    part1: |input| {
        let mut cpu = get_cpu(input, [0; 6])?;
        while cpu.step() {}
        Ok(cpu.registers[0].to_string())
    },
    part2: |input| {
        let mut cpu = get_cpu(input, [1, 0, 0, 0, 0, 0])?;
        // Determined by manually analyzing the assembly, it may not match other assemblies
        // After 100 instructions, the max value should be set
        for _ in 0..100 {
            cpu.step();
        }
        let &max = cpu.registers.iter().max().unwrap();
        // Calculating the sum of integer factors
        Ok((1..=(max as f64).sqrt() as usize)
            .filter(|i| max % i == 0)
            .map(|factor| {
                let other_factor = max / factor;
                if other_factor == factor {
                    factor
                } else {
                    other_factor + factor
                }
            })
            .sum::<usize>()
            .to_string())
    },
};

#[derive(Debug)]
struct CPU {
    ip: usize,
    registers: [usize; 6],
    instructions: Vec<Instruction>,
}

impl CPU {
    fn step(&mut self) -> bool {
        match self.instructions.get(self.registers[self.ip]) {
            Some(instruction) => {
                self.registers[instruction.parameters[2]] = instruction.run(self.registers);
                self.registers[self.ip] += 1;
                true
            }
            None => false,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    kind: InstructionKind,
    parameters: [usize; 3],
}

impl Instruction {
    fn run(&self, registers: [usize; 6]) -> usize {
        use self::InstructionKind::*;
        let [a, b, _] = self.parameters;
        match self.kind {
            Addr => registers[a] + registers[b],
            Addi => registers[a] + b,
            Mulr => registers[a] * registers[b],
            Muli => registers[a] * b,
            Banr => registers[a] & registers[b],
            Bani => registers[a] & b,
            Borr => registers[a] & registers[b],
            Bori => registers[a] & b,
            Setr => registers[a],
            Seti => a,
            Gtir => (a > registers[b]).into(),
            Gtri => (registers[a] > b).into(),
            Gtrr => (registers[a] > registers[b]).into(),
            Eqir => (a == registers[b]).into(),
            Eqri => (registers[a] == b).into(),
            Eqrr => (registers[a] == registers[b]).into(),
        }
    }
}

#[derive(Debug)]
enum InstructionKind {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn get_cpu(input: &str, registers: [usize; 6]) -> Result<CPU, Box<dyn Error + '_>> {
    match cpu(CompleteStr(input))? {
        (CompleteStr(""), cpu) => Ok(CPU { registers, ..cpu }),
        (text, _) => Err(format!("Found text after input: {}", text))?,
    }
}

named!(
    cpu(CompleteStr<'_>) -> CPU,
    do_parse!(
        tag!("#ip ")
            >> ip: integer
            >> tag!("\n")
            >> instructions: many0!(instruction)
            >> (CPU {
                ip,
                instructions,
                registers: [0; 6]
            })
    )
);

named!(
    instruction(CompleteStr<'_>) -> Instruction,
    do_parse!(
        kind: instruction_kind
            >> tag!(" ")
            >> first: integer
            >> tag!(" ")
            >> second: integer
            >> tag!(" ")
            >> third: integer
            >> tag!("\n")
            >> (Instruction {
                kind,
                parameters: [first, second, third]
            })
    )
);

named!(
    instruction_kind(CompleteStr<'_>) -> InstructionKind,
    alt!(
        tag!("addr") => { |_| InstructionKind::Addr } |
        tag!("addi") => { |_| InstructionKind::Addi } |
        tag!("mulr") => { |_| InstructionKind::Mulr } |
        tag!("muli") => { |_| InstructionKind::Muli } |
        tag!("banr") => { |_| InstructionKind::Banr } |
        tag!("bani") => { |_| InstructionKind::Bani } |
        tag!("borr") => { |_| InstructionKind::Borr } |
        tag!("bori") => { |_| InstructionKind::Bori } |
        tag!("setr") => { |_| InstructionKind::Setr } |
        tag!("seti") => { |_| InstructionKind::Seti } |
        tag!("gtir") => { |_| InstructionKind::Gtir } |
        tag!("gtri") => { |_| InstructionKind::Gtri } |
        tag!("gtrr") => { |_| InstructionKind::Gtrr } |
        tag!("eqir") => { |_| InstructionKind::Eqir } |
        tag!("eqri") => { |_| InstructionKind::Eqri } |
        tag!("eqrr") => { |_| InstructionKind::Eqrr }
    )
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> usize,
    map_res!(take_while!(|c| char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY19.part1,
        example: lines!(
            "#ip 0"
            "seti 5 0 1"
            "seti 6 0 2"
            "addi 0 1 0"
            "addr 1 2 3"
            "setr 1 0 0"
            "seti 8 0 4"
            "seti 9 0 5"
        ) => 7,
        input: 1_922,
    );
    test!(
        DAY19.part2,
        input: 22_302_144,
    );
}
