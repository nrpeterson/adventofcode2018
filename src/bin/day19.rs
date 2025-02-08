use adventofcode2018::build_main;
use adventofcode2018::elf::parse_machine;

fn part1(input: &str) -> usize {
    let mut machine = parse_machine(input).unwrap().1;
    machine.last().unwrap()[0]
}

fn part2(input: &str) -> usize {
    let mut machine = parse_machine(input).unwrap().1;
    machine.registers[0] = 1;
    machine.is_break[1] = true;

    let r = machine.last().unwrap();

    (1..=r[2]).filter(|&n| r[2] % n == 0).sum()
}

build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
}