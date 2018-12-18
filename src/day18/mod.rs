use crate::Solution;
use std::collections::HashMap;
use std::error::Error;

pub(crate) const DAY18: Solution = Solution {
    part1: |input| {
        let mut input = parse_input(input)?;
        for _ in 0..10 {
            input = input.next_state();
        }
        Ok((input.tiles.iter().filter(|&&t| t == Tile::Tree).count()
            * input
                .tiles
                .iter()
                .filter(|&&t| t == Tile::Lumberyard)
                .count())
        .to_string())
    },
    part2: |input| {
        let mut input = parse_input(input)?;
        let mut states = HashMap::new();
        for i in 0..1_000_000_000 {
            if let Some(previous) = states.insert(input.tiles.clone(), i) {
                let delta = i - previous;
                for _ in 0..(1_000_000_000 - i) % delta {
                    input = input.next_state();
                }
                break;
            }
            input = input.next_state();
        }
        Ok((input.tiles.iter().filter(|&&t| t == Tile::Tree).count()
            * input
                .tiles
                .iter()
                .filter(|&&t| t == Tile::Lumberyard)
                .count())
        .to_string())
    },
};

fn parse_input(input: &str) -> Result<Board, Box<dyn Error>> {
    let mut tiles = Vec::new();
    let mut width = input.len();
    let mut last_width = None;
    for (i, c) in input.bytes().enumerate() {
        match c {
            b'.' => tiles.push(Tile::OpenGround),
            b'|' => tiles.push(Tile::Tree),
            b'#' => tiles.push(Tile::Lumberyard),
            b'\n' => {
                if let Some(last_width) = last_width {
                    assert_eq!(last_width, i - width - 1);
                } else {
                    width = i;
                }
                last_width = Some(i);
            }
            _ => Err(format!("Unexpected character {:?}", char::from(c)))?,
        }
    }
    Ok(Board { tiles, width })
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Tile>,
    width: usize,
}

impl Board {
    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        Some(
            *self
                .tiles
                .get(y.checked_mul(self.width)?..)?
                .get(..self.width)?
                .get(x)?,
        )
    }

    fn next_state(&self) -> Board {
        let tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(i, &tile)| {
                let x = i % self.width;
                let y = i / self.width;
                let nearby = self.nearby(x, y);
                match tile {
                    Tile::OpenGround => {
                        if nearby.filter(|&t| t == Tile::Tree).count() >= 3 {
                            return Tile::Tree;
                        }
                    }
                    Tile::Tree => {
                        if nearby.filter(|&t| t == Tile::Lumberyard).count() >= 3 {
                            return Tile::Lumberyard;
                        }
                    }
                    Tile::Lumberyard => {
                        let mut found_lumberyard = false;
                        let mut found_tree = false;
                        for tile in nearby {
                            match tile {
                                Tile::Lumberyard => found_lumberyard = true,
                                Tile::Tree => found_tree = true,
                                Tile::OpenGround => {}
                            }
                        }
                        if !found_lumberyard || !found_tree {
                            return Tile::OpenGround;
                        }
                    }
                }
                tile
            })
            .collect();
        Board {
            tiles,
            width: self.width,
        }
    }

    fn nearby(&self, x: usize, y: usize) -> impl Iterator<Item = Tile> + '_ {
        let x = x as isize;
        let y = y as isize;
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter_map(move |(x, y)| self.get(x as usize, y as usize))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    OpenGround,
    Tree,
    Lumberyard,
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY18.part1,
        example: lines!(
            ".#.#...|#."
            ".....#|##|"
            ".|..|...#."
            "..|#.....#"
            "#.#|||#|#|"
            "...#.||..."
            ".|....|..."
            "||...#|.#|"
            "|.||||..|."
            "...#.|..|."
        ) => 1_147,
        input: 620_624,
    );
    test!(
        DAY18.part2,
        input: 169_234,
    );
}
