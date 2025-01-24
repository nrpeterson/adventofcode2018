use adventofcode2018::build_main;
use std::array;

fn conjugate(c: char) -> char {
    if c.is_ascii_uppercase() {
        ((c as u8) + ('a' as u8 - 'A' as u8)) as char
    }
    else {
        ((c as u8) - ('a' as u8 - 'A' as u8)) as char
    }
}

fn index(c: char) -> usize {
    if c.is_ascii_uppercase() {
        (c as usize) - ('A' as usize)
    }
    else {
        (c as usize) - ('a' as usize)
    }
}

fn part1(input: &str) -> usize {
    let mut stack = Vec::new();

    for c in input.chars() {
        if let Some(last) = stack.last() {
            if *last == conjugate(c) {
                stack.pop();
                continue
            }
        }

        stack.push(c)
    }

    stack.len()
}

fn part2(input: &str) -> usize {
    let mut stacks: [Vec<char>; 26] = array::from_fn(|_| Vec::new());

    for c in input.chars() {
        for i in 0..26 {
            if i == index(c) {
                continue;
            }

            if let Some(last) = stacks[i].last() {
                if *last == conjugate(c) {
                    stacks[i].pop();
                    continue;
                }
            }
            stacks[i].push(c);
        }
    }

    stacks.into_iter().map(|v| v.len()).min().unwrap()
}

build_main!("day05.txt", "Part 1" => part1, "Part 2" => part2);