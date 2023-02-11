use crate::{Config, Result};

pub fn run(config: Config) -> Result<()> {
    println!("{config}");
    Ok(())
}
