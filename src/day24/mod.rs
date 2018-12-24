use crate::Solution;
use nom::{
    alpha, alt, call, char, delimited, do_parse, error_position, many0, map_res, named, opt,
    preceded, separated_nonempty_list, tag, take_while, terminated, tuple, tuple_parser,
    types::CompleteStr,
};
use std::cmp::Reverse;
use std::collections::HashSet;
use std::error::Error;

pub(crate) const DAY24: Solution = Solution {
    part1: |input| {
        let mut sides = get_sides(input)?;
        while run_simulation(&mut sides) {}
        for &(a, b) in &[(0, 1), (1, 0)] {
            if sides[a].iter().all(|a| a.units == 0) {
                return Ok(sides[b].iter().map(|b| b.units).sum::<u32>().to_string());
            }
        }
        Err("Neither side won")?
    },
    part2: |input| {
        let sides = get_sides(input)?;
        for boost in 0.. {
            let mut sides = sides.clone();
            for unit in &mut sides[0] {
                unit.attack += boost;
            }
            while run_simulation(&mut sides) {}
            if sides[1].iter().all(|b| b.units == 0) {
                return Ok(sides[0].iter().map(|b| b.units).sum::<u32>().to_string());
            }
        }
        unreachable!()
    },
};

fn get_sides(input: &str) -> Result<[Vec<Army<'_>>; 2], Box<dyn Error + '_>> {
    match sides(CompleteStr(input))? {
        (CompleteStr(""), sides) => Ok(sides),
        _ => Err("Unexpected text after match")?,
    }
}

named!(
    sides(CompleteStr<'_>) -> [Vec<Army<'_>>; 2],
    do_parse!(
        tag!("Immune System:\n")
            >> immune: armies
            >> tag!("\nInfection:\n")
            >> infection: armies
            >> ([immune, infection])
    )
);

named!(
    armies(CompleteStr<'_>) -> Vec<Army<'_>>,
    many0!(terminated!(army, char!('\n')))
);

#[derive(Clone, Debug)]
struct Army<'a> {
    units: u32,
    hit_points: u32,
    weaknesses: HashSet<&'a str>,
    immunities: HashSet<&'a str>,
    attack: u32,
    attack_type: &'a str,
    initiative: u32,
}

named!(
    army(CompleteStr<'_>) -> Army,
    do_parse!(
        units: integer
            >> tag!(" units each with ")
            >> hit_points: integer
            >> tag!(" hit points ")
            >> table: opt!(delimited!(char!('('), table, tag!(") ")))
            >> tag!("with an attack that does ")
            >> attack: integer
            >> char!(' ')
            >> attack_type: alpha
            >> tag!(" damage at initiative ")
            >> initiative: integer
            >> ({
                let (weaknesses, immunities) = table.unwrap_or_default();
                Army {
                    units,
                    hit_points,
                    weaknesses,
                    immunities,
                    attack,
                    attack_type: &attack_type,
                    initiative,
                }
            })
    )
);

named!(
    table(CompleteStr<'_>) -> (HashSet<&str>, HashSet<&str>),
    alt!(
        do_parse!(
            weak: weak >>
            tag!("; ") >>
            immune: immune >>
            (weak, immune)
        ) |
        do_parse!(
            immune: immune >>
            tag!("; ") >>
            weak: weak >>
            (weak, immune)
        ) |
        weak => { |weak| (weak, HashSet::new()) } |
        immune => { |immune| (HashSet::new(), immune) }
    )
);

named!(
    weak(CompleteStr<'_>) -> HashSet<&str>,
    preceded!(tag!("weak to "), list)
);
named!(
    immune(CompleteStr<'_>) -> HashSet<&str>,
    preceded!(tag!("immune to "), list)
);

named!(
    list(CompleteStr<'_>) -> HashSet<&str>,
    do_parse!(
        list: separated_nonempty_list!(tag!(", "), alpha) >> (list.iter().map(|&s| *s).collect())
    )
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> u32,
    map_res!(take_while!(|c| char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

fn run_simulation(sides: &mut [Vec<Army<'_>>; 2]) -> bool {
    let mut attacks: Vec<_> = [(0, 1), (1, 0)]
        .iter()
        .flat_map(|&(attacker, defender)| {
            let attacker_side = &sides[attacker];
            let defender_side = &sides[defender];
            let mut choices: Vec<_> = (0..attacker_side.len())
                .map(|i| (attacker, defender, i, None))
                .collect();
            choices.sort_by_key(|&(_, _, i, _)| {
                let army = &attacker_side[i];
                Reverse((army.units * army.attack, army.initiative))
            });
            let mut selected = HashSet::new();
            for (_, _, i, target) in &mut choices {
                let attack_type = attacker_side[*i].attack_type;
                let attacked = defender_side
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !selected.contains(i))
                    .filter(|(_, d)| d.units != 0)
                    .filter(|(_, d)| !d.immunities.contains(attack_type))
                    .max_by_key(|(_, d)| {
                        (
                            d.weaknesses.contains(attack_type),
                            d.units * d.attack,
                            d.initiative,
                        )
                    })
                    .map(|(i, _)| i);
                *target = attacked;
                if let Some(attacked) = attacked {
                    selected.insert(attacked);
                }
            }
            choices
        })
        .collect();
    attacks.sort_by_key(|&(attacker, _, i, _)| Reverse(sides[attacker][i].initiative));
    let mut progressed = false;
    for (attacker, defender, i, attack) in attacks {
        if sides[attacker][i].units == 0 {
            continue;
        }
        if let Some(attack) = attack {
            let attacker = &sides[attacker][i];
            let mut base_power = attacker.units * attacker.attack;
            let attack_type = attacker.attack_type;
            let defender = &mut sides[defender][attack];
            if defender.weaknesses.contains(attack_type) {
                base_power *= 2;
            }
            let defender_units = defender.units;
            defender.units = defender_units.saturating_sub(base_power / defender.hit_points);
            progressed |= defender_units != defender.units;
        }
    }
    progressed
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY24.part1,
        example: lines!(
            "Immune System:"
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2"
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3"
            ""
            "Infection:"
            "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1"
            "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"
        ) => 5_216,
        input: 20_340,
    );
    test!(
        DAY24.part2,
        example: lines!(
            "Immune System:"
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2"
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3"
            ""
            "Infection:"
            "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1"
            "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"
        ) => 51,
        input: 3_862,
    );
}
