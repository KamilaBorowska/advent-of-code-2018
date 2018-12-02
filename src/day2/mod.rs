use crate::Solution;
use failure::err_msg;
use std::collections::HashMap;

pub(super) const DAY2: Solution = Solution {
    part1: |input| {
        let mut twice = 0;
        let mut thrice = 0;
        for line in input.lines() {
            let mut counts = HashMap::new();
            for c in line.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }
            if counts.values().any(|&v| v == 2) {
                twice += 1;
            }
            if counts.values().any(|&v| v == 3) {
                thrice += 1;
            }
        }
        Ok((twice * thrice).to_string())
    },
    part2: |input| {
        let lines: Vec<&str> = input.lines().collect();
        let pair = lines
            .iter()
            .flat_map(|&a| lines.iter().map(move |&b| (a, b)))
            .find(|&pair| differs_by_exactly_one_character(pair))
            .ok_or_else(|| err_msg("No common IDs found"))?;
        Ok(zip_chars(pair)
            .filter(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect())
    },
};

fn differs_by_exactly_one_character(pair: (&str, &str)) -> bool {
    zip_chars(pair).filter(|(a, b)| a != b).count() == 1
}

fn zip_chars<'a>((a, b): (&'a str, &'a str)) -> impl Iterator<Item = (char, char)> + 'a {
    a.chars().zip(b.chars())
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY2.part1,
        empty: "" => 0,
        example: lines!("abcdef" "bababc" "abbcde" "abcccd" "aabcdd" "abcdee" "ababab") => 12,
        input: 7936,
    );
    test!(
        DAY2.part2,
        fn empty_input_fails() {
            assert!((DAY2.part2)("").is_err());
        }
        fn input_with_no_common_ids_fails() {
            assert!((DAY2.part2)(lines!("aa" "bb")).is_err());
        }
        example: lines!("abcde" "fghij" "klmno" "pqrst" "fguij" "axcye" "wvxyz") => "fgij",
        input: "lnfqdscwjyteorambzuchrgpx",
    );
}
