use crate::Solution;
use nom::types::CompleteStr;
use nom::{call, do_parse, error_position, map_res, named, tag, take_while1};
use std::error::Error;

pub(super) const DAY9: Solution = Solution {
    part1: |input| {
        let (players, last_marble) = get_puzzle_input(input)?;
        get_max_score(players, last_marble)
    },
    part2: |input| {
        let (players, last_marble) = get_puzzle_input(input)?;
        get_max_score(players, last_marble * 100)
    },
};

fn get_max_score(players: usize, last_marble: u32) -> Result<String, Box<dyn Error>> {
    let mut marbles = ArrayCyclicList::new(0);
    let mut cursor = marbles.cursor();
    let mut scores = vec![0; players];
    let mut player_numbers = (0..players).cycle();
    for marble_number in 1..=last_marble {
        let player_number = player_numbers.next().ok_or("No players")?;
        if marble_number % 23 == 0 {
            for _ in 0..=7 {
                cursor.prev();
            }
            scores[player_number] += cursor.remove() + marble_number;
            cursor.next();
        } else {
            cursor.next();
            cursor.insert(marble_number);
        }
    }
    Ok(scores.iter().max().unwrap().to_string())
}

fn get_puzzle_input(input: &str) -> Result<(usize, u32), Box<dyn Error + '_>> {
    let (rest, result) = puzzle_input(CompleteStr(input))?;
    if rest.is_empty() {
        Ok(result)
    } else {
        Err("Found text after input".into())
    }
}

named!(
    puzzle_input(CompleteStr<'_>) -> (usize, u32),
    do_parse!(
        player: integer_usize
            >> tag!(" players; last marble is worth ")
            >> last_marble: integer_u32
            >> tag!(" points")
            >> (player, last_marble)
    )
);

#[rustfmt::skip]
named!(
    integer_u32(CompleteStr<'_>) -> u32,
    map_res!(
        take_while1!(|c| char::is_digit(c, 10)),
        |x: CompleteStr<'_>| x.parse()
    )
);

#[rustfmt::skip]
named!(
    integer_usize(CompleteStr<'_>) -> usize,
    map_res!(
        take_while1!(|c| char::is_digit(c, 10)),
        |x: CompleteStr<'_>| x.parse()
    )
);

struct ArrayCyclicList<T> {
    nodes: Vec<ArrayCyclicListNode<T>>,
}

impl<T> ArrayCyclicList<T> {
    fn new(value: T) -> Self {
        ArrayCyclicList {
            nodes: vec![ArrayCyclicListNode {
                value,
                prev: 0,
                next: 0,
            }],
        }
    }

    fn cursor(&mut self) -> ArrayCyclicListCursor<'_, T> {
        ArrayCyclicListCursor {
            list: self,
            position: 0,
        }
    }
}

struct ArrayCyclicListCursor<'a, T> {
    list: &'a mut ArrayCyclicList<T>,
    position: usize,
}

impl<T> ArrayCyclicListCursor<'_, T> {
    fn next(&mut self) {
        self.position = self.list.nodes[self.position].next;
    }

    fn prev(&mut self) {
        self.position = self.list.nodes[self.position].prev;
    }

    fn insert(&mut self, value: T) {
        let Self {
            position,
            list: ArrayCyclicList { nodes },
        } = self;
        let insert_position = nodes.len();
        let next = *position;
        let next_node = &mut nodes[next];
        let prev = next_node.prev;
        next_node.prev = insert_position;
        nodes.push(ArrayCyclicListNode { value, prev, next });
        nodes[prev].next = insert_position;
    }

    fn remove(&mut self) -> T {
        let Self {
            position,
            list: ArrayCyclicList { nodes },
        } = self;
        let removed_position = *position;
        let ArrayCyclicListNode { prev, next, .. } = nodes[removed_position];
        *position = next;
        nodes[prev].next = next;
        nodes[next].prev = prev;
        let elem = nodes.swap_remove(removed_position).value;
        // Correct freshly swapped in node
        if let Some(&ArrayCyclicListNode { prev, next, .. }) = nodes.get(removed_position) {
            nodes[prev].next = removed_position;
            nodes[next].prev = removed_position;
        }
        elem
    }
}

struct ArrayCyclicListNode<T> {
    value: T,
    prev: usize,
    next: usize,
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY9.part1,
        example1: "9 players; last marble is worth 25 points" => 32,
        example2: "10 players; last marble is worth 1618 points" => 8_317,
        example3: "13 players; last marble is worth 7999 points" => 146_373,
        example4: "17 players; last marble is worth 1104 points" => 2_764,
        example5: "21 players; last marble is worth 6111 points" => 54_718,
        example6: "30 players; last marble is worth 5807 points" => 37_305,
        input: "464 players; last marble is worth 71730 points" => 380_705,
    );
    test!(
        DAY9.part2,
        input: "464 players; last marble is worth 71730 points" => 3_171_801_582,
    );
}
