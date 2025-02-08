use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::sequence::{preceded, separated_pair};
use adventofcode2018::build_main;
use crate::Gear::{ClimbingGear, Neither, Torch};
use crate::Terrain::*;

#[derive(Copy, Clone)]
enum Terrain {
    Rocky,
    Wet,
    Narrow
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Gear {
    Neither,
    Torch,
    ClimbingGear
}

impl Gear {
    fn works_for(&self, terrain: Terrain) -> bool {
        match (*self, terrain) {
            (Neither, Rocky) | (Torch, Wet) | (ClimbingGear, Narrow) => false,
            _ => true
        }
    }
}

const GEAR: [Gear; 3] = [Neither, Torch, ClimbingGear];


#[derive(Copy, Clone)]
struct Metrics {
    erosion_level: usize,
    risk_level: usize
}

fn neighbors(point: (usize, usize)) -> Vec<(usize, usize)> {
    let (y, x) = point;
    let mut results = vec![(y, x + 1), (y + 1, x)];
    if y > 0 {
        results.push((y - 1, x));
    }
    if x > 0 {
        results.push((y, x - 1));
    }

    results
}

type State = ((usize, usize), Gear);

struct Cave {
    cache: HashMap<(usize, usize), Metrics>,
    target: (usize, usize),
    depth: usize
}

impl Cave {
    fn metrics(&mut self, (y, x): (usize, usize)) -> &Metrics {
        if !self.cache.contains_key(&(y, x)) {
            let geologic_index = match (y, x) {
                (0, 0) => 0,
                c if c == self.target => 0,
                (0, x) => x * 16807,
                (y, 0) => y * 48271,
                (y, x) => {
                    self.erosion_level((y-1, x)) * self.erosion_level((y, x-1))
                }
            };

            let erosion_level = (geologic_index + self.depth) % 20183;
            let risk_level = erosion_level % 3;

            self.cache.insert((y, x), Metrics { erosion_level, risk_level });
        }

        &self.cache[&(y, x)]
    }

    fn erosion_level(&mut self, point: (usize, usize)) -> usize {
        self.metrics(point).erosion_level
    }

    fn risk_level(&mut self, point: (usize, usize)) -> usize {
        self.metrics(point).risk_level
    }

    fn terrain(&mut self, point: (usize, usize)) -> Terrain {
        match self.risk_level(point) {
            0 => Terrain::Rocky,
            1 => Terrain::Wet,
            _ => Terrain::Narrow
        }
    }

    fn edges(&mut self, from: State) -> Vec<(State, usize)> {
        let (point, gear) = from;
        let mut result = Vec::new();
        let cur_terrain = self.terrain(point);

        GEAR.iter()
            .filter(|&&g| g != gear && g.works_for(cur_terrain))
            .for_each(|&g| {
                result.push(((point, g), 7));
            });

        neighbors(point).into_iter()
            .filter(|&nbr| gear.works_for(self.terrain(nbr)))
            .for_each(|nbr| result.push(((nbr, gear), 1)));

        result
    }
}

fn dijkstra(cave: &mut Cave, start: State, target: State) -> usize {
    let mut dists: HashMap<State, usize> = HashMap::new();

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), start));

    while let Some((Reverse(dist), state)) = q.pop() {
        let cur_dist = dists.entry(state).or_insert(usize::MAX);

        if dist < *cur_dist {
            if state == target {
                return dist;
            }

            *cur_dist = dist;
            for (nbr, wt) in cave.edges(state) {
                if dist + wt < *dists.entry(nbr).or_insert(usize::MAX) {
                    q.push((Reverse(dist + wt), nbr));
                }
            }
        }
    }

    usize::MAX
}

fn parse_input(input: &str) -> IResult<&str, Cave> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    map(
        separated_pair(
            preceded(tag("depth: "), number),
            newline,
            preceded(tag("target: "), separated_pair(number, tag(","), number))
        ),
        |(depth, (x, y))| Cave { cache: HashMap::new(), target: (y, x), depth }
    )(input)
}

fn part1(input: &str) -> usize {
    let mut cave = parse_input(input).unwrap().1;
    let (y, x) = cave.target;

    let result = (0..=y).cartesian_product(0..=x)
        .map(|p| cave.risk_level(p))
        .sum();

    result
}

fn part2(input: &str) -> usize {
    let mut cave = parse_input(input).unwrap().1;
    let start = ((0, 0), Torch);
    let end = (cave.target, Torch);

    dijkstra(&mut cave, start, end)
}

build_main!("day22.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use crate::{part1};

    #[test]
    fn test_part1() {
        assert_eq!(part1("depth: 510\ntarget: 10,10\n"), 114);
    }
}