use crate::Solution;
use num_complex::Complex;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub(crate) const DAY13: Solution = Solution {
    part1: |input| {
        let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let mut carts: Vec<_> = get_carts(&map).collect();
        let mut positions: HashSet<_> = carts.iter().map(|c| c.position).collect();
        loop {
            carts.sort_by_key(|c| (c.position.im, c.position.re));
            for cart in &mut carts {
                if let Some(Complex { re, im }) = cart.tick(&mut positions)? {
                    return Ok(format!("{},{}", re, im));
                }
            }
        }
    },
    part2: |input| {
        let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let mut carts: HashMap<_, _> = get_carts(&map).enumerate().collect();
        let mut positions: HashMap<_, _> = carts.iter().map(|(&i, c)| (c.position, i)).collect();
        loop {
            let mut indexes: Vec<_> = carts.keys().cloned().collect();
            indexes.sort_by_key(|i| {
                let position = carts[i].position;
                (position.re, position.im)
            });
            for i in indexes {
                if let Some(cart) = carts.get_mut(&i) {
                    if let Some((position, other)) = cart.tick(&mut InsertMap(i, &mut positions))? {
                        carts.remove(&i);
                        carts.remove(&other);
                        positions.remove(&position);
                    }
                }
            }
            if carts.len() <= 1 {
                let Complex { re, im } = carts.values().next().ok_or("All cars crashed")?.position;
                return Ok(format!("{},{}", re, im));
            }
        }
    },
};

fn get_carts<'a>(map: &'a [&[u8]]) -> impl Iterator<Item = Cart<'a>> {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |rest| (y, rest)))
        .filter_map(move |(y, (x, tile))| {
            let direction = match tile {
                b'^' => -Complex::i(),
                b'v' => Complex::i(),
                b'<' => (-1).into(),
                b'>' => 1.into(),
                _ => return None,
            };
            Some(Cart {
                map,
                position: Complex::new(x as isize, y as isize),
                direction,
                intersection_step: 0,
            })
        })
}

struct Cart<'a> {
    map: &'a [&'a [u8]],
    position: Complex<isize>,
    direction: Complex<i8>,
    intersection_step: u8,
}

impl Cart<'_> {
    fn tick<P: PositionSet>(
        &mut self,
        positions: &mut P,
    ) -> Result<Option<P::InsertionResult>, Box<dyn Error>> {
        self.direction = match self.map[self.position.im as usize][self.position.re as usize] {
            b'^' | b'v' | b'|' | b'>' | b'<' | b'-' => self.direction,
            b'+' => {
                let intersection_step = self.intersection_step;
                self.intersection_step = (intersection_step + 1) % 3;
                self.direction * (0..intersection_step).fold(-Complex::i(), |a, _| a * Complex::i())
            }
            b'\\' => {
                self.direction
                    * if self.direction.re == 0 {
                        -Complex::i()
                    } else {
                        Complex::i()
                    }
            }
            b'/' => {
                self.direction
                    * if self.direction.re == 0 {
                        Complex::i()
                    } else {
                        -Complex::i()
                    }
            }
            t => Err(format!("Unrecognized tile {:?}", char::from(t)))?,
        };
        Ok(self.move_cart(positions))
    }

    fn move_cart<P: PositionSet>(&mut self, positions: &mut P) -> Option<P::InsertionResult> {
        assert!(positions.remove(self.position));
        self.position += Complex::new(self.direction.re.into(), self.direction.im.into());
        positions.insert(self.position)
    }
}

trait PositionSet {
    type InsertionResult;
    fn remove(&mut self, p: Complex<isize>) -> bool;
    fn insert(&mut self, p: Complex<isize>) -> Option<Self::InsertionResult>;
}

impl PositionSet for HashSet<Complex<isize>> {
    type InsertionResult = Complex<isize>;
    fn remove(&mut self, p: Complex<isize>) -> bool {
        self.remove(&p)
    }
    fn insert(&mut self, p: Complex<isize>) -> Option<Complex<isize>> {
        if self.insert(p) {
            None
        } else {
            Some(p)
        }
    }
}

struct InsertMap<'a>(usize, &'a mut HashMap<Complex<isize>, usize>);

impl PositionSet for InsertMap<'_> {
    type InsertionResult = (Complex<isize>, usize);
    fn remove(&mut self, p: Complex<isize>) -> bool {
        self.1.remove(&p).is_some()
    }
    fn insert(&mut self, p: Complex<isize>) -> Option<(Complex<isize>, usize)> {
        Some((p, self.1.insert(p, self.0)?))
    }
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY13.part1,
        example1: lines!(
            "|"
            "v"
            "|"
            "|"
            "|"
            "^"
            "|"
        ) => "0,3",
        example2: lines!(
            r"/->-\        "
            r"|   |  /----\"
            r"| /-+--+-\  |"
            r"| | |  | v  |"
            r"\-+-/  \-+--/"
            r"  \------/   "
        ) => "7,3",
        input: "111,13",
    );
    test!(
        DAY13.part2,
        example: lines!(
            r"/>-<\  "
            r"|   |  "
            r"| /<+-\"
            r"| | | v"
            r"\>+</ |"
            r"  |   ^"
            r"  \<->/"
        ) => "6,4",
        input: "16,73",
    );
}
