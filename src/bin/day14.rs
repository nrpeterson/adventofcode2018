use itertools::Itertools;
use adventofcode2018::build_main;

struct Kitchen {
    scoreboard: Vec<u8>,
    elf1_cur: usize,
    elf2_cur: usize,
    i: usize
}

impl Kitchen {
    fn step(&mut self) {
        let e1_score = self.scoreboard[self.elf1_cur];
        let e2_score = self.scoreboard[self.elf2_cur];
        let new_score = e1_score + e2_score;

        if new_score < 10 {
            self.scoreboard.push(new_score);
        }
        else {
            self.scoreboard.push(new_score / 10);
            self.scoreboard.push(new_score % 10);
        }

        self.elf1_cur += 1 + (e1_score as usize);
        self.elf1_cur %= self.scoreboard.len();
        self.elf2_cur += 1 + (e2_score as usize);
        self.elf2_cur %= self.scoreboard.len();
    }
}

impl Iterator for Kitchen {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i >= self.scoreboard.len() {
            self.step();
        }

        let result = Some(self.scoreboard[self.i]);
        self.i += 1;

        result
    }
}

struct UntilMatch<I> {
    target: Vec<u8>,
    it: I,
    matches: Vec<bool>
}

impl<I> Iterator for UntilMatch<I> where I: Iterator<Item=u8> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.it.next()?;

        if self.matches[self.target.len() - 1] {
            return None;
        }

        let mut new_matches = vec![false; self.matches.len()];

        if c == self.target[0] {
            new_matches[0] = true;
        }

        for i in 0..self.target.len() - 1 {
            if self.matches[i] && c == self.target[i + 1] {
                new_matches[i+1] = true;
            }
        }

        self.matches = new_matches;

        Some(c)
    }
}

fn part1(input: &str) -> String {
    let target = input.parse::<usize>().unwrap();
    let kitchen = Kitchen { scoreboard: vec![3, 7], elf1_cur: 0, elf2_cur: 1, i: 0 };
    kitchen.dropping(target).take(10).map(|d| d.to_string()).join("")
}

fn part2(input: &str) -> usize {
    let target = input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect_vec();
    let n = target.len();

    let kitchen = Kitchen { scoreboard: vec![3, 7], elf1_cur: 0, elf2_cur: 1, i: 0 };
    let until_target = UntilMatch { target: target.clone(), it: kitchen, matches: vec![false; n]};

    until_target.enumerate()
        .last()
        .unwrap()
        .0 - n + 1
}

build_main!("day14.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("9"), "5158916779");
        assert_eq!(part1("5"), "0124515891");
        assert_eq!(part1("18"), "9251071085");
        assert_eq!(part1("2018"), "5941429882");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }
}