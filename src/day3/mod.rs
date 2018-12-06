use crate::Solution;
use failure::{bail, err_msg};
use nom::types::CompleteStr;
use nom::{call, do_parse, error_position, map_res, named, tag, take_while1, take_while1_s};
use std::collections::HashMap;

pub(super) const DAY3: Solution = Solution {
    part1: |input| {
        Ok(get_claim_table(input)?
            .values()
            .filter(|&&s| s == ClaimState::More)
            .count()
            .to_string())
    },
    part2: |input| {
        let claim_table = get_claim_table(input)?;
        for claim in get_claims(input) {
            let claim = claim?;
            if get_squares(&claim).all(|square| claim_table[&square] == ClaimState::Once) {
                return Ok(claim.num.to_string());
            }
        }
        bail!("No non-overlapping claims");
    },
};

fn get_claim_table(input: &str) -> Result<HashMap<(u16, u16), ClaimState>, failure::Error> {
    let mut claimed = HashMap::new();
    for claim in get_claims(input) {
        for square in get_squares(&claim?) {
            claimed
                .entry(square)
                .and_modify(|s| *s = ClaimState::More)
                .or_insert(ClaimState::Once);
        }
    }
    Ok(claimed)
}

fn get_claims(input: &str) -> impl Iterator<Item = Result<Claim, failure::Error>> + '_ {
    input.lines().map(|line| {
        let (rest, claim) = claim(CompleteStr(line)).map_err(|_| err_msg("Parse failure"))?;
        if rest.is_empty() {
            Ok(claim)
        } else {
            bail!("Unexpected additional text after a claim")
        }
    })
}

fn get_squares(
    &Claim {
        position_x,
        position_y,
        area_x,
        area_y,
        ..
    }: &Claim,
) -> impl Iterator<Item = (u16, u16)> {
    (0..area_x).flat_map(move |x| (0..area_y).map(move |y| (position_x + x, position_y + y)))
}

struct Claim {
    num: u16,
    position_x: u16,
    position_y: u16,
    area_x: u16,
    area_y: u16,
}

named!(
    claim(CompleteStr<'_>) -> Claim,
    do_parse!(
        tag!("#")
            >> num: integer
            >> tag!(" @ ")
            >> position_x: integer
            >> tag!(",")
            >> position_y: integer
            >> tag!(": ")
            >> area_x: integer
            >> tag!("x")
            >> area_y: integer
            >> (Claim {
                num,
                position_x,
                position_y,
                area_x,
                area_y
            })
    )
);

named!(
    integer<CompleteStr<'_>, u16>,
    map_res!(
        take_while1_s!(|c| char::is_digit(c, 10)),
        |x: CompleteStr<'_>| x.parse()
    )
);

#[derive(Copy, Clone, PartialEq, Eq)]
enum ClaimState {
    Once,
    More,
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY3.part1,
        empty: "" => 0,
        example1: "#123 @ 3,2: 5x4" => 0,
        example2: lines!("#1 @ 1,3: 4x4" "#2 @ 3,1: 4x4" "#3 @ 5,5: 2x2") => 4,
        input: 124850,
    );
    test!(
        DAY3.part2,
        example1: "#123 @ 3,2: 5x4" => 123,
        example2: lines!("#1 @ 1,3: 4x4" "#2 @ 3,1: 4x4" "#3 @ 5,5: 2x2") => 3,
        input: 1097,
    );
}
