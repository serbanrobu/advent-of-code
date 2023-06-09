use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

pub fn part_1(Input(food): Input) -> Calories {
    food.iter()
        .map(|items| items.iter().sum())
        .max()
        .unwrap_or(0)
}

pub fn part_2(Input(food): Input) -> Calories {
    let mut calories: Vec<Calories> = food.iter().map(|items| items.iter().sum()).collect();
    calories.sort();
    calories.into_iter().rev().take(3).sum()
}

type Calories = u32;

type Items = Vec<Calories>;

type Food = Vec<Items>;

pub struct Input(Food);

#[derive(Debug, Error)]
pub enum InputError {
    #[error("invalid calories for elf {elf_idx}, item {item_idx}: {err}")]
    InvalidCalories {
        elf_idx: usize,
        item_idx: usize,
        err: ParseIntError,
    },
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n\n")
            .enumerate()
            .map(|(elf_idx, items)| {
                items
                    .lines()
                    .enumerate()
                    .map(|(item_idx, item)| {
                        item.parse().map_err(|err| InputError::InvalidCalories {
                            elf_idx,
                            item_idx,
                            err,
                        })
                    })
                    .collect::<Result<Items, _>>()
            })
            .collect::<Result<Food, _>>()
            .map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_part_1() {
        let output = part_1(INPUT.parse().unwrap());

        assert_eq!(output, 24000);
    }

    #[test]
    fn test_part_2() {
        let output = part_2(INPUT.parse().unwrap());

        assert_eq!(output, 45000);
    }
}
