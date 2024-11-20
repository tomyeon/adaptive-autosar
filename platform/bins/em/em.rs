pub mod application;
pub mod config;
pub mod event;
pub mod function_group_state;

use std::sync::mpsc::Receiver;

use anyhow::Result;
use function_group_state::group::{group, InternalFgMode};
use tokio::spawn;
use tokio::sync::mpsc;

/*
                                           Something Structure to manage function group state for every group
                                           HashMap<String, String> // Function Group / State
                                               |- MachineFg, /Off, Startup, Restart, Shutdown ..
                                               |- Drving,    /Off, On, Verify ..

   request to chagen function group    Get current function group


   thread1             main thread      thread2
   ------              -----------     ---------
      |                    |               |
      |                    |               |
      |                    |            (StateClient Server)  <---------- SM
      |                    |               |
   change_state  <---------`---------------`
      ^
      |
      V
    ---------------------------------
   | shread resource to manage state |   <--------> thread3 (ExecutionClient Server)
    ---------------------------------

*/

#[tokio::main]
async fn main() -> Result<()> {
    let arg = config::argument::parse()?;
    let machine_manifest = config::configuration::load_machine_manifest(arg.config.as_str())?;
    let execution_manifest = config::configuration::load_execution_manifest(
        arg.config.as_str(),
        arg.rw_oara_root.as_str(),
    )?;

    let _ = config::configuration::validate_manifest(&machine_manifest, &execution_manifest)?;
    let fg_state = group(machine_manifest, execution_manifest)?;

    let (resp_tx, mut resp_rx) = mpsc::channel(1);
    let (tx, mut rx) = mpsc::channel::<event::RequestChangeState>(5);

    let _handle = tokio::spawn(event::state_manager::state_manager(rx));

    // let's change to MachineFg's Startup
    let machine_fg_startup =
        event::RequestChangeState::new("MachineFg".to_owned(), "Startup".to_owned(), resp_tx);
    tx.send(machine_fg_startup).await?;

    match resp_rx.recv().await {
        Some(response) => {
            if let Some(error) = response {
                println!("something wrong : {:?}", error);
                todo!();
            }
        }
        None => {
            println!("Channel might be broken")
        }
    }

    //let _handle = tokio::spawn(event::state_manager::state_receiver());
    Ok(())
}
