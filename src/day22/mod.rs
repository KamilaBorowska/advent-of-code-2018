use crate::Solution;
use nom::types::CompleteStr;
use nom::{call, do_parse, error_position, map_res, named, tag, take_while};
use std::cmp::Reverse;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;

type XyPair = (u32, u32);
const ADJACENT_CALLBACKS: &[fn(u32, u32) -> Option<XyPair>] = &[
    |x, y| Some((x.checked_sub(1)?, y)),
    |x, y| Some((x + 1, y)),
    |x, y| Some((x, y.checked_sub(1)?)),
    |x, y| Some((x, y + 1)),
];

pub(crate) const DAY22: Solution = Solution {
    part1: |input| {
        let mut map = Map::new(input)?;
        let MapPosition { x, y } = map.target;
        Ok((0..=x)
            .flat_map(|x| (0..=y).map(move |y| MapPosition { x, y }))
            .map(|position| map.get_corrosion(position) % 3)
            .sum::<u32>()
            .to_string())
    },
    part2: |input| {
        let mut map = Map::new(input)?;
        let mut heap = BinaryHeap::new();
        let mut checked = HashSet::new();
        heap.push((
            Reverse(0u32),
            QueuePosition {
                map_position: MapPosition { x: 0, y: 0 },
                blocked_tile: 1,
            },
        ));
        while let Some((Reverse(time), position)) = heap.pop() {
            let QueuePosition {
                map_position,
                blocked_tile,
            } = position;
            if map_position == map.target && blocked_tile == 1 {
                return Ok(time.to_string());
            }
            if !checked.insert(position) {
                continue;
            }
            let MapPosition { x, y } = map_position;
            heap.extend(
                ADJACENT_CALLBACKS
                    .iter()
                    .filter_map(|f| f(x, y))
                    .map(|(x, y)| MapPosition { x, y })
                    .filter(|&p| map.get_corrosion(p) % 3 != blocked_tile.into())
                    .map(|map_position| {
                        (
                            Reverse(time + 1),
                            QueuePosition {
                                map_position,
                                blocked_tile,
                            },
                        )
                    }),
            );
            heap.push((
                Reverse(time + 7),
                QueuePosition {
                    map_position,
                    blocked_tile: match (map.get_corrosion(map_position) % 3, blocked_tile) {
                        (0, 2) | (2, 0) => 1,
                        (0, 1) | (1, 0) => 2,
                        (1, 2) | (2, 1) => 0,
                        _ => panic!("Unexpected pairing of board position and blocked tile"),
                    },
                },
            ));
        }
        panic!("Cannot reach max position");
    },
};

struct Map {
    board: HashMap<MapPosition, u32>,
    target: MapPosition,
    depth: Depth,
}

impl Map {
    fn new(text: &str) -> Result<Self, Box<dyn Error + '_>> {
        match input(CompleteStr(text))? {
            (CompleteStr(""), rest) => Ok(rest),
            _ => Err("Unexpected text after input".into()),
        }
    }

    fn get_corrosion(&mut self, position: MapPosition) -> u32 {
        let depth = self.depth;
        match self.board.entry(position) {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(vacant) => match position {
                MapPosition { x, y: 0 } => *vacant.insert(depth.corrosion(x * 16_807)),
                MapPosition { x: 0, y } => *vacant.insert(depth.corrosion(y * 48_271)),
                _ if position == self.target => *vacant.insert(depth.corrosion(0)),
                MapPosition { x, y } => {
                    let value = depth.corrosion(
                        self.get_corrosion(MapPosition { x: x - 1, y })
                            * self.get_corrosion(MapPosition { x, y: y - 1 }),
                    );
                    self.board.insert(position, value);
                    value
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct MapPosition {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone)]
struct Depth(u32);

impl Depth {
    fn corrosion(self, geologic: u32) -> u32 {
        (geologic + self.0) % 20_183
    }
}

named!(
    input(CompleteStr<'_>) -> Map,
    do_parse!(
        tag!("depth: ")
            >> depth: integer
            >> tag!("\ntarget: ")
            >> x: integer
            >> tag!(",")
            >> y: integer
            >> tag!("\n")
            >> (Map {
                depth: Depth(depth),
                target: MapPosition { x, y },
                board: HashMap::new()
            })
    )
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> u32,
    map_res!(take_while!(|c| char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct QueuePosition {
    map_position: MapPosition,
    blocked_tile: u8,
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY22.part1,
        example: lines!(
            "depth: 510"
            "target: 10,10"
        ) => 114,
        input: 6_323,
    );
    test!(
        DAY22.part2,
        example: lines!(
            "depth: 510"
            "target: 10,10"
        ) => 45,
        input: 982, // 975 is too low
    );
}
