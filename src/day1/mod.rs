use crate::Solution;
use failure::bail;
use std::collections::HashSet;

pub(super) const DAY1: Solution = Solution {
    part1: |input| {
        let mut sum = 0;
        for line in input.lines() {
            sum += line.parse::<i32>()?;
        }
        Ok(sum)
    },
    part2: |input| {
        let lines = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()?;
        if lines.is_empty() {
            bail!("Empty input");
        }
        let mut sum = 0;
        let mut found = HashSet::new();
        for line in lines.iter().cycle() {
            if !found.insert(sum) {
                return Ok(sum);
            }
            sum += line;
        }
        unreachable!()
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY1.part1,
        empty: "" => 0,
        example1: lines!(1 -2 3 1) => 3,
        example2: lines!(1 1 1) => 3,
        example3: lines!(1 1 -2) => 0,
        example4: lines!(-1 -2 -3) => -6,
        input: 430,
    );
    test!(
        DAY1.part2,
        fn empty_input_fails() {
            assert!((DAY1.part2)("").is_err());
        }
        example1: lines!(1 -2 3 1) => 2,
        example2: lines!(1 -1) => 0,
        example3: lines!(3 3 4 -2 -4) => 10,
        example4: lines!(-6 3 8 5 -6) => 5,
        example5: lines!(7 7 -2 -7 -4) => 14,
        input: 462,
    );
}
