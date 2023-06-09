use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline, space1},
    combinator::{map, value},
    error::Error,
    multi::separated_list0,
    sequence::{pair, preceded},
    Finish, IResult,
};
use std::{fmt, ops::Deref, str::FromStr};

const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

struct Xs {
    value: i32,
    instruction_index: usize,
    instructions: Instructions,
    instruction_cycle: u32,
}

impl Xs {
    fn new(instructions: Instructions) -> Self {
        Self {
            value: 1,
            instruction_index: 0,
            instructions,
            instruction_cycle: 1,
        }
    }
}

impl Iterator for Xs {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(instruction) = self.instructions.get(self.instruction_index) else {
            return None;
        };

        let value = self.value;

        match instruction {
            Instruction::Addx(v) if self.instruction_cycle == 2 => {
                self.value += v;
                self.instruction_index += 1;
                self.instruction_cycle = 1;
            }
            Instruction::Noop if self.instruction_cycle == 1 => {
                self.instruction_index += 1;
                self.instruction_cycle = 1;
            }
            _ => {
                self.instruction_cycle += 1;
            }
        }

        Some(value)
    }
}

pub fn part_1(instructions: Instructions) -> i32 {
    let xs: Vec<_> = Xs::new(instructions).take(220).collect();

    CYCLES
        .into_iter()
        .map(|c| (c as i32) * xs.get(c - 1).copied().unwrap_or_default())
        .sum()
}

#[derive(Debug)]
pub enum Pixel {
    Lit,
    Dark,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}

pub struct Crt([[Pixel; 40]; 6]);

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for pixel in row {
                write!(f, "{pixel}")?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub fn part_2(instructions: Instructions) -> Crt {
    let mut crt: Vec<[Pixel; 40]> = vec![];
    let mut xs = Xs::new(instructions);

    for _ in 0..6 {
        let mut row = vec![];

        for position in 0..40 {
            let x = xs.next().unwrap_or_default();

            let pixel = if position >= x - 1 && position <= x + 1 {
                Pixel::Lit
            } else {
                Pixel::Dark
            };

            row.push(pixel);
        }

        crt.push(row.try_into().unwrap());
    }

    Crt(crt.try_into().unwrap())
}

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(pair(tag("addx"), space1), i32), Instruction::Addx),
        value(Instruction::Noop, tag("noop")),
    ))(input)
}

#[derive(Debug, PartialEq)]
pub struct Instructions(Vec<Instruction>);

impl Deref for Instructions {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Instructions> {
    map(separated_list0(newline, parse_instruction), Instructions)(input)
}

impl FromStr for Instructions {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_instructions(s).finish() {
            Ok((_remaining, instructions)) => Ok(instructions),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTRUCTIONS: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_parse_instructions() {
        use Instruction::*;

        assert_eq!(
            "noop\naddx 3\naddx -5".parse(),
            Ok(Instructions(vec![Noop, Addx(3), Addx(-5)]))
        )
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(INSTRUCTIONS.parse().unwrap());

        assert_eq!(answer, 13140);
    }
}
