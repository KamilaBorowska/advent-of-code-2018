use crate::Solution;
use nom::types::CompleteStr;
use nom::{
    alt, call, delimited, do_parse, error_position, map_res, named, tag, take_while1,
    take_while1_s, tuple_parser,
};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::ops::Range;
use std::vec;

pub(super) const DAY4: Solution = Solution {
    part1: |input| {
        let mut total_asleep_times = HashMap::new();
        let mut asleep_times = HashMap::new();
        let mut parser = LineParser::new(input);
        while let Some(SleepRange { guard, minutes }) = parser.next_sleep_range()? {
            *total_asleep_times.entry(guard).or_insert(0) += minutes.len();
            for minute in minutes {
                *asleep_times
                    .entry(guard)
                    .or_insert_with(HashMap::new)
                    .entry(minute)
                    .or_insert(0) += 1;
            }
        }
        let worst_guard = find_max_value(&total_asleep_times).ok_or("No guards")?;
        let minute = find_max_value(&asleep_times[worst_guard]).unwrap();
        Ok((worst_guard * minute).to_string())
    },
    part2: |input| {
        let mut asleep_times = HashMap::new();
        let mut parser = LineParser::new(input);
        while let Some(SleepRange { guard, minutes }) = parser.next_sleep_range()? {
            for minute in minutes {
                *asleep_times.entry((guard, minute)).or_insert(0) += 1;
            }
        }
        let (minute, guard) = find_max_value(&asleep_times).ok_or("No guards")?;
        Ok((minute * guard).to_string())
    },
};

struct LineParser<'a> {
    iterator: vec::IntoIter<&'a str>,
    current_guard: Option<u32>,
    asleep_start_time: Option<u32>,
}

impl<'a> LineParser<'a> {
    fn new(input: &str) -> LineParser<'_> {
        LineParser {
            iterator: get_sorted_lines_iter(input),
            current_guard: None,
            asleep_start_time: None,
        }
    }

    fn next_sleep_range(&mut self) -> Result<Option<SleepRange>, Box<dyn Error + 'a>> {
        for line in &mut self.iterator {
            let Line { minute, action } = get_action_line(line)?;
            match action {
                Action::BeginsShift { guard } => self.current_guard = Some(guard),
                Action::FallsAsleep => self.asleep_start_time = Some(minute),
                Action::WakesUp => {
                    let current_guard = self.current_guard.ok_or("No guard on shift")?;
                    let current_asleep_time = self.asleep_start_time.ok_or("Guard didn't sleep")?;
                    return Ok(Some(SleepRange {
                        guard: current_guard,
                        minutes: current_asleep_time..minute,
                    }));
                }
            }
        }
        Ok(None)
    }
}

fn get_sorted_lines_iter(input: &str) -> vec::IntoIter<&str> {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();
    lines.into_iter()
}

fn get_action_line(line: &str) -> Result<Line, Box<dyn Error + '_>> {
    let (rest, action_line) = action_line(CompleteStr(line))?;
    if rest.is_empty() {
        Ok(action_line)
    } else {
        Err("Unexpected additional text after an action line")?
    }
}

struct Line {
    minute: u32,
    action: Action,
}

enum Action {
    BeginsShift { guard: u32 },
    FallsAsleep,
    WakesUp,
}

named!(
    action_line(CompleteStr<'_>) -> Line,
    do_parse!(
        tag!("[")
            >> integer
            >> tag!("-")
            >> integer
            >> tag!("-")
            >> integer
            >> tag!(" ")
            >> integer
            >> tag!(":")
            >> minute: integer
            >> tag!("] ")
            >> action: action
            >> (Line { minute, action })
    )
);

named!(
    integer(CompleteStr<'_>) -> u32,
    map_res!(
        take_while1_s!(|c| char::is_digit(c, 10)),
        |x: CompleteStr<'_>| x.parse()
    )
);

named!(
    action(CompleteStr<'_>) -> Action,
    alt!(
        delimited!(tag!("Guard #"), integer, tag!(" begins shift")) => { |guard| Action::BeginsShift { guard } } |
        tag!("falls asleep") => { |_| Action::FallsAsleep } |
        tag!("wakes up") => { |_| Action::WakesUp }
    )
);

struct SleepRange {
    guard: u32,
    minutes: Range<u32>,
}

fn find_max_value<K>(map: &HashMap<K, usize>) -> Option<&K>
where
    K: Hash + Eq,
{
    let (key, _) = map.iter().max_by_key(|&(_, v)| v)?;
    Some(key)
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY4.part1,
        example: lines!(
            "[1518-11-01 00:00] Guard #10 begins shift"
            "[1518-11-01 00:05] falls asleep"
            "[1518-11-01 00:25] wakes up"
            "[1518-11-01 00:30] falls asleep"
            "[1518-11-01 00:55] wakes up"
            "[1518-11-01 23:58] Guard #99 begins shift"
            "[1518-11-02 00:40] falls asleep"
            "[1518-11-02 00:50] wakes up"
            "[1518-11-03 00:05] Guard #10 begins shift"
            "[1518-11-03 00:24] falls asleep"
            "[1518-11-03 00:29] wakes up"
            "[1518-11-04 00:02] Guard #99 begins shift"
            "[1518-11-04 00:36] falls asleep"
            "[1518-11-04 00:46] wakes up"
            "[1518-11-05 00:03] Guard #99 begins shift"
            "[1518-11-05 00:45] falls asleep"
            "[1518-11-05 00:55] wakes up"
        ) => 240,
        input: 87681,
    );
    test!(
        DAY4.part2,
        example: lines!(
            "[1518-11-01 00:00] Guard #10 begins shift"
            "[1518-11-01 00:05] falls asleep"
            "[1518-11-01 00:25] wakes up"
            "[1518-11-01 00:30] falls asleep"
            "[1518-11-01 00:55] wakes up"
            "[1518-11-01 23:58] Guard #99 begins shift"
            "[1518-11-02 00:40] falls asleep"
            "[1518-11-02 00:50] wakes up"
            "[1518-11-03 00:05] Guard #10 begins shift"
            "[1518-11-03 00:24] falls asleep"
            "[1518-11-03 00:29] wakes up"
            "[1518-11-04 00:02] Guard #99 begins shift"
            "[1518-11-04 00:36] falls asleep"
            "[1518-11-04 00:46] wakes up"
            "[1518-11-05 00:03] Guard #99 begins shift"
            "[1518-11-05 00:45] falls asleep"
            "[1518-11-05 00:55] wakes up"
        ) => 4455,
        input: 136461,
    );
}
