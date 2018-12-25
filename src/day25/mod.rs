use crate::Solution;
use nom::{call, char, do_parse, error_position, map_res, named, take_while, types::CompleteStr};
use std::error::Error;

pub(crate) const DAY25: Solution = Solution {
    part1: |input| {
        let mut constellations: Vec<Vec<Position>> = Vec::new();
        for line in input.lines() {
            let new_position = get_position(line)?;
            let mut constellation = vec![new_position];
            for i in (0..constellations.len()).rev() {
                for position in &constellations[i] {
                    if position.distance_to(new_position) <= 3 {
                        constellation.extend(constellations.swap_remove(i));
                        break;
                    }
                }
            }
            constellations.push(constellation);
        }
        Ok(constellations.len().to_string())
    },
    part2: |_| Ok(String::from("Trigger the Underflow")),
};

fn get_position(line: &str) -> Result<Position, Box<dyn Error + '_>> {
    match position(CompleteStr(line))? {
        (CompleteStr(""), position) => Ok(position),
        _ => Err("Found text after constellation")?,
    }
}

#[derive(Copy, Clone)]
struct Position([i8; 4]);

impl Position {
    fn distance_to(self, other: Self) -> i32 {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(&a, &b)| (i32::from(a) - i32::from(b)).abs())
            .sum()
    }
}

named!(
    position(CompleteStr<'_>) -> Position,
    do_parse!(
        a: integer
            >> char!(',')
            >> b: integer
            >> char!(',')
            >> c: integer
            >> char!(',')
            >> d: integer
            >> (Position([a, b, c, d]))
    )
);

named!(
    integer(CompleteStr<'_>) -> i8,
    map_res!(
        take_while!(|c| c == '-' || char::is_digit(c, 10)),
        |x: CompleteStr<'_>| x.parse()
    )
);

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY25.part1,
        example1: lines!(
            "0,0,0,0"
            "3,0,0,0"
            "0,3,0,0"
            "0,0,3,0"
            "0,0,0,3"
            "0,0,0,6"
            "9,0,0,0"
            "12,0,0,0"
        ) => 2,
        example2: lines!(
            "-1,2,2,0"
            "0,0,2,-2"
            "0,0,0,-2"
            "-1,2,0,0"
            "-2,-2,-2,2"
            "3,0,2,-1"
            "-1,3,2,2"
            "-1,0,-1,0"
            "0,2,1,-2"
            "3,0,0,0"
        ) => 4,
        example3: lines!(
            "1,-1,0,1"
            "2,0,-1,0"
            "3,2,-1,0"
            "0,0,3,1"
            "0,0,-1,-1"
            "2,3,-2,0"
            "-2,2,0,0"
            "2,-2,0,-1"
            "1,-1,0,-1"
            "3,2,0,2"
        ) => 3,
        example4: lines!(
            "1,-1,-1,-2"
            "-2,-2,0,1"
            "0,2,1,3"
            "-2,3,-2,1"
            "0,2,3,-2"
            "-1,-1,1,-2"
            "0,-2,-1,0"
            "-2,2,3,-1"
            "1,2,2,0"
            "-1,-2,0,-2"
        ) => 8,
        input: 428,
    );
}
