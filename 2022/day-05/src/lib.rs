mod parser;
mod procedure;

use procedure::Procedure;

pub fn part_1(procedure: Procedure) -> String {
    let mut stacks = procedure.stacks;

    for step in procedure.steps {
        for _ in 0..step.crates_quantity {
            let Some(target_stack) = stacks.get_mut((step.target_stack - 1) as usize) else {
                break;
            };

            let Some(crate_) = target_stack.pop() else {
                break;
            };

            let Some(destination_stack) = stacks.get_mut((step.destination_stack - 1) as usize) else {
                break;
            };

            destination_stack.push(crate_);
        }
    }

    stacks
        .iter()
        .map(|s| s.last().map(|c| c.0).unwrap_or(' '))
        .collect()
}

pub fn part_2(procedure: Procedure) -> String {
    let mut stacks = procedure.stacks;

    for step in procedure.steps {
        let Some(target_stack) = stacks.get_mut((step.target_stack - 1) as usize) else {
            break;
        };

        let mut crates = target_stack
            .drain((target_stack.len() - step.crates_quantity as usize)..)
            .collect::<Vec<_>>();

        let Some(destination_stack) = stacks.get_mut((step.destination_stack - 1) as usize) else {
            break;
        };

        destination_stack.append(&mut crates);
    }

    stacks
        .iter()
        .map(|s| s.last().map(|c| c.0).unwrap_or(' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROCEDURE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part_1() {
        let answer = part_1(PROCEDURE.parse().unwrap());

        assert_eq!(answer, "CMZ".to_owned());
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(PROCEDURE.parse().unwrap());

        assert_eq!(answer, "MCD".to_owned());
    }
}
