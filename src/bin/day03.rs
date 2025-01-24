use std::cmp::{max, min};
use std::collections::HashSet;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, char as ch, newline};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use adventofcode2018::build_main;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Rectangle {
    x_start: usize,
    y_start: usize,
    x_width: usize,
    y_width: usize
}

impl Rectangle {
    fn x_stop(&self) -> usize {
        self.x_start + self.x_width - 1
    }

    fn y_stop(&self) -> usize {
        self.y_start + self.y_width - 1
    }

    fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        let x_start = max(self.x_start, other.x_start);
        let y_start = max(self.y_start, other.y_start);

        let x_end = min(self.x_stop(), other.x_stop());
        let y_end = min(self.y_stop(), other.y_stop());

        let x_width = (x_end + 1).checked_sub(x_start)?;
        let y_width = (y_end + 1).checked_sub(y_start)?;

        Some(Rectangle { x_start, y_start, x_width, y_width })
    }

    fn coords(&self) -> Vec<(usize, usize)> {
        (self.x_start..=self.x_stop()).cartesian_product(self.y_start..=self.y_stop())
            .collect_vec()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Rectangle>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn rectangle(input: &str) -> IResult<&str, Rectangle> {
        map(
            preceded(
                tuple((ch('#'), digit1, tag(" @ "))),
                tuple((
                    separated_pair(number, ch(','), number),
                    preceded(tag(": "), separated_pair(number, ch('x'), number))
                ))
            ),
            |((x_start, y_start), (x_width, y_width))| {
                Rectangle { x_start, y_start, x_width, y_width }
            }
        )(input)
    }

    separated_list1(newline, rectangle)(input)
}

fn part1(input: &str) -> usize {
    let rectangles = parse_input(input).unwrap().1;

    rectangles.into_iter().tuple_combinations()
        .filter_map(|(r1, r2)| r1.intersection(&r2))
        .flat_map(|r| r.coords())
        .unique()
        .count()
}

fn part2(input: &str) -> usize {
    let rectangles = parse_input(input).unwrap().1;
    let num_rectangles = rectangles.len();

    let bad: HashSet<usize> = rectangles.into_iter().enumerate()
        .map(|(i, r)| (i + 1, r))
        .tuple_combinations()
        .flat_map(|((i1, r1), (i2, r2))| {
            if r1.intersection(&r2).is_some() {
                vec![i1, i2]
            }
            else {
                vec![]
            }
        })
        .collect();

    (1..=num_rectangles).filter(|&i| !bad.contains(&i)).nth(0).unwrap()
}

build_main!("day03.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

        assert_eq!(part1(input), 4);
    }
}