use std::collections::VecDeque;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::{separated_pair, terminated};
use adventofcode2018::build_main;

fn parse_input(input: &str) -> IResult<&str, (usize, usize)> {
    // 419 players; last marble is worth 71052 points
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    separated_pair(
        number,
        tag(" players; last marble is worth "),
        terminated(number, tag(" points"))
    )(input)
}

fn play(players: usize, marbles: usize) -> usize {
    let mut scores = vec![0; players + 1];
    let mut circle = VecDeque::from([1]);

    for marble in 1..=marbles {
        if marble % 23 != 0 {
            circle.rotate_left(2 % circle.len());
            circle.push_front(marble);
        }
        else {
            let player = marble % players;
            scores[player] += marble;
            circle.rotate_right(7);
            scores[player] += circle.pop_front().unwrap();
        }
    }

    scores.into_iter().max().unwrap()
}

fn part1(input: &str) -> usize {
    let (players, marbles) = parse_input(input).unwrap().1;
    play(players, marbles)
}

fn part2(input: &str) -> usize {
    let (players, marbles) = parse_input(input).unwrap().1;
    play(players, marbles * 100)
}

build_main!("day09.txt", "Part 1" => part1, "Part 2" => part2);