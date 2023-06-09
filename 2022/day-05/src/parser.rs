use crate::procedure::{Crate, Procedure, Stack, Step};
use nom::branch::alt;
use nom::combinator::{value, verify};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    combinator::map,
};

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, (crates_quantity, target_stack, destination_stack)) = tuple((
        preceded(tag("move "), u32),
        preceded(tag(" from "), u32),
        preceded(tag(" to "), u32),
    ))(input)?;

    Ok((
        input,
        Step {
            crates_quantity,
            target_stack,
            destination_stack,
        },
    ))
}

type Steps = Vec<Step>;

fn parse_steps(input: &str) -> IResult<&str, Steps> {
    separated_list0(char('\n'), parse_step)(input)
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    map(
        delimited(char('['), verify(anychar, |c| c.is_uppercase()), char(']')),
        Crate,
    )(input)
}

type Layer = Vec<Option<Crate>>;

fn parse_layer(input: &str) -> IResult<&str, Layer> {
    separated_list0(
        char(' '),
        alt((value(None, tag("   ")), map(parse_crate, Some))),
    )(input)
}

type Layers = Vec<Layer>;

fn parse_layers(input: &str) -> IResult<&str, Layers> {
    separated_list0(char('\n'), parse_layer)(input)
}

fn parse_stack_number(input: &str) -> IResult<&str, u32> {
    delimited(char(' '), u32, char(' '))(input)
}

fn parse_stack_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(char(' '), parse_stack_number)(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Stack>> {
    let (input, stack_layers) = terminated(parse_layers, parse_stack_numbers)(input)?;

    let mut stacks: Vec<Stack> = vec![];

    for stack_layer in stack_layers.into_iter().rev() {
        for (j, crate_option) in stack_layer.into_iter().enumerate() {
            if stacks.len() <= j {
                let stack = vec![];
                stacks.push(stack);
            }

            let stack = &mut stacks[j];

            if let Some(crate_) = crate_option {
                stack.push(crate_);
            }
        }
    }

    Ok((input, stacks))
}

pub fn parse_procedure(input: &str) -> IResult<&str, Procedure> {
    let (input, (stacks, steps)) = separated_pair(parse_stacks, tag("\n\n"), parse_steps)(input)?;

    Ok((input, Procedure { stacks, steps }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_step() {
        const STEP: &str = "move 1 from 2 to 1";

        assert_eq!(
            parse_step(STEP),
            Ok((
                "",
                Step {
                    crates_quantity: 1,
                    target_stack: 2,
                    destination_stack: 1
                }
            ))
        );
    }

    #[test]
    fn test_parse_steps() {
        const STEPS: &str = "move 1 from 2 to 1
move 3 from 1 to 3";

        assert_eq!(
            parse_steps(STEPS),
            Ok((
                "",
                vec![
                    Step {
                        crates_quantity: 1,
                        target_stack: 2,
                        destination_stack: 1
                    },
                    Step {
                        crates_quantity: 3,
                        target_stack: 1,
                        destination_stack: 3
                    }
                ]
            ))
        )
    }

    #[test]
    fn test_parse_crate() {
        const CRATE: &str = "[A]";

        assert_eq!(parse_crate(CRATE), Ok(("", Crate('A'))));
    }

    #[test]
    fn test_parse_layer() {
        const LAYER: &str = "    [D]    ";

        assert_eq!(
            parse_layer(LAYER),
            Ok(("", vec![None, Some(Crate('D')), None]))
        );
    }

    #[test]
    fn test_parse_layers() {
        const LAYERS: &str = "    [D]    
[N] [C]    ";

        assert_eq!(
            parse_layers(LAYERS),
            Ok((
                "",
                vec![
                    vec![None, Some(Crate('D')), None],
                    vec![Some(Crate('N')), Some(Crate('C')), None]
                ]
            ))
        );
    }

    #[test]
    fn test_parse_stack_number() {
        const STACK_NUMBER: &str = " 1 ";

        assert_eq!(parse_stack_number(STACK_NUMBER), Ok(("", 1)));
    }

    #[test]
    fn test_parse_stack_numbers() {
        const STACK_NUMBERS: &str = " 1   2   3 ";

        assert_eq!(parse_stack_numbers(STACK_NUMBERS), Ok(("", vec![1, 2, 3])));
    }

    #[test]
    fn test_parse_stacks() {
        const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;

        assert_eq!(
            parse_stacks(INPUT),
            Ok((
                "",
                vec![
                    vec![Crate('Z'), Crate('N')],
                    vec![Crate('M'), Crate('C'), Crate('D')],
                    vec![Crate('P')],
                ]
            ))
        );
    }

    #[test]
    fn test_parse_procedure() {
        const PROCEDURE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(
            parse_procedure(PROCEDURE),
            Ok((
                "",
                Procedure {
                    stacks: vec![
                        vec![Crate('Z'), Crate('N')],
                        vec![Crate('M'), Crate('C'), Crate('D')],
                        vec![Crate('P')],
                    ],
                    steps: vec![
                        Step {
                            crates_quantity: 1,
                            target_stack: 2,
                            destination_stack: 1
                        },
                        Step {
                            crates_quantity: 3,
                            target_stack: 1,
                            destination_stack: 3
                        },
                        Step {
                            crates_quantity: 2,
                            target_stack: 2,
                            destination_stack: 1
                        },
                        Step {
                            crates_quantity: 1,
                            target_stack: 1,
                            destination_stack: 2
                        }
                    ]
                }
            ))
        )
    }
}
