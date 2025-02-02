use std::collections::HashMap;
use std::ops::Add;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use adventofcode2018::build_main;

#[derive(Copy, Clone)]
struct Pair(i8, i8);
impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const DIRECTIONS: [Pair; 8] = [
    Pair(-1, -1), Pair(-1, 0), Pair(-1, 1),
    Pair(0, -1), Pair(0, 1),
    Pair(1, -1), Pair(1, 0), Pair(1, 1)
];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Space { Empty, Tree, LumberYard }

#[derive(Clone, Eq, PartialEq, Hash)]
struct Level {
    data: Vec<Vec<Space>>
}

impl Level {
    fn score(&self) -> usize {
        let mut trees = 0;
        let mut lumberyards = 0;
        self.data.iter().flatten().for_each(|&space| {
            match space {
                Space::Tree => trees += 1,
                Space::LumberYard => lumberyards += 1,
                Space::Empty => ()
            }
        });

        trees * lumberyards
    }

    fn get(&self,  Pair(i, j): Pair) -> Option<Space> {
        if i < 0 || j < 0 {
            return None;
        }

        Some(*self.data.get(i as usize)?.get(j as usize)?)
    }

    fn neighbors(&self, loc: Pair) -> Vec<Space> {
        DIRECTIONS.iter()
            .filter_map(|&dir| self.get(loc + dir))
            .collect_vec()
    }

    fn round(&mut self) {
        self.data = self.data.iter().enumerate()
            .map(|(i, row)| {
                row.iter().enumerate()
                    .map(|(j, &space)| {
                        let neighbors = self.neighbors(Pair(i as i8, j as i8));
                        match space {
                            Space::Empty => {
                                let trees = neighbors.into_iter()
                                    .filter(|s| *s == Space::Tree)
                                    .count();

                                if trees >= 3 { Space::Tree } else { Space::Empty }
                            },
                            Space::Tree => {
                                let lumberyards = neighbors.into_iter()
                                    .filter(|s| *s == Space::LumberYard)
                                    .count();

                                if lumberyards >= 3 { Space::LumberYard } else { Space::Tree }
                            },
                            Space::LumberYard => {
                                let lumberyards = neighbors.iter()
                                    .filter(|s| **s == Space::LumberYard)
                                    .count();
                                let trees = neighbors.into_iter()
                                    .filter(|s| *s == Space::Tree)
                                    .count();

                                if lumberyards >= 1 && trees >= 1 { Space::LumberYard } else { Space::Empty }
                            }
                        }
                    }).collect()
            }).collect()
    }
}

impl Iterator for Level {
    type Item = Level;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.clone());
        self.round();
        result
    }
}

fn parse_input(input: &str) -> IResult<&str, Level> {
    fn space(input: &str) -> IResult<&str, Space> {
        alt((
            value(Space::Empty, tag(".")),
            value(Space::Tree, tag("|")),
            value(Space::LumberYard, tag("#"))
        ))(input)
    }

    map(
        separated_list1(newline, many1(space)),
        |data| Level { data }
    )(input)
}

fn part1(input: &str) -> usize {
    let mut level = parse_input(input).unwrap().1;
    level.nth(10).unwrap().score()
}

fn part2(input: &str) -> usize {
    let level = parse_input(input).unwrap().1;

    let mut seen = HashMap::new();
    let mut cycle_start = 0;
    let mut cycle_len = 0;
    let mut scores = Vec::new();

    for (i, lvl) in level.enumerate() {
        if seen.contains_key(&lvl) {
            cycle_start = seen[&lvl];
            cycle_len = i - cycle_start;
            break;
        }
        else {
            scores.push(lvl.score());
            seen.insert(lvl, i);
        }
    }

    let target = 1000000000;
    let i = (target - cycle_start) % cycle_len;

    scores[cycle_start + i]
}

build_main!("day18.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1147);
    }
}