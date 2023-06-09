use crate::parser::parse_procedure;
use nom::{error::Error, Finish};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Crate(pub char);

pub type Stack = Vec<Crate>;

#[derive(Debug, PartialEq)]
pub struct Step {
    pub crates_quantity: u32,
    pub target_stack: u32,
    pub destination_stack: u32,
}

#[derive(Debug, PartialEq)]
pub struct Procedure {
    pub stacks: Vec<Stack>,
    pub steps: Vec<Step>,
}

impl FromStr for Procedure {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_procedure(s).finish() {
            Ok((_remaining, procedure)) => Ok(procedure),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}
