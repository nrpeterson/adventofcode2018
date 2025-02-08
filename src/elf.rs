use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline, space1};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use crate::elf::Value::*;
use crate::elf::Instruction::*;

#[derive(Copy, Clone)]
pub enum Value {
    Literal(usize),
    Register(usize)
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Add(usize, Value, usize),
    Mul(usize, Value, usize),
    Ban(usize, Value, usize),
    Bor(usize, Value, usize),
    Set(Value, usize, usize),
    Gt(Value, Value, usize),
    Eq(Value, Value, usize)
}

pub struct Machine {
    pub registers: [usize; 6],
    pub ip: usize,
    pub instructions: Vec<Instruction>,
    pub is_break: Vec<bool>
}

impl Machine {
    fn value_of(&self, value: Value) -> usize {
        match value {
            Literal(x) => x,
            Register(i) => self.registers[i]
        }
    }
}

impl Iterator for Machine {
    type Item = [usize; 6];

    fn next(&mut self) -> Option<Self::Item> {
        let ip = self.registers[self.ip];

        if ip >= self.instructions.len() || self.is_break[ip] {
            return None;
        }

        match self.instructions[self.registers[self.ip]] {
            Add(a, b, c) => {
                let b = self.value_of(b);
                self.registers[c] = self.registers[a] + b;
            },
            Mul(a, b, c) => {
                let b = self.value_of(b);
                self.registers[c] = self.registers[a] * b;
            },
            Ban(a, b, c) => {
                let b = self.value_of(b);
                self.registers[c] = self.registers[a] & b;
            },
            Bor(a, b, c) => {
                let b = self.value_of(b);
                self.registers[c] = self.registers[a] | b;
            },
            Set(a, _, c) => {
                self.registers[c] = self.value_of(a);
            },
            Gt(a, b, c) => {
                let a = self.value_of(a);
                let b = self.value_of(b);
                self.registers[c] = if a > b { 1 } else { 0 };
            },
            Eq(a, b, c) => {
                let a = self.value_of(a);
                let b = self.value_of(b);
                self.registers[c] = if a == b { 1 } else { 0 };
            }
        }
        self.registers[self.ip] += 1;

        Some(self.registers.clone())
    }
}

pub fn parse_machine(input: &str) -> IResult<&str, Machine> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                alpha1,
                preceded(space1, number),
                preceded(space1, number),
                preceded(space1, number)
            )),
            |(op, a, b, c)| {
                match op {
                    "addr" => Add(a, Register(b), c),
                    "addi" => Add(a, Literal(b), c),
                    "mulr" => Mul(a, Register(b), c),
                    "muli" => Mul(a, Literal(b), c),
                    "banr" => Ban(a, Register(b), c),
                    "bani" => Ban(a, Literal(b), c),
                    "borr" => Bor(a, Register(b), c),
                    "bori" => Bor(a, Literal(b), c),
                    "setr" => Set(Register(a), b, c),
                    "seti" => Set(Literal(a), b, c),
                    "gtir" => Gt(Literal(a), Register(b), c),
                    "gtri" => Gt(Register(a), Literal(b), c),
                    "gtrr" => Gt(Register(a), Register(b), c),
                    "eqir" => Eq(Literal(a), Register(b), c),
                    "eqri" => Eq(Register(a), Literal(b), c),
                    "eqrr" => Eq(Register(a), Register(b), c),
                    other => panic!("Unknown instruction: {other}")
                }
            }
        )(input)
    }

    map(
        all_consuming(
            separated_pair(
                preceded(tag("#ip "), number),
                newline,
                separated_list1(newline, instruction)
            )
        ),
        |(ip, instructions)| {
            let is_break = vec![false; instructions.len()];

            Machine {
                registers: [0; 6],
                ip,
                instructions,
                is_break
            }
        }
    )(input)
}