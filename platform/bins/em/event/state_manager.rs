use super::RequestChangeState;
use crate::function_group_state::group::{group, InternalFgMode};
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use tokio::sync::mpsc;

use ara_exec::state_client::OARA_SM_DOMAIN_SOCKET;

// FIMXE :
//pub const OARA_SM_DOMAIN_SOCKET: &'static str = "/tmp/oara_sm_domain_socket";

/*
                                     .---------------.
                                     |      SM       |
                                     `---------------`
                                             |
                                             |
                                             |
            at start                         V
  .---------------.                  .----------------.
  | main thread   |                  | state_receiver | <----[IPC(Domain Socket)]
  `---------------`                  `----------------`
         ^ MachineFg/Startup                 ^ o
         |                                   | |
         |                                   | |
         +-----------------------------------` |
         |                                     |
      CHANNEL (TX/RX)                          |
         |                                     |
         V                                     |
  .---------------.                   .----------------. [static]
  | state_manager | o---------------> | COMMON RESOURCE |
  `---------------`                   `----------------`

    `state_manager` has two channels,
        1. Receive channel : to receive state change request
        2. Send channel : to return the result of state change request
*/

pub async fn state_manager(mut rx: mpsc::Receiver<RequestChangeState>) {
    while let mode = rx.recv().await {
        match mode {
            Some(mode) => {
                let functiong_group = mode.function_group;
                let functiong_group_state = mode.function_group_state;
            }
            None => {
                print!("mode change request channel is broken");
            }
        }
    }
}

pub async fn state_receiver() {
    let socket_path = OARA_SM_DOMAIN_SOCKET;

    let mut stream = UnixStream::connect(socket_path);
}
