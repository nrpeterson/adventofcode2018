use std::collections::HashSet;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use adventofcode2018::build_main;

fn part1(input: &str) -> isize {
    input.lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sum()
}

fn part2(input: &str) -> isize {
    input.lines()
        .map(|line| line.parse::<isize>().unwrap())
        .cycle()
        .fold_while((HashSet::from([0]), 0), |(mut seen, cur), next| {
            let freq = cur + next;
            if !seen.insert(freq) {
                Done((seen, freq))
            }
            else {
                Continue((seen, freq))
            }
        }).into_inner().1
}

build_main!("day01.txt", "Part 1" => part1, "Part 2" => part2);