use crate::Solution;
use enumset::{EnumSet, EnumSetType};
use std::collections::hash_map::{Entry, HashMap};
use std::collections::{HashSet, VecDeque};
use std::fmt::{self, Debug, Formatter};

pub(crate) const DAY15: Solution = Solution {
    part1: |input| {
        let mut game = Game::new(input, 3);
        let mut rounds = 0;
        while game.take_turns() {
            rounds += 1;
        }
        Ok((rounds
            * game
                .players
                .iter()
                .map(|p| u32::from(p.hit_points))
                .sum::<u32>())
        .to_string())
    },
    part2: |input| {
        'checking_powers: for power in 4..=200 {
            let mut game = Game::new(input, power);
            let mut rounds = 0;
            while game.take_turns() {
                rounds += 1;
            }
            if game
                .players
                .iter()
                .any(|p| p.race == Race::Elf && p.hit_points == 0)
            {
                continue 'checking_powers;
            }
            println!(
                "{} {} {}",
                power,
                rounds,
                game.players
                    .iter()
                    .map(|p| u32::from(p.hit_points))
                    .sum::<u32>()
            );
            return Ok((rounds
                * game
                    .players
                    .iter()
                    .map(|p| u32::from(p.hit_points))
                    .sum::<u32>())
            .to_string());
        }
        Err("Even an instant-kill elf won't stop the goblins")?
    },
};

struct Game<'a> {
    board: Board<'a>,
    players: Vec<Player>,
}

impl Game<'_> {
    fn new(input: &str, elves_attack_power: u8) -> Game<'_> {
        let board: Vec<_> = input.lines().map(|x| x.as_bytes()).collect();
        let mut players = Vec::new();
        for (y, line) in board.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let race = match tile {
                    b'G' => Race::Goblin,
                    b'E' => Race::Elf,
                    _ => continue,
                };
                players.push(Player {
                    race,
                    position: Position { x, y },
                    hit_points: 200,
                    attack_power: if race == Race::Elf {
                        elves_attack_power
                    } else {
                        3
                    },
                })
            }
        }
        Game {
            board: Board { board },
            players,
        }
    }

    fn take_turns(&mut self) -> bool {
        self.players.sort_by_key(|p| p.position);
        let mut positions = self.get_positions();
        for player in 0..self.players.len() {
            if self.players[player].hit_points == 0 {
                continue;
            }
            let mut map = EnumSet::new();
            for player in &self.players {
                if player.hit_points != 0 {
                    map.insert(player.race);
                }
            }
            if map.len() != 2 {
                return false;
            }
            self.take_turn(&mut positions, player);
        }
        true
    }

    fn get_positions(&self) -> HashMap<Position, usize> {
        let mut map = HashMap::new();
        for (i, p) in self
            .players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.hit_points != 0)
        {
            assert_eq!(map.insert(p.position, i), None);
        }
        map
    }

    fn take_turn(&mut self, positions: &mut HashMap<Position, usize>, player_number: usize) {
        let mut player = &self.players[player_number];
        let mut target = self.find_nearby_target(positions, player);
        if target.is_none() {
            self.move_to_nearest(positions, player_number);
            player = &self.players[player_number];
            target = self.find_nearby_target(positions, player);
        }
        if let Some(target) = target {
            self.hurt(positions, target, player.attack_power);
        }
    }

    fn find_nearby_target(
        &self,
        positions: &HashMap<Position, usize>,
        player: &Player,
    ) -> Option<usize> {
        player
            .position
            .nearby()
            .filter_map(|position| positions.get(&position))
            .cloned()
            .filter(|&other_player| self.players[other_player].race != player.race)
            .min_by_key(|&other_player| self.players[other_player].hit_points)
    }

    fn move_to_nearest(
        &mut self,
        cached_positions: &mut HashMap<Position, usize>,
        player_number: usize,
    ) {
        let player = &self.players[player_number];
        let positions = self
            .players
            .iter()
            .filter(|p| p.race != player.race && p.hit_points != 0)
            .flat_map(|p| p.get_reachable_positions(&self.board, cached_positions))
            .collect();
        if let Some(new_position) =
            self.board
                .breadth_scan(player.position, cached_positions, &positions)
        {
            assert_eq!(
                cached_positions.remove(&player.position),
                Some(player_number)
            );
            cached_positions.insert(new_position, player_number);
            self.players[player_number].position = new_position;
        }
    }

    fn hurt(
        &mut self,
        positions: &mut HashMap<Position, usize>,
        other_player: usize,
        attack_power: u8,
    ) {
        let mut other_player = &mut self.players[other_player];
        other_player.hit_points = other_player.hit_points.saturating_sub(attack_power);
        if other_player.hit_points == 0 {
            positions.remove(&other_player.position);
        }
    }
}

impl Debug for Game<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let positions = self.get_positions();
        f.debug_list()
            .entries(self.board.board.iter().enumerate().map(|(y, line)| {
                let mut output = String::new();
                let mut players = Vec::new();
                for (x, &tile) in line.iter().enumerate() {
                    output.push(if tile == b'#' {
                        '#'
                    } else if let Some(&player) = positions.get(&Position { x, y }) {
                        let player = &self.players[player];
                        let letter = match player.race {
                            Race::Elf => 'E',
                            Race::Goblin => 'G',
                        };
                        players.push(format!("{}({})", letter, player.hit_points));
                        letter
                    } else {
                        '.'
                    });
                }
                format!("{}    {}", output, players.join(", "))
            }))
            .finish()
    }
}

struct Board<'a> {
    board: Vec<&'a [u8]>,
}

impl Board<'_> {
    fn is_passable(&self, position: Position, cached_positions: &HashMap<Position, usize>) -> bool {
        self.board[position.y][position.x] != b'#' && !cached_positions.contains_key(&position)
    }

    fn breadth_scan(
        &self,
        position: Position,
        cached_positions: &HashMap<Position, usize>,
        targets: &HashSet<Position>,
    ) -> Option<Position> {
        let mut scanned = HashMap::new();
        scanned.insert(position, None);
        let mut to_scan = VecDeque::new();
        to_scan.push_back(position);
        let mut found_targets = Vec::new();
        let mut last_of_level = 1;
        while let Some(original_position) = to_scan.pop_front() {
            last_of_level -= 1;
            for position in original_position.nearby() {
                if self.is_passable(position, cached_positions) {
                    if let Entry::Vacant(vacant) = scanned.entry(position) {
                        vacant.insert(Some(original_position));
                        to_scan.push_back(position);
                        if targets.contains(&position) {
                            found_targets.push(position);
                        }
                    }
                }
            }
            if last_of_level == 0 {
                if !found_targets.is_empty() {
                    let mut prefinal_position = None;
                    let mut final_position = *found_targets.iter().min().unwrap();
                    while let Some(previous_position) = scanned[&final_position] {
                        prefinal_position = Some(final_position);
                        final_position = previous_position;
                    }
                    return prefinal_position;
                }
                last_of_level = to_scan.len();
            }
        }
        None
    }
}

#[derive(Debug)]
struct Player {
    race: Race,
    position: Position,
    hit_points: u8,
    attack_power: u8,
}

impl Player {
    fn get_reachable_positions<'a>(
        &self,
        board: &'a Board<'_>,
        cached_positions: &'a HashMap<Position, usize>,
    ) -> impl Iterator<Item = Position> + 'a {
        self.position
            .nearby()
            .filter(move |&p| board.is_passable(p, cached_positions))
    }
}

#[derive(Debug, EnumSetType)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn nearby(self) -> impl Iterator<Item = Position> {
        let Position { x, y } = self;
        vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
            .into_iter()
            .map(|(x, y)| Position { x, y })
    }
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY15.part1,
        maze_world: lines!(
            "##########"
            "#G.......#"
            "########.#"
            "#EEG#....#"
            "##.##.##.#"
            "#...#..#.#"
            "#.####.#.#"
            "#......#.#"
            "##########"
        ) => 11_346,
        fn attack_opponent_with_lowest_hp() {
            use crate::day15::Game;
            let mut game = Game::new(lines!(
                "#######"
                "#G....#"
                "#..G..#"
                "#.EEG.#"
                "#..G..#"
                "#...G.#"
                "#######"
            ), 3);
            use crate::day15::Race::{Elf, Goblin};
            assert_eq!(game.players.iter().map(|p| p.race).collect::<Vec<_>>(), [Goblin, Goblin, Elf, Elf, Goblin, Goblin, Goblin]);
            let hps = [9, 4, 1, 200, 3, 3, 2];
            for (player, &hp) in game.players.iter_mut().zip(&hps) {
                player.hit_points = hp;
            }
            let mut positions = game.get_positions();
            game.take_turn(&mut positions, 3);
            assert_eq!(game.players[4].hit_points, 0);
            game.take_turn(&mut positions, 3);
            assert_eq!(game.players[5].hit_points, 0);
            game.take_turn(&mut positions, 3);
            assert_eq!(game.players[1].hit_points, 1);
        }
        fn multiple_paths() {
            use crate::day15::{Game, Position};
            let mut game = Game::new(lines!(
                "#####"
                "#G.G#"
                "#.E.#"
                "#G.G#"
                "#####"
            ), 3);
            game.take_turn(&mut game.get_positions(), 2);
            assert_eq!(game.players[2].position, Position { x: 2, y: 1 })
        }
        fn multiple_paths_2x2() {
            use crate::day15::{Game, Position};
            let mut game = Game::new(lines!(
                "#######"
                "#G...G#"
                "#.....#"
                "#..E..#"
                "#.....#"
                "#G...G#"
                "#######"
            ), 3);
            let mut positions = game.get_positions();
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 3, y: 2 });
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 3, y: 1 });
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 2, y: 1 });
        }
        fn multiple_paths_with_blocker() {
            use crate::day15::{Game, Position};
            let mut game = Game::new(lines!(
                "#######"
                "#G#..G#"
                "#.....#"
                "#..E..#"
                "#.....#"
                "#G...G#"
                "#######"
            ), 3);
            let mut positions = game.get_positions();
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 3, y: 2 });
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 3, y: 1 });
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 4, y: 1 });
        }
        fn multiple_paths_with_blocker_in_middle() {
            use crate::day15::{Game, Position};
            let mut game = Game::new(lines!(
                "#######"
                "#G.#.G#"
                "#.....#"
                "#..E..#"
                "#.....#"
                "#G...G#"
                "#######"
            ), 3);
            let mut positions = game.get_positions();
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 3, y: 2 });
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 2, y: 2 });
            game.take_turn(&mut positions, 2);
            assert_eq!(game.players[2].position, Position { x: 2, y: 1 });
        }
        example1: lines!(
            "#######"
            "#.G...#"
            "#...EG#"
            "#.#.#G#"
            "#..G#E#"
            "#.....#"
            "#######"
        ) => 27_730,
        example2: lines!(
            "#######"
            "#G..#E#"
            "#E#E.E#"
            "#G.##.#"
            "#...#E#"
            "#...E.#"
            "#######"
        ) => 36_334,
        example3: lines!(
            "#######"
            "#E..EG#"
            "#.#G.E#"
            "#E.##E#"
            "#G..#.#"
            "#..E#.#"
            "#######"
        ) => 39_514,
        example4: lines!(
            "#######"
            "#E.G#.#"
            "#.#G..#"
            "#G.#.G#"
            "#G..#.#"
            "#...E.#"
            "#######"
        ) => 27_755,
        example5: lines!(
            "#######"
            "#.E...#"
            "#.#..G#"
            "#.###.#"
            "#E#G#G#"
            "#...#G#"
            "#######"
        ) => 28_944,
        example6: lines!(
            "#########"
            "#G......#"
            "#.E.#...#"
            "#..##..G#"
            "#...##..#"
            "#...#...#"
            "#.G...G.#"
            "#.....G.#"
            "#########"
        ) => 18_740,
        input: 222_831,
    );
    test!(
        DAY15.part2,
        example1: lines!(
            "#######"
            "#.G...#"
            "#...EG#"
            "#.#.#G#"
            "#..G#E#"
            "#.....#"
            "#######"
        ) => 4_988,
        example2: lines!(
            "#######"
            "#E..EG#"
            "#.#G.E#"
            "#E.##E#"
            "#G..#.#"
            "#..E#.#"
            "#######"
        ) => 31_284,
        example3: lines!(
            "#######"
            "#E.G#.#"
            "#.#G..#"
            "#G.#.G#"
            "#G..#.#"
            "#...E.#"
            "#######"
        ) => 3_478,
        example4: lines!(
            "#######"
            "#.E...#"
            "#.#..G#"
            "#.###.#"
            "#E#G#G#"
            "#...#G#"
            "#######"
        ) => 6_474,
        example5: lines!(
            "#########"
            "#G......#"
            "#.E.#...#"
            "#..##..G#"
            "#...##..#"
            "#...#...#"
            "#.G...G.#"
            "#.....G.#"
            "#########"
        ) => 1_140,
        input: 54_096,
    );
}
