use crate::Solution;
use failure::err_msg;
use std::collections::HashSet;

pub(super) const DAY5: Solution = Solution {
    part1: |input| {
        let mut queue: Vec<char> = Vec::new();
        for b in input.trim().chars() {
            let reacts = queue
                .last()
                .map(|&a| a != b && a.to_ascii_uppercase() == b.to_ascii_uppercase())
                .unwrap_or(false);
            if reacts {
                queue.pop();
            } else {
                queue.push(b);
            }
        }
        Ok(queue.len().to_string())
    },
    part2: |input| {
        let input = input.trim();
        let letters: HashSet<char> = input.chars().filter(|c| c.is_ascii_lowercase()).collect();
        let min = letters
            .iter()
            .map(|&letter| {
                let mut queue: Vec<char> = Vec::new();
                for b in input
                    .chars()
                    .filter(|&l| l != letter as char && l != letter.to_ascii_uppercase() as char)
                {
                    let reacts = queue
                        .last()
                        .map(|&a| a != b && a.to_ascii_uppercase() == b.to_ascii_uppercase())
                        .unwrap_or(false);
                    if reacts {
                        queue.pop();
                    } else {
                        queue.push(b);
                    }
                }
                queue.len()
            })
            .min()
            .ok_or_else(|| err_msg("Empty input"))?;
        Ok(min.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY5.part1,
        empty: "" => 0,
        example1: "aA" => 0,
        example2: "abBA" => 0,
        example3: "abAB" => 4,
        example4: "aabAAB" => 6,
        example5: "dabAcCaCBAcCcaDA" => 10,
        input: 9202,
    );
    test!(
        DAY5.part2,
        example: "dabAcCaCBAcCcaDA" => 4,
        input: 6394,
    );
}
