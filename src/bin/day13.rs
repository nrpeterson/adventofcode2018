use std::collections::{BTreeMap, HashSet};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char as ch, newline};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use adventofcode2018::build_main;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction { Up, Right, Down, Left }
use Direction::*;

impl Direction {
    fn apply(&self, (i, j): (usize, usize)) -> (usize, usize) {
        match self {
            Up => (i - 1, j),
            Right => (i, j + 1),
            Down => (i + 1, j),
            Left => (i, j - 1)
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

#[derive(Copy, Clone)]
enum Track {
    Empty,
    Vertical,
    Horizontal,
    CurveNE,
    CurveNW,
    Intersection
}
use Track::*;

struct System {
    tracks: Vec<Vec<Track>>,
    carts: BTreeMap<(usize, usize), (Direction, u8)>
}

impl System {
    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut first_collision = None;
        let mut removed = HashSet::new();

        let old_carts = self.carts.iter()
            .map(|(&pos, &(dir, turn))| (pos, (dir, turn)))
            .collect_vec();

        for ((i, j), (dir, turn)) in old_carts {
            if removed.contains(&(i, j)) {
                continue;
            }

            let (new_pos, (new_dir, new_turn)) = match self.tracks[i][j] {
                Empty => unreachable!(),
                Horizontal | Vertical => (dir.apply((i, j)), (dir, turn)),
                CurveNE => {
                    let new_dir = match dir {
                        Up => Right,
                        Right => Up,
                        Down => Left,
                        Left => Down
                    };
                    let new_pos = new_dir.apply((i, j));
                    (new_pos, (new_dir, turn))
                },
                CurveNW => {
                    let new_dir = match dir {
                        Up => Left,
                        Right => Down,
                        Down => Right,
                        Left => Up
                    };
                    let new_pos = new_dir.apply((i, j));
                    (new_pos, (new_dir, turn))
                },
                Intersection => {
                    let new_dir = match turn {
                        0 => dir.turn_left(),
                        1 => dir,
                        _ => dir.turn_right()
                    };

                    let new_turn = (turn + 1) % 3;
                    let new_pos = new_dir.apply((i, j));
                    (new_pos, (new_dir, new_turn))
                }
            };

            self.carts.remove(&(i, j));

            match self.carts.insert(new_pos, (new_dir, new_turn)) {
                None => (),
                Some(_) => {
                    first_collision = first_collision.or(Some(new_pos));
                    removed.insert(new_pos);
                    self.carts.remove(&new_pos);
                }
            }
        }

        first_collision
    }

    fn print(&self) {
        let mut arr = self.tracks.iter()
            .map(|row| {
                row.iter().map(|t| {
                    match t {
                        Empty => ' ',
                        Horizontal => '-',
                        Vertical => '|',
                        CurveNE => '/',
                        CurveNW => '\\',
                        Intersection => '+'
                    }
                }).collect_vec()
            }).collect_vec();

        for (&(i, j), &(dir, _)) in self.carts.iter() {
            arr[i][j] = match dir {
                Up => '^',
                Right => '>',
                Down => 'v',
                Left => '<'
            }
        }

        let s = arr.into_iter()
            .map(|row| row.into_iter().join(""))
            .join("\n");

        println!("{s}");
    }
}

fn parse_input(input: &str) -> IResult<&str, System> {
    fn spot(input: &str) -> IResult<&str, (Option<Direction>, Track)> {
        alt((
            value((None, Horizontal), ch('-')),
            value((Some(Right), Horizontal), ch('>')),
            value((Some(Left), Horizontal), ch('<')),
            value((None, Vertical), ch('|')),
            value((Some(Up), Vertical), ch('^')),
            value((Some(Down), Vertical), ch('v')),
            value((None, CurveNE), ch('/')),
            value((None, CurveNW), ch('\\')),
            value((None, Intersection), ch('+')),
            value((None, Empty), ch(' '))
        ))(input)
    }

    map(
        separated_list1(newline, many1(spot)),
        |v| {
            let tracks = v.iter()
                .map(|row| row.iter().map(|(_, track)| *track).collect_vec())
                .collect_vec();

            let mut carts = BTreeMap::new();

            v.iter().enumerate().for_each(|(i, row)|
                row.iter().enumerate().for_each(|(j, &(dir_opt, _))| {
                    if let Some(dir) = dir_opt {
                        carts.insert((i, j), (dir, 0));
                    }
                })
            );

            System { tracks, carts }
        }
    )(input)
}

fn part1(input: &str) -> String {
    let mut system = parse_input(input).unwrap().1;

    loop {
        match system.tick() {
            None => continue,
            Some((i, j)) => return format!("{j},{i}")
        }
    }
}

fn part2(input: &str) -> String {
    let mut system = parse_input(input).unwrap().1;

    loop {
        system.tick();
        if system.carts.len() == 1 {
            let (i, j) = *system.carts.first_key_value().unwrap().0;
            return format!("{j},{i}");
        }
    }
}

build_main!("day13.txt", "Part 1" => part1, "Part 2" => part2);