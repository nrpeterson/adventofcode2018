use adventofcode2018::build_main;
use std::collections::HashSet;

fn char_counts(input: &str) -> [usize; 26] {
    let mut counts = [0; 26];
    for c in input.chars() {
        let ord = c as usize - 'a' as usize;
        counts[ord] += 1;
    }

    counts
}

fn part1(input: &str) -> usize {
    let (count2, count3) = input.lines()
        .fold((0, 0), |(mut cur2, mut cur3), next| {
            let counts = char_counts(next);
            if counts.iter().any(|&c| c == 2) {
                cur2 += 1;
            }
            if counts.iter().any(|&c| c == 3) {
                cur3 += 1;
            }
                (cur2, cur3)
        });

    count2 * count3
}

fn part2(input: &str) -> String {
    let mut seen = HashSet::new();

    for line in input.lines() {
        for c in 0..line.len() {
            let mut s = line.to_owned();
            s.replace_range(c..c+1, "*");

            if seen.contains(&s) {
                s.remove(c);
                return s;
            }
            else {
                seen.insert(s);
            }
        }
    }

    unreachable!()
}

build_main!("day02.txt", "Part 1" => part1, "Part 2" => part2);