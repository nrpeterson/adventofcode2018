use std::ops::Sub;
use itertools::Itertools;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{all_consuming, map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, tuple};
use adventofcode2018::build_main;

struct Point(isize, isize, isize, isize);

impl Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3)
    }
}

impl Point {
    fn norm(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs() + self.3.abs()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| s.parse::<isize>(),
        )(input)
    }

    fn point(input: &str) -> IResult<&str, Point> {
        map(
            tuple((
                number,
                preceded(char(','), number),
                preceded(char(','), number),
                preceded(char(','), number)
            )),
            |(a, b, c, d)| Point(a, b, c, d)
        )(input)
    }

    all_consuming(separated_list1(newline, point))(input)
}

fn part1(input: &str) -> usize {
    let points = parse_input(input).unwrap().1;

    let mut graph: Vec<Vec<usize>> = vec![vec![]; points.len()];

    for (i, j) in (0..points.len()).tuple_combinations() {
        let pi = &points[i];
        let pj = &points[j];

        if (pi - pj).norm() <= 3 {
            graph[i].push(j);
            graph[j].push(i);
        }
    }

    let mut seen = vec![false; points.len()];

    let mut num_components = 0;

    for i in 0..points.len() {
        if seen[i] {
            continue;
        }

        num_components += 1;

        let mut stack = Vec::new();
        stack.push(i);
        seen[i] = true;

        while let Some(j) = stack.pop() {
            for &nbr in &graph[j] {
                if !seen[nbr] {
                    stack.push(nbr);
                    seen[nbr] = true;
                }
            }
        }
    }

    num_components
}

build_main!("day25.txt", "Part 1" => part1);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";

    const TEST_INPUT2: &str = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";

    const TEST_INPUT3: &str = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";

    const TEST_INPUT4: &str = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1), 2);
        assert_eq!(part1(TEST_INPUT2), 4);
        assert_eq!(part1(TEST_INPUT3), 3);
        assert_eq!(part1(TEST_INPUT4), 8);
    }
}