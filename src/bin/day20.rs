use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{all_consuming, map, opt, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair};
use adventofcode2018::build_main;
use crate::Direction::{East, North, South, West};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pair(isize, isize);

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction { North, South, East, West }

impl Direction {
    fn step_from(&self, pair: Pair) -> Pair {
        let step = match self {
            Direction::North => Pair(-1, 0),
            Direction::South => Pair(1, 0),
            Direction::West => Pair(0, -1),
            Direction::East => Pair(0, 1)
        };

        pair + step
    }
}

#[derive(Clone, Debug)]
struct Regex(Vec<Term>);

#[derive(Clone, Debug)]
enum Term {
    Literal(Direction),
    Branch(Vec<Regex>)
}

fn parse_input(input: &str) -> IResult<&str, Regex> {
    fn term(input: &str) -> IResult<&str, Term> {
        alt((
            value(Term::Literal(North), char('N')),
            value(Term::Literal(South), char('S')),
            value(Term::Literal(East), char('E')),
            value(Term::Literal(West), char('W')),
            map(
                delimited(
                    char('('),
                    pair(separated_list1(char('|'), regex), opt(char('|'))),
                    char(')')
                ),
                |(mut options, empty)| {
                    if empty.is_some() {
                        options.push(Regex(Vec::new()));
                    }

                    Term::Branch(options)
                }
            )
        ))(input)
    }

    fn regex(input: &str) -> IResult<&str, Regex> {
        map(many1(term), Regex)(input)
    }

    all_consuming(delimited(char('^'), regex, char('$')))(input)
}

type Graph = HashMap<Pair, HashSet<Pair>>;

fn build_graph(starts: &HashSet<Pair>, graph: Graph, regex: Regex) -> (HashSet<Pair>, Graph) {
    regex.0.into_iter()
        .fold((starts.clone(), graph), |(ends, mut graph), term| {
            match term {
                Term::Literal(dir) => {
                    let mut new_ends = HashSet::new();
                    for end in ends {
                        let new_end = dir.step_from(end);
                        new_ends.insert(new_end);
                        graph.entry(end).or_default().insert(new_end);
                        graph.entry(new_end).or_default().insert(end);
                    }
                    (new_ends, graph)
                },
                Term::Branch(branches) => {
                    branches.into_iter()
                        .fold((HashSet::new(), graph), |(mut cur, graph), branch| {
                            let (ends, graph) = build_graph(&ends, graph, branch);
                            cur.extend(ends);
                            (cur, graph)
                        })
                }
            }
        })
}

fn part1(input: &str) -> usize {
    let regex = parse_input(input).unwrap().1;
    let starts = HashSet::from([Pair(0, 0)]);
    let (_, graph) = build_graph(&starts, HashMap::new(), regex);

    let mut queue = VecDeque::new();
    queue.push_back((Pair(0, 0), 0));
    let mut best_dist = 0;
    let mut best = Pair(0, 0);
    let mut seen = HashSet::new();
    seen.insert(Pair(0, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if dist > best_dist {
            best_dist = dist;
            best = pos;
        }

        for &nbr in graph[&pos].iter() {
            if !seen.contains(&nbr) {
                seen.insert(nbr);
                queue.push_back((nbr, dist + 1));
            }
        }
    }

    best_dist
}

fn part2(input: &str) -> usize {
    let regex = parse_input(input).unwrap().1;
    let starts = HashSet::from([Pair(0, 0)]);
    let (_, graph) = build_graph(&starts, HashMap::new(), regex);

    let mut queue = VecDeque::new();
    queue.push_back((Pair(0, 0), 0));
    let mut seen = HashSet::new();
    seen.insert(Pair(0, 0));

    let mut result = 0;

    while let Some((pos, dist)) = queue.pop_front() {
        if dist >= 1000 {
            result += 1;
        }

        for &nbr in graph[&pos].iter() {
            if !seen.contains(&nbr) {
                seen.insert(nbr);
                queue.push_back((nbr, dist + 1));
            }
        }
    }

    result
}

build_main!("day20.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "^WNE$";
    const TEST_INPUT2: &str = "^ENWWW(NEEE|SSE(EE|N))$";
    const TEST_INPUT3: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1), 3);
        assert_eq!(part1(TEST_INPUT2), 10);
        assert_eq!(part1(TEST_INPUT3), 18);
    }
}