use nom::character::complete::{digit1, space1};
use nom::combinator::{flat_map, map, map_res};
use nom::IResult;
use nom::multi::count;
use nom::sequence::{pair, preceded, separated_pair};
use adventofcode2018::build_main;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>
}

impl Node {
    fn metadata_total(&self) -> usize {
        let s = self.metadata.iter().sum::<usize>();
        let r= self.children.iter().map(|c| c.metadata_total()).sum::<usize>();
        s + r
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        }
        else {
            self.metadata.iter()
                .filter(|&&x| x > 0 && x <= self.children.len())
                .map(|&x| self.children[x-1].value())
                .sum::<usize>()
        }
    }
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse::<usize>)(input)
    }

    flat_map(
        separated_pair(number, space1, number),
        |(num_children, num_metadata)| {
            map(
                pair(
                    count(preceded(space1, parse_node), num_children),
                    count(preceded(space1, number), num_metadata)
                ),
                |(children, metadata)| Node { children, metadata })
        }
    )(input)
}

fn part1(input: &str) -> usize {
    let node = parse_node(input).unwrap().1;
    node.metadata_total()
}

fn part2(input: &str) -> usize {
    let node = parse_node(input).unwrap().1;
    node.value()
}

build_main!("day08.txt", "Part 1" => part1, "Part 2" => part2);