use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, newline, space1, u32},
    combinator::{map, recognize, value},
    error::Error,
    multi::{many1, separated_list0},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::str::FromStr;

fn make_filesystem(output: Output) -> Dir {
    let mut zipper = Zipper(
        Dir {
            name: "/".to_owned(),
            items: vec![],
        },
        vec![],
    );

    for line in output.0 {
        match line {
            OutputLine::Command(cmd) => match cmd {
                Command::Cd(path) => match path {
                    DirPath::Root => {
                        zipper = zipper.cd_root();
                    }
                    DirPath::Previous => {
                        zipper = zipper.cd_previous();
                    }
                    DirPath::Name(name) => {
                        zipper = zipper.cd(name);
                    }
                },
                Command::Ls => {
                    continue;
                }
            },
            OutputLine::Dir(name) => {
                zipper.0.items.push(Item::Dir(Dir {
                    name,
                    items: vec![],
                }));
            }
            OutputLine::File(size, _name) => {
                zipper.0.items.push(Item::File { size });
            }
        }
    }

    zipper.cd_root().0
}

pub fn part_1(output: Output) -> u32 {
    let root = make_filesystem(output);
    let mut dirs = root.subdirs();
    dirs.push(&root);

    dirs.into_iter()
        .map(Dir::size)
        .filter(|s| *s <= 100_000)
        .sum()
}

pub fn part_2(output: Output) -> u32 {
    let root = make_filesystem(output);
    let mut dirs = root.subdirs();
    dirs.push(&root);

    let total_space = 70_000_000;
    let used_space = root.size();
    let unused_space = total_space - used_space;
    let required_space = 30_000_000;
    let space_to_erase = if unused_space < required_space {
        required_space - unused_space
    } else {
        0
    };

    dirs.into_iter()
        .map(Dir::size)
        .filter(|s| *s > space_to_erase)
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Crumb(String, Vec<Item>, Vec<Item>);

#[derive(Debug)]
struct Zipper(Dir, Vec<Crumb>);

impl Zipper {
    fn cd_previous(mut self) -> Self {
        let Some(Crumb(name, prev, next)) = self.1.pop() else {
            return self;
        };

        let mut items = prev;
        items.push(Item::Dir(self.0));
        items.extend_from_slice(&next);

        Zipper(Dir { name, items }, self.1)
    }

    fn cd_root(self) -> Self {
        let mut zipper = self;

        while !zipper.1.is_empty() {
            zipper = zipper.cd_previous();
        }

        zipper
    }

    fn cd(self, name: String) -> Self {
        let Self(current_dir, crumbs) = self;

        for (index, item) in current_dir.items.iter().enumerate() {
            let Item::Dir(dir) = item else {
                continue;
            };

            if dir.name != name {
                continue;
            }

            let mut crumbs = crumbs;

            crumbs.push(Crumb(
                current_dir.name,
                current_dir.items[0..index].to_owned(),
                current_dir.items[index + 1..].to_owned(),
            ));

            return Zipper(dir.to_owned(), crumbs);
        }

        Self(current_dir, crumbs)
    }
}

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    items: Vec<Item>,
}

impl Dir {
    fn size(&self) -> u32 {
        self.items.iter().map(Item::size).sum()
    }

    fn subdirs(&self) -> Vec<&Dir> {
        self.items
            .iter()
            .filter_map(|i| match i {
                Item::Dir(d) => Some(d),
                _ => None,
            })
            .flat_map(|d| {
                let mut subdirs = d.subdirs();
                subdirs.push(d);
                subdirs
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
enum Item {
    Dir(Dir),
    File { size: u32 },
}

impl Item {
    fn size(&self) -> u32 {
        match self {
            Self::Dir(dir) => dir.size(),
            Self::File { size } => *size,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DirPath {
    Name(String),
    Previous,
    Root,
}

fn parse_dir_path(input: &str) -> IResult<&str, DirPath> {
    alt((
        value(DirPath::Root, char('/')),
        value(DirPath::Previous, tag("..")),
        map(alphanumeric1, |s: &str| DirPath::Name(s.to_owned())),
    ))(input)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Cd(DirPath),
    Ls,
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        value(Command::Ls, tag("ls")),
        map(preceded(tag("cd "), parse_dir_path), Command::Cd),
    ))(input)
}

#[derive(Debug, PartialEq)]
pub enum OutputLine {
    Command(Command),
    Dir(String),
    File(u32, String),
}

fn parse_output_line(input: &str) -> IResult<&str, OutputLine> {
    alt((
        map(preceded(tag("$ "), parse_command), OutputLine::Command),
        map(preceded(tag("dir "), alphanumeric1), |name: &str| {
            OutputLine::Dir(name.to_owned())
        }),
        map(
            separated_pair(
                u32,
                space1,
                recognize(many1(alt((alphanumeric1, tag("."))))),
            ),
            |(size, name): (_, &str)| OutputLine::File(size, name.to_owned()),
        ),
    ))(input)
}

#[derive(Debug, PartialEq)]
pub struct Output(Vec<OutputLine>);

fn parse_output(input: &str) -> IResult<&str, Output> {
    map(separated_list0(newline, parse_output_line), Output)(input)
}

impl FromStr for Output {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_output(s).finish() {
            Ok((_remaining, output)) => Ok(output),
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

    const OUTPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("cd x"),
            Ok(("", Command::Cd(DirPath::Name("x".to_owned()))))
        );

        assert_eq!(
            parse_command("cd .."),
            Ok(("", Command::Cd(DirPath::Previous)))
        );

        assert_eq!(parse_command("cd /"), Ok(("", Command::Cd(DirPath::Root))));

        assert_eq!(parse_command("ls"), Ok(("", Command::Ls)));
    }

    #[test]
    fn test_parse_output() {
        assert_eq!(
            "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
"
            .parse(),
            Ok(Output(vec![
                OutputLine::Command(Command::Cd(DirPath::Root)),
                OutputLine::Command(Command::Ls),
                OutputLine::Dir("a".to_owned()),
                OutputLine::File(14848514, "b.txt".to_owned()),
                OutputLine::File(8504156, "c.dat".to_owned()),
                OutputLine::Dir("d".to_owned()),
                OutputLine::Command(Command::Cd(DirPath::Name("a".to_owned()))),
            ]))
        )
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(OUTPUT.parse().unwrap());

        assert_eq!(answer, 95437);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(OUTPUT.parse().unwrap());

        assert_eq!(answer, 24933642);
    }
}
