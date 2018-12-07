use crate::Solution;
use failure::{bail, format_err};
use nom::types::CompleteStr;
use nom::{anychar, call, do_parse, named, tag};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::mem;

pub(super) const DAY7: Solution = Solution {
    part1: |input| {
        let mut relations = get_relation_map(input)?;
        let mut ordering = String::new();
        let mut heap = get_initial_heap(&relations);
        while let Some(Reverse(current)) = heap.pop() {
            ordering.push(current);
            add_children(current, &mut relations, &mut heap);
        }
        Ok(ordering)
    },
    part2: |input| Ok(order_in_parallel(input, 5, 60)?.to_string()),
};

fn get_relation_map(input: &str) -> Result<HashMap<char, StepRelations>, failure::Error> {
    let mut relations = HashMap::new();
    for dependency in get_dependencies(input) {
        let Dependency { requirement, then } = dependency?;
        relations
            .entry(requirement)
            .or_insert_with(StepRelations::new)
            .children
            .push(then);
        relations
            .entry(then)
            .or_insert_with(StepRelations::new)
            .parent_count += 1;
    }
    Ok(relations)
}

fn get_dependencies(input: &str) -> impl Iterator<Item = Result<Dependency, failure::Error>> + '_ {
    input.lines().map(|line| {
        let (rest, point) =
            dependency(CompleteStr(line)).map_err(|e| format_err!("Parse failure: {}", e))?;
        if rest.is_empty() {
            Ok(point)
        } else {
            bail!("Text found in a line after a dependency");
        }
    })
}

named!(
    dependency(CompleteStr<'_>) -> Dependency,
    do_parse!(
        tag!("Step ")
            >> requirement: anychar
            >> tag!(" must be finished before step ")
            >> then: anychar
            >> tag!(" can begin.")
            >> (Dependency { requirement, then })
    )
);

struct Dependency {
    requirement: char,
    then: char,
}

#[derive(Default)]
struct StepRelations {
    parent_count: usize,
    children: Vec<char>,
}

impl StepRelations {
    fn new() -> Self {
        StepRelations::default()
    }
}

fn get_initial_heap(relations: &HashMap<char, StepRelations>) -> BinaryHeap<Reverse<char>> {
    relations
        .iter()
        .filter(|(_, v)| v.parent_count == 0)
        .map(|(&k, _)| Reverse(k))
        .collect()
}

fn add_children(
    current: char,
    relations: &mut HashMap<char, StepRelations>,
    heap: &mut BinaryHeap<Reverse<char>>,
) {
    let children = mem::replace(
        &mut relations.get_mut(&current).unwrap().children,
        Vec::new(),
    );
    for child in children {
        let parent_count = &mut relations.get_mut(&child).unwrap().parent_count;
        *parent_count -= 1;
        if *parent_count == 0 {
            heap.push(Reverse(child));
        }
    }
}

fn order_in_parallel(
    input: &str,
    elves: usize,
    additional_sleep: u32,
) -> Result<u32, failure::Error> {
    let mut relations = get_relation_map(input)?;
    let mut heap = get_initial_heap(&relations);
    let mut sleep_times = BinaryHeap::new();
    let mut time = 0;
    loop {
        while !heap.is_empty() && sleep_times.len() < elves {
            let Reverse(letter) = heap.pop().unwrap();
            sleep_times.push(Reverse((
                time + u32::from(letter) - u32::from(b'A') + 1 + additional_sleep,
                letter,
            )));
        }
        match sleep_times.pop() {
            None => return Ok(time),
            Some(Reverse((new_time, letter))) => {
                add_children(letter, &mut relations, &mut heap);
                time = new_time;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY7.part1,
        example: lines!(
            "Step C must be finished before step A can begin."
            "Step C must be finished before step F can begin."
            "Step A must be finished before step B can begin."
            "Step A must be finished before step D can begin."
            "Step B must be finished before step E can begin."
            "Step D must be finished before step E can begin."
            "Step F must be finished before step E can begin."
        ) => "CABDFE",
        input: "SCLPAMQVUWNHODRTGYKBJEFXZI",
    );
    test!(
        DAY7.part2,
        fn simple_example() {
            use crate::day7::order_in_parallel;
            let lines = lines!(
                "Step C must be finished before step A can begin."
                "Step C must be finished before step F can begin."
                "Step A must be finished before step B can begin."
                "Step A must be finished before step D can begin."
                "Step B must be finished before step E can begin."
                "Step D must be finished before step E can begin."
                "Step F must be finished before step E can begin."
            );
            assert_eq!(order_in_parallel(lines, 2, 0).unwrap(), 15);
        }
        input: 1234,
    );
}
