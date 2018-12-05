use crate::Solution;

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
    part2: |_input| unimplemented!(),
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
}
