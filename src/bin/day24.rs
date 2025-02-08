use crate::GroupRef::{ImmuneSystem, Infection};
use crate::Modifiers::{Immunities, Weaknesses};
use adventofcode2018::build_main;
use itertools::{chain, Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, multispace1, newline, space1};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use std::cmp::{min, Reverse};

#[derive(Debug, Clone)]
struct Group {
    units: usize,
    hit_points: usize,
    attack_damage: usize,
    attack_type: String,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>
}

impl Group {
    fn effective_damage(&self) -> usize {
        self.units * self.attack_damage
    }

    fn damage_to(&self, enemy: &Group) -> usize {
        let base_damage = self.units * self.attack_damage;
        if enemy.weaknesses.contains(&self.attack_type) {
            2 * base_damage
        }
        else if enemy.immunities.contains(&self.attack_type) { 0 }
        else { base_damage }
    }
}

fn find_choices(allies: &[Group], enemies: &[Group]) -> Vec<Option<usize>> {
    let mut chosen = enemies.iter().map(|_| false).collect_vec();

    let mut order = (0..allies.len())
        .filter(|&i| allies[i].units > 0)
        .collect_vec();

    order.sort_by_key(|&i| Reverse((allies[i].effective_damage(), allies[i].initiative)));

    let mut choices = allies.iter().map(|_| None).collect_vec();

    for i in order {
        let attacker = &allies[i];
        let mut choice = None;
        let mut best_stats = (0, 0, 0);

        for j in 0..enemies.len() {
            let defender = &enemies[j];
            if defender.units == 0
                || chosen[j]
                || defender.immunities.contains(&attacker.attack_type) {
                continue
            }

            let damage = attacker.damage_to(defender);
            let effective_damage = defender.effective_damage();
            let initiative = defender.initiative;
            let stats = (damage, effective_damage, initiative);

            if stats > best_stats {
                choice = Some(j);
                best_stats = stats;
            }
        }

        if let Some(j) = choice {
            chosen[j] = true;
            choices[i] = Some(j);
        }
    }

    choices
}

#[derive(Debug)]
enum GroupRef {
    Infection(usize),
    ImmuneSystem(usize)
}

struct TurnStats {
    units_killed: usize,
    immune_system_units_remaining: usize,
    infection_units_remaining: usize
}

#[derive(Debug, Clone)]
struct War {
    immune_system: Vec<Group>,
    infection: Vec<Group>
}

impl War {
    fn apply_boost(&mut self, boost: usize) {
        self.immune_system.iter_mut()
            .for_each(|g| g.attack_damage += boost);
    }

    fn get(&self, gref: &GroupRef) -> &Group {
        match gref {
            &Infection(i) => &self.infection[i],
            &ImmuneSystem(i) => &self.immune_system[i]
        }
    }

    fn get_mut(&mut self, gref: &GroupRef) -> &mut Group {
        match gref {
            &Infection(i) => &mut self.infection[i],
            &ImmuneSystem(i) => &mut self.immune_system[i]
        }
    }

    fn advance(&mut self) -> TurnStats {
        let mut units_killed = 0;

        let infection_choices = find_choices(&self.infection, &self.immune_system);
        let immune_system_choices = find_choices(&self.immune_system, &self.infection);

        let mut turn_order =
            chain!(
                (0..self.infection.len()).map(|i| Infection(i)),
                (0..self.immune_system.len()).map(|j| ImmuneSystem(j))
            )
            .filter(|r| self.get(r).units > 0)
            .collect_vec();

        turn_order.sort_by_key(|r| Reverse(self.get(r).initiative));

        for r in turn_order {
            let attacker = self.get(&r);

            let choice = match r {
                Infection(i) => infection_choices[i].map(ImmuneSystem),
                ImmuneSystem(i) => immune_system_choices[i].map(Infection)
            };

            if let Some(defref) = choice {
                let defender = self.get(&defref);
                let damage = attacker.damage_to(defender);
                let num_dead = min(defender.units, damage / defender.hit_points);
                units_killed += num_dead;
                self.get_mut(&defref).units -= num_dead;
            }
        }

        let immune_system_units_remaining = self.immune_system.iter()
            .map(|g| g.units)
            .sum::<usize>();

        let infection_units_remaining = self.infection.iter()
            .map(|g| g.units)
            .sum::<usize>();

        TurnStats { units_killed, immune_system_units_remaining, infection_units_remaining }
    }
}

struct WarIter {
    war: War,
    is_done: bool
}

impl IntoIterator for War {
    type Item = TurnStats;
    type IntoIter = WarIter;

    fn into_iter(self) -> Self::IntoIter {
        WarIter { war: self, is_done: false }
    }
}

impl Iterator for WarIter {
    type Item = TurnStats;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let stats = self.war.advance();

        if stats.units_killed == 0
            || stats.immune_system_units_remaining == 0
            || stats.infection_units_remaining == 0 {
            self.is_done = true;
        }

        Some(stats)
    }
}

enum Modifiers {
    Immunities(Vec<String>),
    Weaknesses(Vec<String>)
}

fn parse_input(input: &str) -> IResult<&str, War> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn modifiers(input: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
        fn modifier(input: &str) -> IResult<&str, Modifiers> {
            alt((
                map(
                    preceded(tag("weak to "), separated_list1(tag(", "), alpha1)),
                    |v: Vec<&str>| Weaknesses(v.into_iter().map(|s| s.to_owned()).collect_vec())),
                map(
                    preceded(tag("immune to "), separated_list1(tag(", "), alpha1)),
                    |v: Vec<&str>| Immunities(v.into_iter().map(|s| s.to_owned()).collect_vec()))
            ))(input)
        }

        map(
            opt(
                delimited(
                    tag(" ("),
                    separated_list1(tag("; "), modifier),
                    char(')')
                )
            ),
            |v| {
                v.unwrap_or_default().into_iter()
                    .fold((Vec::new(), Vec::new()), |(imm, wkns), cur| {
                        match cur {
                            Immunities(immunities) => { (immunities, wkns) },
                            Weaknesses(weaknesses) => { (imm, weaknesses) }
                        }
                    })
            }
        )(input)
    }

    fn group(input: &str) -> IResult<&str, Group> {
        map(
            tuple((
                number,
                delimited(tag(" units each with "), number, tag(" hit points")),
                modifiers,
                preceded(tag(" with an attack that does "), number),
                map(delimited(space1, alpha1, tag(" damage")), |s: &str| s.to_owned()),
                preceded(tag(" at initiative "), number)
            )),
            |(units, hit_points, (immunities, weaknesses), attack_damage, attack_type, initiative)| {
                Group { units, hit_points, attack_damage, attack_type, initiative, immunities, weaknesses }
            }
        )(input)
    }

    all_consuming(
        map(
            separated_pair(
                preceded(pair(tag("Immune System:"), newline), separated_list1(newline, group)),
                multispace1,
                preceded(pair(tag("Infection:"), newline), separated_list1(newline, group))
            ),
            |(immune_system, infection)| War { immune_system, infection }
        )
    )(input)
}

fn part1(input: &str) -> usize {
    let war = parse_input(input).unwrap().1;
    let it = war.into_iter();

    it.last().map(|s| s.infection_units_remaining + s.immune_system_units_remaining).unwrap()
}

fn part2(input: &str) -> usize {
    let base_war = parse_input(input).unwrap().1;

    for boost in 0.. {
        let mut war = base_war.clone();
        war.apply_boost(boost);
        let it = war.into_iter();
        let result = it.last().unwrap();
        if result.infection_units_remaining == 0 {
            return result.immune_system_units_remaining;
        }
    }

    unreachable!()
}

build_main!("day24.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5216);
    }
}