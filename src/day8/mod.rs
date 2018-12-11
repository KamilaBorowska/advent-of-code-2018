use crate::Solution;
use std::error::Error;

pub(super) const DAY8: Solution = Solution {
    part1: |input| Ok(get_node_sum(&mut get_input_iterator(input))?.to_string()),
    part2: |input| Ok(get_root_node_value(&mut get_input_iterator(input))?.to_string()),
};

fn get_input_iterator(input: &str) -> impl Iterator<Item = Result<usize, Box<dyn Error>>> + '_ {
    input.split_whitespace().map(|x| Ok(x.parse()?))
}

fn get_node_sum(
    iter: &mut impl Iterator<Item = Result<usize, Box<dyn Error>>>,
) -> Result<usize, Box<dyn Error>> {
    let child_nodes = iter.next().ok_or("Missing child nodes quantity")??;
    let metadata = iter.next().ok_or("Missing metadata")??;
    let mut sum = 0;
    for _ in 0..child_nodes {
        sum += get_node_sum(iter)?;
    }
    for _ in 0..metadata {
        sum += iter.next().ok_or("Missing metadata")??;
    }
    Ok(sum)
}

fn get_root_node_value(
    iter: &mut impl Iterator<Item = Result<usize, Box<dyn Error>>>,
) -> Result<usize, Box<dyn Error>> {
    let child_nodes_count = iter.next().ok_or("Missing child nodes quantity")??;
    let metadata = iter.next().ok_or("Missing metadata")??;
    let mut child_nodes = Vec::new();
    for _ in 0..child_nodes_count {
        child_nodes.push(get_root_node_value(iter)?);
    }
    let mut sum = 0;
    for _ in 0..metadata {
        let value = iter.next().ok_or("Missing metadata")??;
        sum += if child_nodes.is_empty() {
            value
        } else {
            child_nodes.get(value - 1).cloned().unwrap_or(0)
        };
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY8.part1,
        example: "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2" => 138,
        input: 40_036,
    );
    test!(
        DAY8.part2,
        example: "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2" => 66,
        input: 21_677,
    );
}
