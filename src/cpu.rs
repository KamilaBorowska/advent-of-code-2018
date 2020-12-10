use nom::types::CompleteStr;
use nom::{alt, call, do_parse, error_position, many0, map_res, named, tag, take_while};
use std::error::Error;

#[derive(Debug)]
pub(crate) struct CPU {
    ip: usize,
    pub(crate) registers: [usize; 6],
    instructions: Vec<Instruction>,
}

impl CPU {
    pub(crate) fn step(&mut self) -> bool {
        match self.current_instruction() {
            Some(instruction) => {
                self.registers[instruction.parameters[2]] = instruction.run(self.registers);
                self.registers[self.ip] += 1;
                true
            }
            None => false,
        }
    }

    pub(crate) fn current_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.registers[self.ip]).cloned()
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Instruction {
    pub(crate) kind: InstructionKind,
    pub(crate) parameters: [usize; 3],
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
            Borr => registers[a] | registers[b],
            Bori => registers[a] | b,
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

#[derive(Copy, Clone, Debug)]
pub(crate) enum InstructionKind {
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

pub(crate) fn get_cpu(input: &str, registers: [usize; 6]) -> Result<CPU, Box<dyn Error + '_>> {
    match cpu(CompleteStr(input))? {
        (CompleteStr(""), cpu) => Ok(CPU { registers, ..cpu }),
        (text, _) => Err(format!("Found text after input: {}", text).into()),
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
