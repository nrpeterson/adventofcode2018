use adventofcode2018::build_main;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, newline};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn parse_input(input: &str) -> IResult<&str, Vec<(char, char)>> {
    all_consuming(
        separated_list1(
            newline,
            delimited(
                tag("Step "),
                separated_pair(anychar, tag(" must be finished before step "), anychar),
                tag(" can begin.")
            )
        )
    )(input)
}

fn part1(input: &str) -> String {
    let mut preds: HashMap<char, Vec<char>> = HashMap::new();

    for (pred, step) in parse_input(input).unwrap().1 {
        preds.entry(pred).or_insert_with(Vec::new);
        preds.entry(step).or_default().push(pred);
    }

    let mut ready = BinaryHeap::new();

    preds.iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(&step, _)| Reverse(step))
        .for_each(|s| ready.push(s));

    let mut result = Vec::new();

    while let Some(Reverse(step)) = ready.pop() {
        result.push(step);

        for (&k, v) in preds.iter_mut() {
            if let Some(i) = v.iter().position(|&s| s == step) {
                v.remove(i);
                if v.is_empty() {
                    ready.push(Reverse(k));
                }
            }
        }
    }

    result.iter().join("")
}

fn time_req(c: char) -> usize {
    (c as usize) + 61 - ('A' as usize)
}

fn part2(input: &str) -> usize {
    let mut preds: HashMap<char, Vec<char>> = HashMap::new();

    for (pred, step) in parse_input(input).unwrap().1 {
        preds.entry(pred).or_insert_with(Vec::new);
        preds.entry(step).or_default().push(pred);
    }

    let mut ready: BinaryHeap<Reverse<(char, usize)>> = BinaryHeap::new();

    preds.iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(&step, _)| Reverse((step, time_req(step))))
        .for_each(|s| ready.push(s));

    let mut time = 0;

    let mut workers = [None; 5];


    loop {
        // See if anybody's done.
        workers.iter_mut().for_each(|worker| {
            if let Some((step, 0)) = *worker {
                for (&k, v) in preds.iter_mut() {
                    if let Some(i) = v.iter().position(|&s| s == step) {
                        v.remove(i);
                        if v.is_empty() {
                            ready.push(Reverse((k, time_req(k))));
                        }
                    }
                }

                *worker = None;
            }
        });

        // Pick up new work if required
        workers.iter_mut().for_each(|worker| {
            if worker.is_none() {
                *worker = ready.pop().map(|Reverse(s)| s);
            }
        });

        // If everybody's idle, we're done here.
        if workers.iter().all(|w| w.is_none()) {
            return time
        }

        // Decrement remaining time
        workers.iter_mut().for_each(|worker| {
            if let Some((_, t)) = worker {
                *t -= 1;
            }
        });

        time += 1;
    }
}

build_main!("day07.txt", "Part 1" => part1, "Part 2" => part2);