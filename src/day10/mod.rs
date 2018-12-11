use crate::Solution;
use itertools::Itertools;
use regex::Regex;
use std::error::Error;

const FONT_HEIGHT: i32 = 10;

pub(super) const DAY10: Solution = Solution {
    part1: |input| Ok(run_simulation(input)?.0),
    part2: |input| Ok(run_simulation(input)?.1.to_string()),
};

fn run_simulation(input: &str) -> Result<(String, usize), Box<dyn Error>> {
    let mut particles = get_particles(input)?;
    for i in 0.. {
        let (min_y, max_y) = particles
            .iter()
            .map(|p| p.position_y)
            .minmax()
            .into_option()
            .unwrap();
        if max_y - min_y + 1 == FONT_HEIGHT {
            let (min_x, max_x) = particles
                .iter()
                .map(|p| p.position_x)
                .minmax()
                .into_option()
                .unwrap();
            let mut grid = vec![[false; FONT_HEIGHT as usize]; (max_x - min_x + 1) as usize];
            let mut output = String::from("\n");
            for p in &particles {
                grid[(p.position_x - min_x) as usize][(p.position_y - min_y) as usize] = true;
            }
            for y in 0..FONT_HEIGHT {
                for x in &grid {
                    output.push(if x[y as usize] { '#' } else { '.' });
                }
                output.push('\n');
            }
            return Ok((output, i));
        }
        for particle in &mut particles {
            particle.tick();
        }
    }
    unreachable!()
}

fn get_particles(input: &str) -> Result<Vec<Particle>, Box<dyn Error>> {
    let regex = Regex::new(
        r"(?x)
            ^
            position=< \s* ( -? \d+ ) , \s* ( -? \d+ ) > \s*
            velocity=< \s* ( -? \d+ ) , \s* ( -? \d+ ) >
            $
        ",
    )
    .unwrap();
    input
        .lines()
        .map(|line| {
            let caps = regex.captures(line).ok_or("Match failure")?;
            Ok(Particle {
                position_x: caps[1].parse().unwrap(),
                position_y: caps[2].parse().unwrap(),
                velocity_x: caps[3].parse().unwrap(),
                velocity_y: caps[4].parse().unwrap(),
            })
        })
        .collect()
}

struct Particle {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Particle {
    fn tick(&mut self) {
        self.position_x += self.velocity_x;
        self.position_y += self.velocity_y;
    }
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY10.part2,
        input: 10_003,
    );
}
