use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ops::{Add, Index};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char as ch, newline};
use nom::combinator::{all_consuming, map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use adventofcode2018::build_main;
use crate::Race::{Elf, Goblin};
use crate::Step::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Pair(isize, isize);

impl Add for Pair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

const UP: Pair = Pair(-1, 0);
const DOWN: Pair = Pair(1, 0);
const LEFT: Pair = Pair(0, -1);
const RIGHT: Pair = Pair(0, 1);
const DIRECTIONS: [Pair; 4] = [UP, LEFT, RIGHT, DOWN];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Race { Elf, Goblin }

#[derive(Copy, Clone, Debug)]
struct Warrior {
    hp: usize,
    race: Race,
    attack_power: usize,
    position: Pair
}

enum Step {
    StartRound(usize),
    Movement { warrior_id: usize, from: Pair, to: Option<Pair> },
    Attack {
        warrior_id: usize,
        position: Pair,
        target_id: Option<usize>,
        target_position: Option<Pair>,
        target_final_hp: Option<usize>
    },
    Done {
        completed_rounds: usize,
        total_hp: usize
    }
}

#[derive(Copy, Clone)]
enum Phase { Move, Attack, Done }

#[derive(Clone)]
struct Level {
    is_wall: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    warriors: Vec<Warrior>,
    positions: HashMap<Pair, usize>,
    cur_round: usize,
    turn_order: VecDeque<usize>,
    phase: Phase
}

impl Level {
    fn print(&self) {
        let s = (0..self.rows).map(|i| {
            (0..self.cols).map(|j| {
                let pos = Pair(i as isize, j as isize);

                if self.is_wall[i][j] { '#' }
                else if let Some(&warrior_id) = self.positions.get(&pos) {
                    match self.warriors[warrior_id].race {
                        Elf => 'E',
                        Goblin => 'G'
                    }
                }
                else { '.' }
            }).collect::<String>()
        }).join("\n");
        println!("{s}");
        println!();
    }

    fn is_open(&self, pos: Pair) -> bool {
        0 <= pos.0 && pos.0 < self.rows as isize && 0 <= pos.1 && pos.1 < self.cols as isize
            && !self.positions.contains_key(&pos) && !self.is_wall[pos.0 as usize][pos.1 as usize]
    }

    fn open_neighbors(&self, pos: Pair) -> Vec<Pair> {
        DIRECTIONS.iter()
            .map(|&d| pos + d)
            .filter(|&p| self.is_open(p))
            .collect_vec()
    }

    fn enemy_positions(&self, pos: Pair) -> Vec<Pair> {
        if !self.positions.contains_key(&pos) {
            return Vec::new();
        }

        let race = self.warriors[self.positions[&pos]].race;

        self.positions.iter()
            .filter(|&(_, &i)| {
                let warrior = &self.warriors[i];
                warrior.race != race
            })
            .map(|(&pos, _)| pos)
            .collect_vec()
    }

    fn find_move(&self, pos: Pair) -> Option<Pair> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        distances.insert(pos, (0, None));
        for nbr in self.open_neighbors(pos) {
            distances.insert(nbr, (1, Some(nbr)));
            queue.push_back((nbr, 1, nbr));
        }

        while let Some((p, dist, via)) = queue.pop_front() {
            for nbr in self.open_neighbors(p) {
                if !distances.contains_key(&nbr) {
                    distances.insert(nbr, (dist + 1, Some(via)));
                    queue.push_back((nbr, dist + 1, via));
                }
            }
        }

        let index = *self.positions.get(&pos)?;

        self.enemy_positions(pos).into_iter()
            .flat_map(|tgt| {
                DIRECTIONS.map(|d| d + tgt)
            })
            .filter(|&tgt| tgt == pos || self.is_open(tgt))
            .filter_map(|tgt| {
                let &(dist, via) = distances.get(&tgt)?;
                Some((dist, tgt, via))
            })
            .min()
            .and_then(|(_, tgt, via)| via)
    }

    fn pick_attack(&self, pos: Pair) -> Option<Pair> {
        let index = *self.positions.get(&pos)?;
        assert!(self.warriors[index].hp > 0);

        let race = self.warriors[index].race;

        DIRECTIONS.iter()
            .map(|&d| pos + d)
            .filter_map(|p| {
                let index = *self.positions.get(&p)?;
                let enemy = &self.warriors[index];
                if enemy.race != race { Some((enemy.hp, p)) }
                else { None }
            })
            .min()
            .map(|(_, p)| p)
    }

    fn step(&mut self) -> Option<Step> {
        if self.turn_order.is_empty() {
            self.turn_order = self.warriors.iter().enumerate()
                .filter(|&(_, &w)| w.hp > 0)
                .sorted_by_key(|(_, w)| w.position)
                .map(|(i, _)| i)
                .collect();

            self.cur_round += 1;
            self.phase = Phase::Move;
            return Some(StartRound(self.cur_round));
        }

        let warrior_id = *self.turn_order.front().unwrap();
        if self.warriors[warrior_id].hp == 0 {
            self.turn_order.pop_front();
            self.phase = Phase::Move;
            return self.step();
        }

        if self.enemy_positions(self.warriors[warrior_id].position).is_empty() {
            let completed_rounds = self.cur_round - 1;
            let total_hp = self.warriors.iter().map(|warrior| warrior.hp).sum::<usize>();
            self.phase = Phase::Done;
            return Some(Done { completed_rounds, total_hp })
        }

        match self.phase {
            Phase::Done => None,
            Phase::Move => {
                let from = self.warriors[warrior_id].position;
                let to = self.find_move(from);
                let result = Some(Movement { warrior_id, from, to });

                if let Some(p) = to {
                    self.warriors[warrior_id].position = p;
                    self.positions.remove(&from);
                    self.positions.insert(p, warrior_id);
                }

                self.phase = Phase::Attack;
                result
            },
            Phase::Attack => {
                let position = self.warriors[warrior_id].position;
                let target_position = self.pick_attack(position);
                let target_id = target_position.map(|p| self.positions[&p]);
                let target_final_hp = target_id.map(|tid| {
                    let attack_power = self.warriors[warrior_id].attack_power;
                    let cur_hp = self.warriors[tid].hp;

                    if cur_hp > attack_power {
                        self.warriors[tid].hp -= attack_power;
                    }
                    else {
                        self.warriors[tid].hp = 0;
                        let p = self.warriors[tid].position;
                        self.positions.remove(&p);
                    }

                    self.warriors[tid].hp
                });

                let result = Some(
                    Attack {
                        warrior_id,
                        position,
                        target_id,
                        target_position,
                        target_final_hp
                });

                self.phase = Phase::Move;
                self.turn_order.pop_front();

                result
            }
        }
    }
}

impl Iterator for Level {
    type Item = Step;
    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}


fn parse_input(input: &str) -> IResult<&str, Level> {
    fn space(input: &str) -> IResult<&str, (Option<Race>, bool)> {
        alt((
            value((None, false), ch('.')),
            value((None, true), ch('#')),
            value((Some(Elf), false), ch('E')),
            value((Some(Goblin), false), ch('G'))
        ))(input)
    }

    map(
        all_consuming(separated_list1(newline, many1(space))),
        |v| {
            let mut warriors = Vec::new();
            let mut positions = HashMap::new();

            v.iter().enumerate().for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, &(warrior_type, _))| {
                    if let Some(race) = warrior_type {
                        let position = Pair(i as isize, j as isize);
                        let warrior = Warrior { hp: 200, race, attack_power: 3, position };
                        warriors.push(warrior);
                        let id = warriors.len() - 1;
                        positions.insert(position, id);
                    }

                })
            });

            let is_wall = v.into_iter()
                .map(|row| row.into_iter().map(|(_, b)| b).collect_vec())
                .collect_vec();

            let rows = is_wall.len();
            let cols = is_wall[0].len();

            Level {
                is_wall,
                rows,
                cols,
                warriors,
                positions,
                cur_round: 0,
                turn_order: VecDeque::new(),
                phase: Phase::Move
            }
        }
    )(input)
}

fn part1(input: &str) -> usize {
    let mut level = parse_input(input).unwrap().1;
    println!("Initial:");
    level.print();
    println!();

    while let Some(step) = level.next() {
        match step {
            Done { completed_rounds, total_hp } => return completed_rounds * total_hp,
            StartRound(i) => { println!("Starting round {i}"); level.print(); println!(); },
            Movement { warrior_id, from, to } => {
                if let Some(to) = to {
                    println!("Warrior {warrior_id} moved from ({},{}) to ({},{})", from.0, from.1, to.0, to.1);
                }
            },
            Attack { warrior_id, position, target_id, target_position, target_final_hp } => {
                if let (Some(target_id), Some(target_position), Some(target_final_hp)) = (target_id, target_position, target_final_hp) { {
                    println!("Warrior {warrior_id} attacked warrior {target_id} at ({},{}), leaving them at {target_final_hp} HP", target_position.0, target_position.1);
                }}
            }
        }
    }

    let Some(Done { completed_rounds, total_hp }) = level.last() else { panic!() };
    completed_rounds * total_hp
}

fn part2(input: &str) -> usize {
    let level = parse_input(input).unwrap().1;
    println!("Initial:");
    level.print();
    println!();

    for power in 4.. {
        let mut level_mod = level.clone();
        level_mod.warriors.iter_mut().for_each(|w| {
            if w.race == Elf {
                w.attack_power = power;
            }
        });

        while let Some(step) = level_mod.next() {
            match step {
                Done { completed_rounds, total_hp } => return completed_rounds * total_hp,
                Attack { warrior_id, position, target_id, target_position, target_final_hp } => {
                    if let Some(0) = target_final_hp {
                        if let Some(tid) = target_id {
                            if level_mod.warriors[tid].race == Elf {
                                break;
                            }
                        }
                    }
                },
                _ => ()
            }
        }
    }


    unreachable!()
}

build_main!("day15.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const TEST_INPUT_2: &str = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const TEST_INPUT_3: &str = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const TEST_INPUT_4: &str = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const TEST_INPUT_5: &str = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 27730);
        assert_eq!(part1(TEST_INPUT_2), 39514);
        assert_eq!(part1(TEST_INPUT_3), 27755);
        assert_eq!(part1(TEST_INPUT_4), 28944);
        assert_eq!(part1(TEST_INPUT_5), 18740);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 4988);
        assert_eq!(part2(TEST_INPUT_2), 31284);
        assert_eq!(part2(TEST_INPUT_3), 3478);
        assert_eq!(part2(TEST_INPUT_4), 6474);
        assert_eq!(part2(TEST_INPUT_5), 1140);
    }
}