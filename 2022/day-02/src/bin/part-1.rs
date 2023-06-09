use color_eyre::Result;
use day_02::part_1;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    println!("{}", part_1(input.parse()?));

    Ok(())
}
