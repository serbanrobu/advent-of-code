use color_eyre::{eyre::ContextCompat, Result};
use day_06::part_2;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    println!(
        "{}",
        part_2(&input).wrap_err("expected a start-of-message marker")?
    );

    Ok(())
}
