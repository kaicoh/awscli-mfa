use crate::{MfaConfig, Result};

pub fn run(config: MfaConfig) -> Result<()> {
    println!("{config}");
    Ok(())
}
