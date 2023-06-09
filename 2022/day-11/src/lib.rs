use std::{iter::repeat, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, space1, u64},
    combinator::{map, value},
    error::Error,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Debug, PartialEq)]
enum Operand {
    Old,
    Val(u64),
}

impl Operand {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Self::Old => old,
            Self::Val(val) => *val,
        }
    }
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((value(Operand::Old, tag("old")), map(u64, Operand::Val)))(input)
}

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Mul(Operand),
    Add(Operand),
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Self::Mul(operand) => old * operand.eval(old),
            Self::Add(operand) => old + operand.eval(old),
        }
    }
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(
            preceded(pair(char('*'), space1), parse_operand),
            Operation::Mul,
        ),
        map(
            preceded(pair(char('+'), space1), parse_operand),
            Operation::Add,
        ),
    ))(input)
}

#[derive(Clone, Debug, PartialEq)]
struct Test {
    divisible_by: u64,
    if_true_throw_to_monkey: u64,
    if_false_throw_to_monkey: u64,
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    map(
        tuple((
            delimited(tag("divisible by "), u64, newline),
            delimited(pair(space1, tag("If true: throw to monkey ")), u64, newline),
            preceded(pair(space1, tag("If false: throw to monkey ")), u64),
        )),
        |(divisible_by, if_true_throw_to_monkey, if_false_throw_to_monkey)| Test {
            divisible_by,
            if_true_throw_to_monkey,
            if_false_throw_to_monkey,
        },
    )(input)
}

type Item = u64;

#[derive(Clone, Debug, PartialEq)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    preceded(
        tuple((tag("Monkey "), u64, char(':'), newline)),
        map(
            tuple((
                delimited(
                    pair(space1, tag("Starting items: ")),
                    separated_list0(tag(", "), u64),
                    newline,
                ),
                delimited(
                    pair(space1, tag("Operation: new = old ")),
                    parse_operation,
                    newline,
                ),
                preceded(pair(space1, tag("Test: ")), parse_test),
            )),
            |(items, operation, test)| Monkey {
                items,
                operation,
                test,
            },
        ),
    )(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Monkeys(Vec<Monkey>);

fn parse_monkeys(input: &str) -> IResult<&str, Monkeys> {
    map(separated_list0(tag("\n\n"), parse_monkey), Monkeys)(input)
}

impl FromStr for Monkeys {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_monkeys(s).finish() {
            Ok((_remaining, monkeys)) => Ok(monkeys),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn part_1(Monkeys(mut monkeys): Monkeys) -> u64 {
    let mut inspected_items_count: Vec<u64> = repeat(0).take(monkeys.len()).collect();

    for _round in 0..20 {
        let magic_trick: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();

        for monkey_index in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_index].items.pop() {
                inspected_items_count[monkey_index] += 1;

                let monkey = &monkeys[monkey_index];
                let new_item = monkey.operation.eval(item) / 3 % magic_trick;

                let new_monkey_index = if new_item % monkey.test.divisible_by == 0 {
                    monkey.test.if_true_throw_to_monkey
                } else {
                    monkey.test.if_false_throw_to_monkey
                };

                monkeys[new_monkey_index as usize].items.push(new_item);
            }
        }
    }

    inspected_items_count.sort();

    inspected_items_count.iter().rev().take(2).product()
}

pub fn part_2(Monkeys(mut monkeys): Monkeys) -> u64 {
    let mut inspected_items_count: Vec<u64> = repeat(0).take(monkeys.len()).collect();
    for _round in 0..10_000 {
        let magic_trick: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();

        for monkey_index in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_index].items.pop() {
                inspected_items_count[monkey_index] += 1;

                let monkey = &monkeys[monkey_index];
                let new_item = monkey.operation.eval(item) % magic_trick;

                let new_monkey_index = if new_item % monkey.test.divisible_by == 0 {
                    monkey.test.if_true_throw_to_monkey
                } else {
                    monkey.test.if_false_throw_to_monkey
                };

                monkeys[new_monkey_index as usize].items.push(new_item);
            }
        }
    }

    inspected_items_count.sort();

    inspected_items_count.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MONKEYS: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_parse_operand() {
        assert_eq!(parse_operand("19"), Ok(("", Operand::Val(19))));

        assert_eq!(parse_operand("old"), Ok(("", Operand::Old)));
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("+ 6"),
            Ok(("", Operation::Add(Operand::Val(6))))
        );

        assert_eq!(
            parse_operation("* old"),
            Ok(("", Operation::Mul(Operand::Old)))
        );
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(
            parse_test(
                "divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            ),
            Ok((
                "",
                Test {
                    divisible_by: 23,
                    if_true_throw_to_monkey: 2,
                    if_false_throw_to_monkey: 3,
                }
            ))
        );
    }

    #[test]
    fn test_parse_monkeys() {
        assert_eq!(
            MONKEYS.parse(),
            Ok(Monkeys(vec![
                Monkey {
                    items: vec![79, 98],
                    operation: Operation::Mul(Operand::Val(19)),
                    test: Test {
                        divisible_by: 23,
                        if_true_throw_to_monkey: 2,
                        if_false_throw_to_monkey: 3,
                    }
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: Operation::Add(Operand::Val(6)),
                    test: Test {
                        divisible_by: 19,
                        if_true_throw_to_monkey: 2,
                        if_false_throw_to_monkey: 0,
                    }
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: Operation::Mul(Operand::Old),
                    test: Test {
                        divisible_by: 13,
                        if_true_throw_to_monkey: 1,
                        if_false_throw_to_monkey: 3,
                    }
                },
                Monkey {
                    items: vec![74],
                    operation: Operation::Add(Operand::Val(3)),
                    test: Test {
                        divisible_by: 17,
                        if_true_throw_to_monkey: 0,
                        if_false_throw_to_monkey: 1,
                    }
                },
            ]))
        );
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(MONKEYS.parse().unwrap());

        assert_eq!(answer, 10605);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(MONKEYS.parse().unwrap());

        assert_eq!(answer, 2713310158);
    }
}
