use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("updraft", "abi/updraft.abi.json")?
        .generate()?
        .write_to_file("src/abi/updraft.rs")?;
    Abigen::new("idea", "abi/idea.abi.json")?
        .generate()?
        .write_to_file("src/abi/idea.rs")?;
    Abigen::new("solution", "abi/solution.abi.json")?
        .generate()?
        .write_to_file("src/abi/solution.rs")?;
    Ok(())
}