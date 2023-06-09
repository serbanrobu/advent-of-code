use color_eyre::Result;
use day_03::part_2;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    println!("{}", part_2(input.parse()?));

    Ok(())
}
