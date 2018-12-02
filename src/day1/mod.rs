use crate::Solution;
use failure::{bail, err_msg};
use std::collections::HashSet;

pub(super) const DAY1: Solution = Solution {
    part1: |input| {
        let mut sum = 0;
        for line in input.lines() {
            sum = line
                .parse::<i64>()?
                .checked_add(sum)
                .ok_or_else(|| err_msg("Integer overflow"))?;
        }
        Ok(sum.to_string())
    },
    part2: |input| {
        let lines = input
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<i64>, _>>()?;
        let mut sum = 0;
        let mut found = HashSet::new();
        for line in lines.iter().cycle() {
            if !found.insert(sum) {
                return Ok(sum.to_string());
            }
            sum = line
                .checked_add(sum)
                .ok_or_else(|| err_msg("Integer overflow"))?;
        }
        bail!("Empty input");
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY1.part1,
        fn overflow_fails() {
            assert!((DAY1.part2)(&format!("{}\n{0}", i64::max_value())).is_err());
        }
        empty: "" => 0,
        example1: lines!(1 -2 3 1) => 3,
        example2: lines!(1 1 1) => 3,
        example3: lines!(1 1 -2) => 0,
        example4: lines!(-1 -2 -3) => "-6",
        fn test_max_value() {
            let input = format!("{}\n{}", i64::max_value(), i64::min_value());
            assert_eq!((DAY1.part1)(&input).unwrap(), "-1");
        }
        input: 430,
    );
    test!(
        DAY1.part2,
        fn overflow_fails() {
            assert!((DAY1.part2)(&i64::max_value().to_string()).is_err());
        }
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
