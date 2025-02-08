use std::collections::HashSet;
use adventofcode2018::build_main;
use adventofcode2018::elf::parse_machine;

fn part1(input: &str) -> usize {
    let mut machine = parse_machine(input).unwrap().1;
    machine.is_break[28] = true;
    let rs = machine.last().unwrap();
    rs[1]
}

fn part2(input: &str) -> usize {
    let mut machine = parse_machine(input).unwrap().1;
    let mut seen = HashSet::new();
    let mut prev = 0;

    for rs in machine {
        if rs[5] == 28 {
            if seen.contains(&rs[1]) {
                return prev;
            }
            seen.insert(rs[1]);
            prev = rs[1];
        }
    }

    unreachable!()
}

build_main!("day21.txt", "Part 1" => part1, "Part 2" => part2);