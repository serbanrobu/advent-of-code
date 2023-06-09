use nom::{
    branch::alt,
    character::complete::{char, newline, space1, u8},
    combinator::{map, value},
    error::Error,
    multi::separated_list0,
    sequence::separated_pair,
    Finish, IResult,
};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    iter::repeat,
    ops::Add,
    str::FromStr,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point [{} {}]", self.x, self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

pub fn part_1(motions: Motions) -> usize {
    let mut head_trail = vec![Point::default()];
    let mut tail_trail = vec![Point::default()];

    for (direction, steps) in motions.0 {
        let unit = Point::from(direction);

        for _ in 0..steps {
            let head = head_trail.last().copied().unwrap_or_default();
            let new_head = head + unit;
            head_trail.push(new_head);

            let tail = tail_trail.last().copied().unwrap_or_default();

            let new_tail = tail
                + Point {
                    x: new_head.x.cmp(&tail.x) as _,
                    y: new_head.y.cmp(&tail.y) as _,
                };

            if new_head == new_tail {
                continue;
            }

            tail_trail.push(new_tail);
        }
    }

    tail_trail.iter().collect::<HashSet<_>>().len()
}

pub fn part_2(motions: Motions) -> usize {
    let mut head_trail: Vec<_> = vec![Point::default()];
    let mut tail_trails: Vec<_> = repeat(vec![Point::default()]).take(9).collect();

    for (direction, steps) in motions.0 {
        let unit = Point::from(direction);

        for _ in 0..steps {
            let head = head_trail.last().copied().unwrap_or_default();
            let mut new_head = head + unit;
            head_trail.push(new_head);

            for tail_trail in tail_trails.iter_mut() {
                let tail = tail_trail.last().copied().unwrap_or_default();

                let new_tail = tail
                    + Point {
                        x: new_head.x.cmp(&tail.x) as _,
                        y: new_head.y.cmp(&tail.y) as _,
                    };

                if new_head == new_tail {
                    break;
                }

                tail_trail.push(new_tail);
                new_head = new_tail;
            }
        }
    }

    tail_trails
        .last()
        .unwrap()
        .iter()
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(input)
}

type Steps = u8;

type Motion = (Direction, Steps);

pub struct Motions(Vec<(Direction, Steps)>);

fn parse_motion(input: &str) -> IResult<&str, Motion> {
    separated_pair(parse_direction, space1, u8)(input)
}

fn parse_motions(input: &str) -> IResult<&str, Motions> {
    map(separated_list0(newline, parse_motion), Motions)(input)
}

impl FromStr for Motions {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_motions(s).finish() {
            Ok((_remaining, motions)) => Ok(motions),
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

    const MOTIONS: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_parse_motions() {
        let motions: Motions = MOTIONS.parse().unwrap();

        assert_eq!(
            motions.0,
            [
                (Direction::Right, 4),
                (Direction::Up, 4),
                (Direction::Left, 3),
                (Direction::Down, 1),
                (Direction::Right, 4),
                (Direction::Down, 1),
                (Direction::Left, 5),
                (Direction::Right, 2),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(MOTIONS.parse().unwrap());

        assert_eq!(answer, 13);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(MOTIONS.parse().unwrap());

        assert_eq!(answer, 1);

        let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

        let answer = part_2(input.parse().unwrap());

        assert_eq!(answer, 36);
    }
}
