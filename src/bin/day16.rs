use std::collections::HashSet;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1, newline, space1};
use nom::combinator::{all_consuming, map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use adventofcode2018::build_main;

#[derive(Copy, Clone)]
struct Instruction {
    op_code: usize,
    a: usize,
    b: usize,
    c: usize
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Operation {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}
use Operation::*;

const OPERATIONS: [Operation; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
];

impl Operation {
    fn apply(&self, a: usize, b: usize, c: usize, registers: &[usize; 4]) -> [usize; 4] {
        let mut result = registers.clone();

        match self {
            Addr => result[c] = result[a] + result[b],
            Addi => result[c] = result[a] + b,
            Mulr => result[c] = result[a] * result[b],
            Muli => result[c] = result[a] * b,
            Banr => result[c] = result[a] & result[b],
            Bani => result[c] = result[a] & b,
            Borr => result[c] = result[a] | result[b],
            Bori => result[c] = result[a] | b,
            Setr => result[c] = result[a],
            Seti => result[c] = a,
            Gtir => result[c] = if a > result[b] { 1 } else { 0 },
            Gtri => result[c] = if result[a] > b { 1 } else { 0 },
            Gtrr => result[c] = if result[a] > result[b] { 1 } else { 0 },
            Eqir => result[c] = if a == result[b] { 1 } else { 0 },
            Eqri => result[c] = if result[a] == b { 1 } else { 0 },
            Eqrr => result[c] = if result[a] == result[b] { 1 } else { 0 }
        };

        result
    }
}

struct Sample {
    before: [usize; 4],
    instruction: Instruction,
    after: [usize; 4]
}


fn parse_input(input: &str) -> IResult<&str, (Vec<Sample>, Vec<Instruction>)> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn registers(input: &str) -> IResult<&str, [usize; 4]> {
        map(
            delimited(
                tag("["),
                tuple((
                    terminated(number, tag(", ")),
                    terminated(number, tag(", ")),
                    terminated(number, tag(", ")),
                    number
                )),
                tag("]")
            ),
            |(a, b, c, d)| [a, b, c, d]
        )(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        map(
            separated_list1(space1, number),
            |v| Instruction { op_code: v[0], a: v[1], b: v[2], c: v[3] }
        )(input)
    }

    fn sample(input: &str) -> IResult<&str, Sample> {
        map(
            tuple((
                delimited(tag("Before: "), registers, newline),
                terminated(instruction, newline),
                delimited(tag("After:  "), registers, newline)
            )),
            |(before, instruction, after)| {
                Sample { before, instruction, after }
            }
        )(input)
    }

    all_consuming(
        separated_pair(
            separated_list1(newline, sample),
            multispace1,
            separated_list1(newline, instruction)
        )
    )(input)
}

fn mappings(sample: &Sample, so_far: &[Option<Operation>; 16]) -> Vec<[Option<Operation>; 16]> {
    let Sample { before, instruction, after } = sample;
    let &Instruction { op_code, a, b, c } = instruction;

    match so_far[op_code] {
        Some(op) => {
            if op.apply(a, b, c, before) == *after {
                vec![so_far.clone()]
            }
            else {
                vec![]
            }
        },
        None => {
            let mut results = Vec::new();
            let seen: HashSet<Operation> = so_far.iter()
                .filter_map(|v| *v)
                .collect();

            for op in OPERATIONS.iter() {
                if seen.contains(op) {
                    continue
                }
                if op.apply(a, b, c, before) == *after {
                    let mut option = so_far.clone();
                    option[op_code] = Some(*op);
                    results.push(option);
                }
            }
            results
        }
    }

}

fn part1(input: &str) -> usize {
    let (samples, _) = parse_input(input).unwrap().1;
    let empty_mapping = [None; 16];
    samples.into_iter()
        .map(|s| mappings(&s, &empty_mapping).len())
        .filter(|&n| n >= 3)
        .count()
}

fn part2(input: &str) -> usize {
    let (mut samples, program) = parse_input(input).unwrap().1;

    let empty_mapping = [None; 16];
    samples.sort_by_key(|s| mappings(s, &empty_mapping).len());

    let options = samples.iter()
        .fold(vec![empty_mapping], |cur, next| {
            cur.into_iter()
                .flat_map(|opt| mappings(next, &opt))
                .collect_vec()
        });

    assert_eq!(options.len(), 1);
    let mapping = options[0].map(|v| v.unwrap());

    let result = program.into_iter()
        .fold([0; 4], |acc, instr| {
            let Instruction { op_code, a, b, c } = instr;
            let op = mapping[op_code];
            op.apply(a, b, c, &acc)
        });

    result[0]
}

build_main!("day16.txt", "Part 1" => part1, "Part 2" => part2);