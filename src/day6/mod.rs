use crate::Solution;
use itertools::Itertools;
use nom::types::CompleteStr;
use nom::{call, do_parse, error_position, map_res, named, tag, take_while1};
use std::cmp::Ordering;
use std::error::Error;

pub(super) const DAY6: Solution = Solution {
    part1: |input| {
        let mut points = get_points(input)?;
        let (min_x, max_x) = points
            .iter()
            .map(|p| p.x)
            .minmax()
            .into_option()
            .ok_or("No points")?;
        let (min_y, max_y) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let mut min_distance = i32::max_value();
                let mut min = None;
                for point in &mut points {
                    let distance = (point.x - x).abs() + (point.y - y).abs();
                    match distance.cmp(&min_distance) {
                        Ordering::Less => {
                            min_distance = distance;
                            min = Some(point);
                        }
                        Ordering::Equal => {
                            min = None;
                        }
                        Ordering::Greater => {}
                    }
                }
                if let Some(min) = min {
                    min.count += 1;
                }
            }
        }
        let max_point = points
            .iter()
            .filter(|&&Point { x, y, .. }| x != min_x && x != max_x && y != min_y && y != max_y)
            .map(|p| p.count)
            .max()
            .ok_or("No non-infinite points")?
            .to_string();
        Ok(max_point)
    },
    part2: |input| Ok(find_region_size(input, 10_000)?.to_string()),
};

fn get_points(input: &str) -> Result<Vec<Point>, Box<dyn Error + '_>> {
    input
        .lines()
        .map(|line| {
            let (rest, point) = point(CompleteStr(line))?;
            if rest.is_empty() {
                Ok(point)
            } else {
                Err("Text found in a line after point".into())
            }
        })
        .collect()
}

named!(
    point(CompleteStr<'_>) -> Point,
    do_parse!(x: integer >> tag!(", ") >> y: integer >> (Point { x, y, count: 0 }))
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> i32,
    map_res!(take_while1!(|c| char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

struct Point {
    x: i32,
    y: i32,
    count: u32,
}

fn find_region_size(input: &str, max_total_distance: i32) -> Result<usize, Box<dyn Error + '_>> {
    let points = get_points(input)?;
    let range_modifier = max_total_distance / points.len() as i32;
    let (min_x, max_x) = points
        .iter()
        .map(|p| p.x)
        .minmax()
        .into_option()
        .ok_or("No points")?;
    let (min_y, max_y) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
    let safe_area = (min_x - range_modifier..=max_x + range_modifier)
        .flat_map(|x| (min_y - range_modifier..=max_y + range_modifier).map(move |y| (y, x)))
        .filter(|(x, y)| {
            points
                .iter()
                .map(|p| (p.x - x).abs() + (p.y - y).abs())
                .sum::<i32>()
                < max_total_distance
        })
        .count();
    Ok(safe_area)
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY6.part1,
        example: lines!("1, 1" "1, 6" "8, 3" "3, 4" "5, 5" "8, 9") => 17,
        input: 3969,
    );
    test!(
        DAY6.part2,
        fn test_find_region_size() {
            use crate::day6::find_region_size;
            assert_eq!(find_region_size(lines!("1, 1" "1, 6" "8, 3" "3, 4" "5, 5" "8, 9"), 32).unwrap(), 16);
        }
        input: 42123,
    );
}
