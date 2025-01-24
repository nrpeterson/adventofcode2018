use std::collections::HashMap;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adventofcode2018::build_main;

fn parse_input(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    all_consuming(
        separated_list1(
            newline,
            separated_pair(number, tag(", "), number)
        )
    )(input)
}

fn part1(input: &str) -> usize {
    let points = parse_input(input).unwrap().1;

    let mut components: HashMap<(usize, usize), (usize, bool)> = HashMap::new();

    let i_max = points.iter().map(|&(i, _)| i).max().unwrap();
    let j_max = points.iter().map(|&(_, j)| j).max().unwrap();

    (0..=i_max).cartesian_product(0..=j_max)
        .for_each(|(i, j)| {
            let closest = points.iter().cloned()
                .min_set_by_key(|&(p_i, p_j)| p_i.abs_diff(i) + p_j.abs_diff(j));

            if closest.len() == 1 {
                let p = closest[0];
                let entry = components.entry(p).or_insert((0, true));
                entry.0 += 1;

                if i == 0 || j == 0 || i == i_max || j == j_max {
                    entry.1 = false;
                }
            }
        });

    components.into_iter()
        .filter(|(_, (_, is_finite))| *is_finite)
        .map(|(_, (count, _))| count)
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let points = parse_input(input).unwrap().1;

    let i_sum = points.iter().map(|&(i, _)| i).sum::<usize>();
    let j_sum = points.iter().map(|&(_, j)| j).sum::<usize>();

    let i_max = (10000 + i_sum) / points.len();
    let j_max = (10000 + j_sum) / points.len();

    (0..=i_max).cartesian_product(0..=j_max)
        .filter(|&(i, j)| {
            points.iter()
                .map(|&(p_i, p_j)| p_i.abs_diff(i) + p_j.abs_diff(j))
                .sum::<usize>() < 10000
        })
        .count()
}

build_main!("day06.txt", "Part 1" => part1, "Part 2" => part2);