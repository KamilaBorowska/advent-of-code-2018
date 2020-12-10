use crate::Solution;
use regex_syntax::hir::{Anchor, Group, Hir, HirKind, Literal};
use regex_syntax::ParserBuilder;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;

pub(crate) const DAY20: Solution = Solution {
    part1: |input| {
        let hir = ParserBuilder::new()
            .nest_limit(1_000)
            .build()
            .parse(input)?;
        let hir = get_regex_without_anchors(&hir)?;
        let maze = Maze::from_regex_hir(hir)?;
        Ok(maze.find_furthest_room()?.to_string())
    },
    part2: |input| {
        let hir = ParserBuilder::new()
            .nest_limit(1_000)
            .build()
            .parse(input)?;
        let hir = get_regex_without_anchors(&hir)?;
        let maze = Maze::from_regex_hir(hir)?;
        Ok(maze
            .count_rooms_with_shortest_path_through_at_least_1000_doors()?
            .to_string())
    },
};

fn get_regex_without_anchors(hir: &Hir) -> Result<&[Hir], Box<dyn Error>> {
    if let HirKind::Concat(hirs) = hir.kind() {
        let (start, hirs) = hirs.split_first().unwrap();
        if let HirKind::Anchor(Anchor::StartText) = start.kind() {
        } else {
            return Err("Regular expression doesn't start with ^".into());
        }
        let (end, hirs) = hirs.split_last().unwrap();
        if let HirKind::Anchor(Anchor::EndText) = end.kind() {
        } else {
            return Err("Regular expression doesn't end with $".into());
        }
        Ok(hirs)
    } else {
        Err("Regular expression is too short".into())
    }
}

#[derive(Debug)]
struct Maze {
    rooms: HashMap<(i32, i32), Room>,
}

impl Maze {
    fn from_regex_hir(concat_hirs: &[Hir]) -> Result<Self, Box<dyn Error>> {
        let mut maze = Self {
            rooms: HashMap::new(),
        };
        maze.put_hir_from_concat_iter(concat_hirs, (0, 0), |_, _| Ok(()))?;
        Ok(maze)
    }

    fn put_hir_from_concat_iter(
        &mut self,
        concat_hirs: &[Hir],
        position: (i32, i32),
        mut callback: impl FnMut(&mut Self, (i32, i32)) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        self.put_hir_from_concat_iter_inner(&mut concat_hirs.iter(), position, &mut callback)
    }

    fn put_hir_from_concat_iter_inner<'a>(
        &mut self,
        iterator: &mut impl Iterator<Item = &'a Hir>,
        position: (i32, i32),
        callback: &mut impl FnMut(&mut Self, (i32, i32)) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(hir) = iterator.next() {
            let callback = &mut *callback;
            self.put_hir(hir, position, &mut move |maze, position| {
                maze.put_hir_from_concat_iter_inner(iterator, position, callback)
            })?;
        }
        callback(self, position)
    }

    fn put_hir(
        &mut self,
        hir: &Hir,
        position: (i32, i32),
        callback: &mut (dyn FnMut(&mut Self, (i32, i32)) -> Result<(), Box<dyn Error>>),
    ) -> Result<(), Box<dyn Error>> {
        match hir.kind() {
            HirKind::Empty => callback(self, position),
            &HirKind::Literal(Literal::Unicode(c)) => {
                let new_position = self.put_letter(c, position)?;
                callback(self, new_position)
            }
            HirKind::Group(Group { hir, .. }) => self.put_hir(hir, position, callback),
            HirKind::Alternation(hirs) => {
                let mut cache = HashSet::new();
                for hir in hirs {
                    self.put_hir(hir, position, &mut |maze, position| {
                        if cache.insert(position) {
                            callback(maze, position)?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            }
            HirKind::Concat(hirs) => self.put_hir_from_concat_iter(&hirs, position, callback),
            _ => Err(format!("Unexpected HIR kind: {}", hir).into()),
        }
    }

    fn put_letter(
        &mut self,
        letter: char,
        position: (i32, i32),
    ) -> Result<(i32, i32), Box<dyn Error>> {
        let mut new_position = position;
        match letter {
            'W' => new_position.0 -= 1,
            'E' => new_position.0 += 1,
            'N' => new_position.1 += 1,
            'S' => new_position.1 -= 1,
            _ => return Err(format!("Unexpected letter {:?}", letter).into()),
        }
        self.rooms
            .entry(new_position)
            .or_default()
            .doors
            .insert(position);
        self.rooms
            .entry(position)
            .or_default()
            .doors
            .insert(new_position);
        Ok(new_position)
    }

    fn find_furthest_room(&self) -> Result<usize, Box<dyn Error>> {
        let mut to_check = VecDeque::new();
        to_check.push_back((0, 0));
        let mut visited = HashSet::new();
        let mut path_length = 0;
        let mut last_of_level = 1;
        while let Some(position) = to_check.pop_front() {
            last_of_level -= 1;
            if visited.insert(position) {
                for &room in &self.rooms[&position].doors {
                    to_check.push_back(room);
                }
            }
            if visited.len() == self.rooms.len() {
                return Ok(path_length);
            }
            if last_of_level == 0 {
                path_length += 1;
                last_of_level = to_check.len();
            }
        }
        Err("Not all points are reachable".into())
    }

    fn count_rooms_with_shortest_path_through_at_least_1000_doors(
        &self,
    ) -> Result<usize, Box<dyn Error>> {
        let mut to_check = VecDeque::new();
        to_check.push_back((0, 0));
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let mut path_length = 0;
        let mut last_of_level = 1;
        while let Some(position) = to_check.pop_front() {
            last_of_level -= 1;
            visited.insert(position);
            if visited.len() == self.rooms.len() {
                return Ok(0);
            }
            for &room in &self.rooms[&position].doors {
                if !visited.insert(room) {
                    continue;
                }
                to_check.push_back(room);
            }
            if last_of_level == 0 {
                path_length += 1;
                if path_length == 1000 {
                    return Ok(self.rooms.len() - visited.len() + to_check.len());
                }
                last_of_level = to_check.len();
            }
        }
        Err("Not all points are reachable".into())
    }
}

#[derive(Debug, Default)]
struct Room {
    doors: HashSet<(i32, i32)>,
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY20.part1,
        example1: "^WNE$" => 3,
        example2: "^ENWWW(NEEE|SSE(EE|N))$" => 10,
        example3: "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$" => 18,
        example4: "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$" => 23,
        example5: "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$" => 31,
        input: 3_739,
    );
    test!(
        DAY20.part2,
        input: 8_409,
    );
}
