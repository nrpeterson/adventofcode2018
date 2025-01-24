use std::collections::HashMap;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{all_consuming, map, map_res, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use adventofcode2018::build_main;
use crate::Event::{BeginsShift, FallsAsleep, WakesUp};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Event {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Timestamp, Event)>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn timestamp(input: &str) -> IResult<&str, Timestamp> {
        map(
            delimited(
                tag("["),
                tuple((number, tag("-"), number, tag("-"), number, space1, number, tag(":"), number)),
                tag("]")
            ),
            |(year, _, month, _, day, _, hour, _, minute)| {
                Timestamp { year, month, day, hour, minute }
            }
        )(input)
    }

    fn event(input: &str) -> IResult<&str, Event> {
        alt((
            value(FallsAsleep, tag("falls asleep")),
            value(WakesUp, tag("wakes up")),
            map(
                delimited(
                    tag("Guard #"),
                    number,
                    tag(" begins shift")
                ),
                BeginsShift
            )
        ))(input)
    }

    fn line(input: &str) -> IResult<&str, (Timestamp, Event)> {
        separated_pair(timestamp, space1, event)(input)
    }

    map(
        all_consuming(separated_list1(newline, line)),
        |mut v| { v.sort(); v }
    )(input)
}

#[derive(Debug)]
struct State {
    cur_guard: usize,
    asleep_since: Option<usize>,
    counts: HashMap<usize, [usize; 60]>
}

fn run(events: &[(Timestamp, Event)]) -> State {
    let init = State { cur_guard: 0, asleep_since: None, counts: HashMap::new() };

    events.into_iter().fold(init, |mut state, (ts, event)| {
        match *event {
            BeginsShift(id) => {
                state.cur_guard = id;
            },
            FallsAsleep => {
                state.asleep_since = Some(ts.minute);
            },
            WakesUp => {
                if let Some(m) = state.asleep_since {
                    let counts = state.counts.entry(state.cur_guard).or_insert([0; 60]);
                    (m..ts.minute).for_each(|i| {
                        counts[i] += 1;
                    });
                }
                state.asleep_since = None;
            }
        }

        state
    })
}

fn part1(input: &str) -> usize {
    let events = parse_input(input).unwrap().1;

    let state = run(&events);

    let (&guard, counts) = state.counts.iter()
        .max_by_key(|(_, v)| v.iter().sum::<usize>())
        .unwrap();

    let minute = counts.iter().position_max().unwrap();

    guard * minute
}

fn part2(input: &str) -> usize {
    let events = parse_input(input).unwrap().1;
    let state = run(&events);

    state.counts.keys().cloned()
        .cartesian_product(0..60)
        .max_by_key(|&(guard, i)| state.counts[&guard][i])
        .map(|(guard, minute)| guard * minute)
        .unwrap()
}

build_main!("day04.txt", "Part 1" => part1, "Part 2" => part2);