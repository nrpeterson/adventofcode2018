use std::collections::VecDeque;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use adventofcode2018::build_main;
use crate::Line::{Horizontal, Vertical};

struct Level {
    data: Vec<Vec<char>>,
    y_min: usize,
    queue: VecDeque<(usize, usize)>
}

impl Level {
    fn from_lines(lines: &[Line]) -> Level {
        let x_min = lines.iter()
            .map(|line| {
                match line {
                    &Horizontal(_, (x_min, _)) => x_min,
                    &Vertical(_, x) => x
                }
            })
            .min().unwrap() - 1;

        let x_max = lines.iter()
            .map(|line| {
                match line {
                    &Horizontal(_, (_, x_max)) => x_max,
                    &Vertical(_, x) => x
                }
            })
            .max().unwrap() + 1;

        let y_min = lines.iter()
            .map(|line| {
                match line {
                    &Horizontal(y, _) => y,
                    &Vertical((y_min, _), _) => y_min
                }
            })
            .min().unwrap();

        let y_max = lines.iter()
            .map(|line| {
                match line {
                    &Horizontal(y, _) => y,
                    &Vertical((_, y_max), _) => y_max
                }
            })
            .max().unwrap();

        let mut data = (0..=y_max).map(|_| {
            vec!['.'; x_max - x_min + 1]
        }).collect_vec();

        data[0][500-x_min] = '+';

        lines.iter().for_each(|line| {
            match line {
                &Horizontal(y, (x0, x1)) => {
                    (x0..=x1).for_each(|x| data[y][x-x_min] = '#');
                },
                &Vertical((y0, y1), x) => {
                    (y0..=y1).for_each(|y| data[y][x-x_min] = '#');
                }
            }
        });

        let mut queue = VecDeque::new();
        queue.push_front((0, 500-x_min));

        Level { data, y_min, queue }
    }

    fn run(&mut self) {
        while let Some((y, x)) = self.queue.pop_front() {
            match self.data[y][x] {
                '+' | '|' => {
                    if y + 1 == self.data.len() {
                        continue;
                    }

                    match self.data[y+1][x] {
                        '.' => {
                            let mut y0 = y;
                            while y0 + 1 < self.data.len() && !"~#".contains(self.data[y0+1][x]) {
                                y0 += 1;
                            }

                            (y+1..=y0).for_each(|y1| self.data[y1][x] = '|');
                            self.queue.push_back((y0, x));
                        },
                        '#' | '~' => {
                            let mut l = x;
                            while l > 0 && self.data[y][l-1] != '#'
                                && "#~".contains(self.data[y+1][l]) {
                                l -= 1;
                            }

                            let mut r = x;
                            while r + 1 < self.data[y].len() && self.data[y][r+1] != '#'
                                &&  "#~".contains(self.data[y+1][r]) {
                                r += 1;
                            }

                            let left_is_wall = l > 0 && self.data[y][l-1] == '#';
                            let right_is_wall = r + 1 < self.data[y].len()
                                && self.data[y][r+1] == '#';

                            if left_is_wall && right_is_wall {
                                (l..=r).for_each(|x| {
                                    self.data[y][x] = '~';
                                    self.queue.push_back((y - 1, x));
                                });
                            }
                            else {
                                (l..=r).for_each(|x| self.data[y][x] = '|');
                                if !left_is_wall {
                                    self.queue.push_back((y, l));
                                }
                                if !right_is_wall {
                                    self.queue.push_back((y, r));
                                }
                            }
                        },
                        _ => continue
                    }
                },
                _ => continue
            };
            // self.print();
            // println!();
        }
    }
}

#[derive(Copy, Clone)]
enum Line {
    Horizontal(usize, (usize, usize)),
    Vertical((usize, usize), usize)
}

fn parse_input(input: &str) -> IResult<&str, Level> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn horizontal(input: &str) -> IResult<&str, Line> {
        map(
            separated_pair(
                preceded(tag("y="), number),
                tag(", "),
                preceded(tag("x="), separated_pair(number, tag(".."), number))
            ),
            |(y, (x_min, x_max))| Horizontal(y, (x_min, x_max))
        )(input)
    }

    fn vertical(input: &str) -> IResult<&str, Line> {
        map(
            separated_pair(
                preceded(tag("x="), number),
                tag(", "),
                preceded(tag("y="), separated_pair(number, tag(".."), number))
            ),
            |(x, (y_min, y_max))| Vertical((y_min, y_max), x)
        )(input)
    }

    map(
        separated_list1(newline, alt((horizontal, vertical))),
        |lines| Level::from_lines(&lines)
    )(input)
}

fn part1(input: &str) -> usize {
    let mut level = parse_input(input).unwrap().1;
    level.run();

    level.data[level.y_min..].iter()
        .flat_map(|row| row)
        .filter(|&&c| c == '|' || c == '~')
        .count()
}

fn part2(input: &str) -> usize {
    let mut level = parse_input(input).unwrap().1;
    level.run();

    level.data[level.y_min..].iter()
        .flat_map(|row| row)
        .filter(|&&c| c == '~')
        .count()
}

build_main!("day17.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 57);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}

