use adventofcode2018::build_main;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, space0, space1};
use nom::combinator::{all_consuming, map, map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pair(isize, isize);

impl Pair {
    fn norm(&self) -> isize {
        self.0*self.0 + self.1*self.1
    }
}

impl Add for Pair {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pair {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Pair(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for Pair {
    type Output = Pair;
    fn mul(self, rhs: isize) -> Self::Output {
        Pair(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Pair> for Pair {
    type Output = isize;
    fn mul(self, rhs: Pair) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

#[derive(Copy, Clone)]
struct Star {
    pos: Pair,
    vel: Pair,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Star>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| s.parse::<isize>(),
        )(input)
    }

    fn parse_pair(input: &str) -> IResult<&str, Pair> {
        map(
            delimited(
                pair(char('<'), space0),
                separated_pair(number, pair(char(','), space0), number),
                char('>')
            ),
            |(x, y)| Pair(x, y)
        )(input)
    }

    fn line(input: &str) -> IResult<&str, Star> {
        map(
            separated_pair(
                preceded(tag("position="), parse_pair),
                space1,
                preceded(tag("velocity="), parse_pair)
            ),
            |(pos, vel)| Star { pos, vel }
        )
        (input)
    }

    all_consuming(separated_list1(newline, line))(input)
}

fn find_message(mut stars: Vec<(Pair, Pair)>) -> (isize, String) {
    let mut best_score = isize::MAX;
    let mut best_arrangement: HashSet<Pair> = HashSet::new();
    let mut t_best = 0;

    // Find when first two points are at minimum distance
    let (p1, v1) = stars[0];
    let (p2, v2) = stars[1];

    /*
     Want (p1+t*v1) and (p2+t*v2) as close as possible. This happens when relative position and
     relative velocity are orthogonal. So, we need (p2-p1+t(v2-v1))*(v2-v1)=0.

     But this is (p2-p1)*(v2-v1)+t(v2-v1)*(v2-v1)=0, or t = -[(p2-p1)(v2-v1)]/[(v2-v1)(v2-v1)].
     */
    let t_mid = -((p2-p1) * (v2-v1)) / ((v2-v1)*(v2-v1));
    let t_min = max(0, t_mid - 200);
    let t_max = t_mid + 200;

    stars = stars.into_iter()
        .map(|(p, v)| (p + v * t_min, v))
        .collect_vec();

    for t in t_min+1..=t_max {
        stars = stars.into_iter().map(|(pos, vel)| (pos + vel, vel)).collect_vec();

        let score = stars.iter().map(|&(p, _)| p).tuple_combinations()
            .map(|(p1, p2)| (p2 - p1).norm())
            .sum();

        if score < best_score {
            best_score = score;
            best_arrangement = stars.iter().map(|&(p, _)| p).collect();
            t_best = t;
        }
    }

    let (i_min, i_max) = best_arrangement.iter()
        .fold((isize::MAX, isize::MIN), |(mut i_min, mut i_max), &Pair(i, _)| {
            i_min = min(i_min, i);
            i_max = max(i_max, i);
            (i_min, i_max)
        });

    let (j_min, j_max) = best_arrangement.iter()
        .fold((isize::MAX, isize::MIN), |(mut j_min, mut j_max), &Pair(_, j)| {
            j_min = min(j_min, j);
            j_max = max(j_max, j);
            (j_min, j_max)
        });

    let s = (j_min..=j_max).map(|j| {
        (i_min..=i_max).map(|i| {
            if best_arrangement.contains(&Pair(i, j)) { '#' } else { ' ' }
        }).collect::<String>()
    }).join("\n");

    (t_best, s)
}

fn part1(input: &str) -> usize {
    let stars = parse_input(input).unwrap().1
        .into_iter()
        .map(|star| (star.pos, star.vel))
        .collect_vec();


    let (_, s) = find_message(stars);
    println!("{s}");

    0
}

fn part2(input: &str) -> isize {
    let stars = parse_input(input).unwrap().1
        .into_iter()
        .map(|star| (star.pos, star.vel))
        .collect_vec();


    let (t, _) = find_message(stars);
    t
}

build_main!("day10.txt", "Part 1" => part1, "Part 2" => part2);