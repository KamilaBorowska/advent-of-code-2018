use crate::cpu::{get_cpu, Instruction, InstructionKind};
use crate::Solution;
use std::collections::HashSet;

pub(crate) const DAY21: Solution = Solution {
    part1: |input| {
        let mut cpu = get_cpu(input, [0; 6])?;
        while let Some(instruction) = cpu.current_instruction() {
            let Instruction { kind, parameters } = instruction;
            match (kind, parameters) {
                (InstructionKind::Eqrr, [0, x, _]) | (InstructionKind::Eqrr, [x, 0, _]) => {
                    return Ok(cpu.registers[x].to_string());
                }
                _ => {}
            }
            cpu.step();
        }
        Err("Expected to find eq instruction".into())
    },
    part2: |input| {
        let mut cpu = get_cpu(input, [0; 6])?;
        let mut last_result = None;
        let mut found = HashSet::new();
        while let Some(instruction) = cpu.current_instruction() {
            let Instruction { kind, parameters } = instruction;
            match (kind, parameters) {
                (InstructionKind::Eqrr, [0, x, _]) | (InstructionKind::Eqrr, [x, 0, _]) => {
                    let value = cpu.registers[x];
                    if found.insert(value) {
                        last_result = Some(value);
                    } else {
                        return Ok(last_result.ok_or("No results obtained")?.to_string());
                    }
                }
                _ => {}
            }
            cpu.step();
        }
        Err("Program halted on 0".into())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY21.part1,
        input: 12_980_435,
    );

    test!(
        DAY21.part2,
        // Too slow
        #[ignore] input: 14_431_711,
    );
}
