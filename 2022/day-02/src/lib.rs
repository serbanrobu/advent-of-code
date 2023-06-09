use std::{char::ParseCharError, cmp::Ordering, str::FromStr};
use thiserror::Error;

impl From<Strategy> for HandShape {
    fn from(value: Strategy) -> Self {
        match value {
            Strategy::Win => HandShape::Rock,
            Strategy::Draw => HandShape::Paper,
            Strategy::Lose => HandShape::Scissors,
        }
    }
}

pub fn part_1(Input(rounds): Input) -> u32 {
    rounds
        .into_iter()
        .map(|Round(fst, snd)| {
            let opponent_shape = HandShape::from(fst);
            let my_shape = HandShape::from(snd);
            (opponent_shape, my_shape)
        })
        .map(|(opponent_shape, my_shape)| my_shape.outcome(&opponent_shape) + my_shape.score())
        .sum()
}

pub fn part_2(Input(rounds): Input) -> u32 {
    rounds
        .into_iter()
        .map(|Round(fst, snd)| {
            let opponent_shape = HandShape::from(fst);
            let strategy = Strategy::from(snd);
            let my_shape = match strategy {
                Strategy::Win => opponent_shape.next(),
                Strategy::Draw => opponent_shape,
                Strategy::Lose => opponent_shape.prev(),
            };

            (opponent_shape, my_shape)
        })
        .map(|(opponent_shape, my_shape)| my_shape.outcome(&opponent_shape) + my_shape.score())
        .sum()
}

pub struct Round(FirstEncoding, SecondEncoding);

#[derive(Debug, Error)]
pub enum RoundError {
    #[error("expected space separated columns")]
    NoSpaceFound,
    #[error(transparent)]
    FirstEncodingError(#[from] FirstEncodingError),
    #[error(transparent)]
    SecondEncodingError(#[from] SecondEncodingError),
}

impl FromStr for Round {
    type Err = RoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fst, snd) = s.split_once(' ').ok_or(RoundError::NoSpaceFound)?;
        Ok(Round(fst.parse()?, snd.parse()?))
    }
}

enum Strategy {
    Lose,
    Draw,
    Win,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.next() == *other {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl HandShape {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome(&self, other: &Self) -> u32 {
        match self.cmp(other) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn prev(&self) -> Self {
        self.next().next()
    }
}

pub enum FirstEncoding {
    A,
    B,
    C,
}

impl From<FirstEncoding> for HandShape {
    fn from(value: FirstEncoding) -> Self {
        match value {
            FirstEncoding::A => Self::Rock,
            FirstEncoding::B => Self::Paper,
            FirstEncoding::C => Self::Scissors,
        }
    }
}

#[derive(Debug, Error)]
pub enum FirstEncodingError {
    #[error(transparent)]
    NotAChar(#[from] ParseCharError),
    #[error("expected one of 'A', 'B' or 'C', found: {0:?}")]
    UnexpectedChar(char),
}

impl FromStr for FirstEncoding {
    type Err = FirstEncodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse().map_err(FirstEncodingError::NotAChar)? {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            c => Err(FirstEncodingError::UnexpectedChar(c)),
        }
    }
}

pub enum SecondEncoding {
    X,
    Y,
    Z,
}

impl From<SecondEncoding> for Strategy {
    fn from(value: SecondEncoding) -> Self {
        match value {
            SecondEncoding::X => Self::Lose,
            SecondEncoding::Y => Self::Draw,
            SecondEncoding::Z => Self::Win,
        }
    }
}

impl From<SecondEncoding> for HandShape {
    fn from(value: SecondEncoding) -> Self {
        match value {
            SecondEncoding::X => Self::Rock,
            SecondEncoding::Y => Self::Paper,
            SecondEncoding::Z => Self::Scissors,
        }
    }
}

#[derive(Debug, Error)]
pub enum SecondEncodingError {
    #[error(transparent)]
    NotAChar(#[from] ParseCharError),
    #[error("expected one of 'X', 'Y' or 'Z', found: {0:?}")]
    UnexpectedChar(char),
}

impl FromStr for SecondEncoding {
    type Err = SecondEncodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse().map_err(SecondEncodingError::NotAChar)? {
            'X' => Ok(Self::X),
            'Y' => Ok(Self::Y),
            'Z' => Ok(Self::Z),
            c => Err(SecondEncodingError::UnexpectedChar(c)),
        }
    }
}

pub struct Input(Vec<Round>);

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid round {idx}: {err}")]
    InvalidRound { idx: usize, err: RoundError },
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .map(|(idx, line)| {
                line.parse()
                    .map_err(|err| InputError::InvalidRound { idx, err })
            })
            .collect::<Result<Vec<Round>, _>>()
            .map(Input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn test_part_1() {
        let answer = part_1(INPUT.parse().unwrap());

        assert_eq!(answer, 15);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(INPUT.parse().unwrap());

        assert_eq!(answer, 12);
    }
}
