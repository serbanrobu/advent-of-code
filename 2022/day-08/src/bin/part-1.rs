use color_eyre::Result;
use day_08::part_1;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    println!(
        "{}",
        part_1(input.lines().map(|l| l.as_bytes().to_vec()).collect())
    );

    Ok(())
}
