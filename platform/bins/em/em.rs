pub mod config;
pub mod function_group_state;
//pub mod application;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let arg = config::argument::parse()?;
    let machine_manifest = config::configuration::load_machine_manifest(arg.config.as_str())?;
    let execution_manifest = config::configuration::load_execution_manifest(
        arg.config.as_str(),
        arg.rw_oara_root.as_str(),
    )?;

    let _ = config::configuration::validate_manifest(&machine_manifest, &execution_manifest)?;
    let fg_state = function_group_state::group::group(machine_manifest, execution_manifest)?;

    Ok(())
}
