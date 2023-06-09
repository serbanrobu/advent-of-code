use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

pub fn part_1(input: Input) -> u32 {
    input
        .groups
        .iter()
        .flat_map(|g| &g.rucksacks)
        .map(|rucksack| rucksack.shared_item_type().priority() as u32)
        .sum()
}

pub fn part_2(input: Input) -> u32 {
    input.groups.iter().map(|g| g.badge.priority() as u32).sum()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ItemType {
    id: char,
}

#[derive(Debug, Error)]
pub enum ItemTypeError {
    #[error("invalid id {0:?}")]
    InvalidId(char),
}

impl ItemType {
    fn new(id: char) -> Result<Self, ItemTypeError> {
        match id {
            'a'..='z' | 'A'..='Z' => Ok(Self { id }),
            _ => Err(ItemTypeError::InvalidId(id)),
        }
    }

    fn priority(&self) -> u8 {
        match self.id {
            'a'..='z' => (self.id as u8) - 96,
            'A'..='Z' => (self.id as u8) - 38,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Compartment {
    pub item_types: HashSet<ItemType>,
}

impl FromStr for Compartment {
    type Err = CompartmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .enumerate()
            .map(|(idx, c)| {
                ItemType::new(c).map_err(|err| CompartmentError::InvalidItemType { idx, err })
            })
            .collect::<Result<HashSet<_>, _>>()
            .map(|item_types| Compartment { item_types })
    }
}

#[derive(Debug)]
struct Rucksack {
    compartments: [Compartment; 2],
}

impl Rucksack {
    pub fn shared_item_type(&self) -> &ItemType {
        self.compartments[0]
            .item_types
            .intersection(&self.compartments[1].item_types)
            .next()
            .expect("one shared item type")
    }
}

impl FromStr for Rucksack {
    type Err = RucksackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();

        if len & 1 == 1 {
            return Err(Self::Err::UnequalCompartmentSize);
        }

        let (s_1, s_2) = s.split_at(len / 2);

        let compartment_1 = s_1
            .parse::<Compartment>()
            .map_err(Self::Err::Compartment1Error)?;

        let compartment_2 = s_2
            .parse::<Compartment>()
            .map_err(Self::Err::Compartment2Error)?;

        if compartment_1
            .item_types
            .intersection(&compartment_2.item_types)
            .count()
            != 1
        {
            return Err(Self::Err::MultipleSharedItemTypes);
        };

        Ok(Self {
            compartments: [compartment_1, compartment_2],
        })
    }
}

#[derive(Debug, Error)]
pub enum RucksackError {
    #[error("the size of the compartments is not equal")]
    UnequalCompartmentSize,
    #[error("the compartments contains multiple shared item types")]
    MultipleSharedItemTypes,
    #[error("invalid compartment 1: {0}")]
    Compartment1Error(CompartmentError),
    #[error("invalid compartment 2: {0}")]
    Compartment2Error(CompartmentError),
}

#[derive(Debug, Error)]
pub enum CompartmentError {
    #[error("invalid item type {idx}: {err}")]
    InvalidItemType { idx: usize, err: ItemTypeError },
}

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("expected a badge")]
    NoBadge,
    #[error("expected a unique badge")]
    NotUniqueBadge,
    #[error("expected 3 rucksacks for the group")]
    NotExactlyRucksacks,
    #[error("invalid rucksack {idx}: {err}")]
    InvalidRucksack { idx: usize, err: RucksackError },
}

pub struct Group {
    badge: ItemType,
    rucksacks: [Rucksack; 3],
}

impl TryFrom<&[&str]> for Group {
    type Error = GroupError;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let rucksacks: [Rucksack; 3] = value
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                line.parse::<Rucksack>()
                    .map_err(|err| Self::Error::InvalidRucksack { idx, err })
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| Self::Error::NotExactlyRucksacks)?;

        let badges = rucksacks
            .iter()
            .map(|r| {
                r.compartments.iter().fold(HashSet::new(), |acc, c| {
                    acc.union(&c.item_types).cloned().collect()
                })
            })
            .reduce(|acc, set| acc.intersection(&set).cloned().collect())
            .unwrap();

        if badges.len() > 1 {
            return Err(GroupError::NotUniqueBadge);
        }

        let Some(badge) = badges.into_iter().next() else {
            return Err(GroupError::NoBadge);
        };

        Ok(Group { badge, rucksacks })
    }
}

pub struct Input {
    groups: Vec<Group>,
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid group {idx}: {err}")]
    InvalidGroup { idx: usize, err: GroupError },
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .enumerate()
            .map(|(idx, chunk)| {
                chunk
                    .try_into()
                    .map_err(|err| InputError::InvalidGroup { idx, err })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { groups })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part_1() {
        let answer = part_1(INPUT.parse().unwrap());

        assert_eq!(answer, 157);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(INPUT.parse().unwrap());

        assert_eq!(answer, 70);
    }
}
