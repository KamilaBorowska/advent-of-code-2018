use crate::Solution;
use itertools::Itertools;
use nom::types::CompleteStr;
use nom::{alt, call, do_parse, error_position, map_res, named, tag, take_while};
use std::collections::hash_map::{Entry, HashMap};
use std::error::Error;
use std::ops::RangeInclusive;

pub(crate) const DAY17: Solution = Solution {
    part1: |input| {
        let mut board = Board::new(input)?;
        board.run_water(500, 0);
        Ok(board
            .count_tiles(&[BlockState::SolidWater, BlockState::Flowing])
            .to_string())
    },
    part2: |input| {
        let mut board = Board::new(input)?;
        board.run_water(500, 0);
        Ok(board.count_tiles(&[BlockState::SolidWater]).to_string())
    },
};

struct Board {
    map: HashMap<(i32, i32), BlockState>,
    min_y: i32,
    max_y: i32,
}

impl Board {
    fn new(input: &str) -> Result<Self, Box<dyn Error + '_>> {
        let mut map = HashMap::new();
        for line in input.lines() {
            match get_line(line)? {
                Line::XY(x, y) => map.extend(y.map(|y| ((x, y), BlockState::Solid))),
                Line::YX(y, x) => map.extend(x.map(|x| ((x, y), BlockState::Solid))),
            }
        }
        let (&min_y, &max_y) = map
            .keys()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .ok_or("No points")?;
        Ok(Self { map, min_y, max_y })
    }

    fn run_water(&mut self, x: i32, y: i32) {
        if y >= self.max_y {
            return;
        }
        if let Entry::Vacant(vacant) = self.map.entry((x, y + 1)) {
            vacant.insert(BlockState::Flowing);
            self.run_water(x, y + 1);
        }
        let is_bottom_solid = self.is_solid(x, y + 1);
        for offset in &[-1, 1] {
            if is_bottom_solid {
                if let Entry::Vacant(vacant) = self.map.entry((x + offset, y)) {
                    vacant.insert(BlockState::Flowing);
                    self.run_water(x + offset, y);
                }
            }
        }
        if self.has_both_walls(x, y) {
            self.fill_level(x, y);
        }
    }

    fn is_solid(&self, x: i32, y: i32) -> bool {
        [Some(&BlockState::Solid), Some(&BlockState::SolidWater)].contains(&self.map.get(&(x, y)))
    }

    fn has_both_walls(&self, x: i32, y: i32) -> bool {
        self.has_wall(x, y, 1) && self.has_wall(x, y, -1)
    }

    fn has_wall(&self, mut x: i32, y: i32, offset: i32) -> bool {
        loop {
            match self.map.get(&(x, y)) {
                None => return false,
                Some(BlockState::Solid) => return true,
                _ => x += offset,
            }
        }
    }

    fn fill_level(&mut self, x: i32, y: i32) {
        self.fill_side(x, y, 1);
        self.fill_side(x, y, -1);
    }

    fn fill_side(&mut self, mut x: i32, y: i32, offset: i32) {
        let map = &mut self.map;
        loop {
            match map.entry((x, y)) {
                Entry::Occupied(occupied) => {
                    let occupied = occupied.into_mut();
                    if *occupied == BlockState::Solid {
                        return;
                    }
                    *occupied = BlockState::SolidWater;
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(BlockState::SolidWater);
                }
            }
            x += offset;
        }
    }

    fn count_tiles(self, values: &[BlockState]) -> usize {
        let min_y = self.min_y;
        self.map
            .into_iter()
            .filter(move |&((_, y), v)| y >= min_y && values.contains(&v))
            .count()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BlockState {
    Solid,
    SolidWater,
    Flowing,
}

fn get_line(text: &str) -> Result<Line, Box<dyn Error + '_>> {
    match line(CompleteStr(text))? {
        (CompleteStr(""), line) => Ok(line),
        _ => Err("Unexpected text after a line")?,
    }
}

enum Line {
    XY(i32, RangeInclusive<i32>),
    YX(i32, RangeInclusive<i32>),
}

named!(
    line(CompleteStr<'_>) -> Line,
    alt!(
        do_parse!(
            tag!("x=") >>
            x: integer >>
            tag!(", y=") >>
            range: range >>
            (x, range)
        ) => { |(x, range)| Line::XY(x, range) } |
        do_parse!(
            tag!("y=") >>
            y: integer >>
            tag!(", x=") >>
            range: range >>
            (y, range)
        ) => { |(y, range)| Line::YX(y, range) }
    )
);

named!(
    range(CompleteStr<'_>) -> RangeInclusive<i32>,
    do_parse!(a: integer >> tag!("..") >> b: integer >> (a..=b))
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> i32,
    map_res!(take_while!(|c| char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY17.part1,
        example: lines!(
            "x=495, y=2..7"
            "y=7, x=495..501"
            "x=501, y=3..7"
            "x=498, y=2..4"
            "x=506, y=1..2"
            "x=498, y=10..13"
            "x=504, y=10..13"
            "y=13, x=498..504"
        ) => 57,
        complex_example: lines!(
            "x=500, y=42..42"
            "x=499, y=45..45"
            "x=501, y=45..45"
            "x=502, y=44..44"
        ) => 10,
        input: 37858,
    );
    test!(
        DAY17.part2,
        example: lines!(
            "x=495, y=2..7"
            "y=7, x=495..501"
            "x=501, y=3..7"
            "x=498, y=2..4"
            "x=506, y=1..2"
            "x=498, y=10..13"
            "x=504, y=10..13"
            "y=13, x=498..504"
        ) => 29,
        input: 30410,
    );
}
