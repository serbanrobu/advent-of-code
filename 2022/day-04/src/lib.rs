use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

pub fn part_1(input: Input) -> usize {
    input
        .section_range_pairs
        .iter()
        .filter(|SectionRangePair(range_1, range_2)| {
            range_1.fully_contains(range_2) || range_2.fully_contains(range_1)
        })
        .count()
}

pub fn part_2(input: Input) -> usize {
    input
        .section_range_pairs
        .iter()
        .filter(|SectionRangePair(range_1, range_2)| range_1.overlap(range_2))
        .count()
}

type Section = u32;

#[derive(Debug, Error)]
pub enum SectionRangeError {
    #[error("expected dash separated sections")]
    DashNotFound,
    #[error("invalid start section")]
    StartSectionError(#[source] ParseIntError),
    #[error("invalid end section")]
    EndSectionError(#[source] ParseIntError),
}

struct SectionRange {
    start: Section,
    end: Section,
}

impl SectionRange {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start
            || self.start <= other.end && self.end >= other.end
            || other.fully_contains(self)
    }
}

impl FromStr for SectionRange {
    type Err = SectionRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(Self::Err::DashNotFound)?;

        Ok(Self {
            start: start.parse().map_err(Self::Err::StartSectionError)?,
            end: end.parse().map_err(Self::Err::EndSectionError)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid section range pair {idx}: {err}")]
    SectionRangePairError {
        idx: usize,
        err: SectionRangePairError,
    },
}

#[derive(Debug, Error)]
pub enum SectionRangePairError {
    #[error("expected comma separated elves")]
    CommaNotFound,
    #[error("invalid first section range")]
    FirstSectionRangeError(#[source] SectionRangeError),
    #[error("invalid second section range")]
    SecondSectionRangeError(#[source] SectionRangeError),
}

pub struct SectionRangePair(SectionRange, SectionRange);

impl FromStr for SectionRangePair {
    type Err = SectionRangePairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (range_1, range_2) = s.split_once(',').ok_or(Self::Err::CommaNotFound)?;

        Ok(Self(
            range_1.parse().map_err(Self::Err::FirstSectionRangeError)?,
            range_2
                .parse()
                .map_err(Self::Err::SecondSectionRangeError)?,
        ))
    }
}

pub struct Input {
    section_range_pairs: Vec<SectionRangePair>,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .map(|(idx, line)| {
                line.parse::<SectionRangePair>()
                    .map_err(|err| Self::Err::SectionRangePairError { idx, err })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|section_range_pairs| Self {
                section_range_pairs,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part_1() {
        let answer = part_1(INPUT.parse().unwrap());

        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(INPUT.parse().unwrap());

        assert_eq!(answer, 4);
    }
}
