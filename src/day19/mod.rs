use crate::cpu::get_cpu;
use crate::Solution;

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
