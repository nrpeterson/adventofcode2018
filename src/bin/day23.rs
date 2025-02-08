use adventofcode2018::build_main;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{all_consuming, map, map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashSet};
use std::ops::{Add, Div, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Pos(isize, isize, isize);

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Div<isize> for Pos {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Pos(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Pos {
    fn norm(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Nanobot {
    pos: Pos,
    radius: isize
}

fn parse_input(input: &str) -> IResult<&str, Vec<Nanobot>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| s.parse::<isize>()
        )(input)
    }

    fn position(input: &str) -> IResult<&str, Pos> {
        map(
            delimited(
                tag("pos=<"),
                tuple((
                    number,
                    preceded(char(','), number),
                    preceded(char(','), number)
                )),
                char('>')
            ),
            |(a, b, c)| Pos(a, b, c)
        )(input)
    }

    fn nanobot(input: &str) -> IResult<&str, Nanobot> {
        map(
            separated_pair(
                position,
                tag(", r="),
                number
            ),
            |(pos, radius)| Nanobot { pos, radius }
        )(input)
    }

    all_consuming(separated_list1(newline, nanobot))(input)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Box { c1: Pos, c2: Pos }

impl Box {
    fn new(mut c1: Pos, mut c2: Pos) -> Self {
        if c2 < c1 {
            (c1, c2) = (c2, c1);
        }

        Box { c1, c2 }
    }

    fn min_dist(&self, to: &Pos) -> isize {
        let x0 = min(self.c1.0, self.c2.0);
        let x1 = max(self.c1.0, self.c2.0);
        let y0 = min(self.c1.1, self.c2.1);
        let y1 = max(self.c1.1, self.c2.1);
        let z0 = min(self.c1.2, self.c2.2);
        let z1 = max(self.c1.2, self.c2.2);

        let x = if x0 <= to.0 && to.0 <= x1 { to.0 }
            else { [x0, x1].into_iter().min_by_key(|&x| x.abs_diff(to.0)).unwrap() };

        let y = if y0 <= to.1 && to.1 <= y1 { to.1 }
            else { [y0, y1].into_iter().min_by_key(|&y| y.abs_diff(to.1)).unwrap() };

        let z = if z0 <= to.2 && to.2 <= z1 { to.2 }
            else { [z0, z1].into_iter().min_by_key(|&z| z.abs_diff(to.2)).unwrap() };

        (Pos(x, y, z) - *to).norm()
    }

    fn midpoint(&self) -> Pos {
       self.c1 + (self.c2 - self.c1) / 2
    }

    fn subdivide(&self) -> Vec<Box> {
        let m = self.midpoint();

        let x0 = min(self.c1.0, self.c2.0);
        let x1 = max(self.c1.0, self.c2.0);
        let y0 = min(self.c1.1, self.c2.1);
        let y1 = max(self.c1.1, self.c2.1);
        let z0 = min(self.c1.2, self.c2.2);
        let z1 = max(self.c1.2, self.c2.2);

        let corners = [
            (x0, y0, z0), (x0, y0, z1), (x0, y1, z0), (x0, y1, z1),
            (x1, y0, z0), (x1, y0, z1), (x1, y1, z0), (x1, y1, z1)
        ];

        corners.into_iter().unique()
            .map(|(x, y, z)| Pos(x, y, z))
            .map(|p| Box::new(p, m))
            .collect_vec()

    }

    fn intersections(&self, bots: &[Nanobot]) -> usize {
        bots.iter()
            .filter(|n| self.min_dist(&n.pos) <= n.radius)
            .count()
    }
}

fn num_in_range(bots: &[Nanobot], point: Pos) -> usize {
    bots.iter()
        .filter(|bot| (bot.pos - point).norm() <= bot.radius)
        .count()
}

fn part1(input: &str) -> usize {
    let nanobots = parse_input(input).unwrap().1;

    let best = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    nanobots.iter()
        .filter(|&n| (n.pos - best.pos).norm() <= best.radius)
        .count()
}

fn part2(input: &str) -> isize {
    let nanobots = parse_input(input).unwrap().1;

    let m = nanobots.iter()
        .flat_map(|n| [n.pos.0 - n.radius, n.pos.1 - n.radius, n.pos.2 - n.radius])
        .min()
        .unwrap();

    let n = nanobots.iter()
        .flat_map(|n| [n.pos.0 + n.radius, n.pos.1 + n.radius, n.pos.2 + n.radius])
        .max()
        .unwrap();

    let full_box = Box::new(Pos(m, m, m), Pos(n, n, n));

    let mut queue = BinaryHeap::new();
    queue.push((full_box.intersections(&nanobots), full_box));

    let mut best_score = 0;
    let mut best_orig_dist = isize::MAX;
    let mut seen: HashSet<Box> = HashSet::new();
    seen.insert(full_box);

    while let Some((intersections, b)) = queue.pop() {
        if intersections < best_score {
            continue;
        }

        let mid = b.midpoint();
        let s = num_in_range(&nanobots, mid);
        if s > best_score || s == best_score && mid.norm() < best_orig_dist {
            best_score = s;
            best_orig_dist = mid.norm();
        }

        for b0 in b.subdivide() {
            let b0_intersections = b0.intersections(&nanobots);

            if !seen.contains(&b0) && b0_intersections >= best_score {
                queue.push((b0_intersections, b0));
                seen.insert(b0);
            }
        }
    }

    best_orig_dist
}

build_main!("day23.txt", "Part 1" => part1, "Part 2" => part2);