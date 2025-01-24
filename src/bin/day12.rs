use adventofcode2018::build_main;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, newline};
use nom::combinator::{map, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

struct Cave {
    has_plant: HashSet<isize>,
    rules: HashMap<u8, bool>,
    min_seen: isize,
    max_seen: isize
}

impl Cave {
    fn get_mask(&self, center: isize) -> u8 {
        (center-2..=center+2).map(|j| if self.has_plant.contains(&j) { 1 } else { 0 })
            .fold(0, |cur, i| 2 * cur + i)
    }

    fn activate(&mut self, i: isize) {
        self.has_plant.insert(i);
        self.min_seen = min(self.min_seen, i);
        self.max_seen = max(self.max_seen, i);
    }

    fn deactivate(&mut self, i: isize) {
        self.has_plant.remove(&i);
    }

    fn update(&mut self) {
        let mut to_activate = Vec::new();
        let mut to_deactivate = Vec::new();
        (self.min_seen - 2..=self.max_seen + 2).for_each(|i| {
            if self.rules[&self.get_mask(i)] {
                to_activate.push(i);
            } else {
                to_deactivate.push(i);
            }
        });

        to_activate.into_iter().for_each(|i| self.activate(i));
        to_deactivate.into_iter().for_each(|i| self.deactivate(i));
    }
}

impl Iterator for Cave {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.has_plant.iter().cloned().sum());
        self.update();
        result
    }
}

fn parse_input(input: &str) -> IResult<&str, Cave> {
    fn initial_state(input: &str) -> IResult<&str, HashSet<isize>> {
        map(
            preceded(
                tag("initial state: "),
                many1(alt((
                    value(true, tag("#")),
                    value(false, tag("."))
                )))
            ),
            |v| v.into_iter().enumerate()
                .filter(|&(_, plant)| plant)
                .map(|(i, _)| i as isize)
                .collect()
        )(input)
    }

    fn rule(input: &str) -> IResult<&str, (u8, bool)> {
        separated_pair(
            map(
                many1(alt((
                    value(true, tag("#")),
                    value(false, tag("."))
                ))),
                |v| v.into_iter().fold(0, |acc, b| {
                    if b { 2 * acc + 1 } else { 2 * acc }
                })
            ),
            tag(" => "),
            alt((
                value(true, tag("#")),
                value(false, tag("."))
            ))
        )(input)
    }

    map(
        separated_pair(
            initial_state,
            multispace1,
            separated_list1(newline, rule)
        ),
        |(has_plant, rules_list)| {
            let rules = rules_list.into_iter().collect();
            let min_seen = *has_plant.iter().min().unwrap();
            let max_seen = *has_plant.iter().max().unwrap();

            Cave { has_plant, rules, min_seen, max_seen }
        }
    )(input)
}

fn part1(input: &str) -> isize {
    parse_input(input).unwrap().1
        .nth(20)
        .unwrap()
}

fn line_coeffs(data: &[isize]) -> Option<(isize, isize)> {
    let b = data[0];
    let a = data[1] - data[0];

    if data.into_iter().enumerate()
        .all(|(i, &value)| value == a * (i as isize) + b) {
        Some((a, b))
    } else { None }
}

fn part2(input: &str) -> isize {
    let cave = parse_input(input).unwrap().1;

    let vals = cave.take(1000).collect_vec();

    let (from, (a, b)) = (0..=900).map(|i| (i, line_coeffs(&vals[i..i+100])))
        .filter_map(|(i, coeffs)| {
            coeffs.map(|cs| (i as isize, cs))
        })
        .next()
        .unwrap();

    a * (50000000000 - from) + b
}

build_main!("day12.txt", "Part 1" => part1, "Part 2" => part2);