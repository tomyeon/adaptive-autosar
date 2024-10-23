pub mod config;
//pub mod application;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let arg = config::argument::parse()?;
    let machine_manifest = config::configuration::load_machine_manifest(arg.config.as_str())?;

    Ok(())
}
