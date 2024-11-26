//use super::RequestChangeState;
use crate::function_group_state::group::FunctionGroupHashMap;
//use std::io::{self, Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
//use tokio::sync::mpsc;
use anyhow::Result;
// /use serde::{Deserialize, Serialize};
use ara_exec::{
    function_group::{
        FunctionGroupState,
    },
    state_client::{
        OARA_SM_DOMAIN_SOCKET,
        SmClientCommand,
        InitialStateError,
        SmResponse,
    },
};
use once_cell::sync::OnceCell;

static INITIAL_STATE: OnceCell<bool> = OnceCell::new();

pub fn set_intial_state(value: bool) {
    INITIAL_STATE.set(value).expect("INITIAL_STATE can only be set once!");
}

pub fn get_intial_state() -> bool {
    *INITIAL_STATE.get().expect("INITIAL_STATE is not set!")
}

// FIMXE :
//pub const OARA_SM_DOMAIN_SOCKET: &'static str = "/tmp/oara_sm_domain_socket";

/*
                                     .---------------.
                                     |      SM       |
                                     `---------------`
                                             |
                                             | <----[IPC(Domain Socket)]
                                             |
            at start                         V
  .---------------.                  .----------------.
  | main thread   |                  | state_receiver |
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

/*pub async fn state_manager(mut rx: mpsc::Receiver<RequestChangeState>) {
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
}*/

pub fn set_state(fg_hashmap: &FunctionGroupHashMap, fg_state: FunctionGroupState) -> Result<()> {

    // change to off state except MachineFg

    match fg_hashmap.get(&fg_state.function_group) {
        Some(state_hashmap) => {
            match state_hashmap.get(&fg_state.function_group_state) {
                Some(manifests) => {
                    for _manifest in manifests {
                        // TODO
                        //manifest.
                    }
                }
                None => {
                    // TBD.
                }
            }
        }
        None => {
            // TBD.
        }
    }


    Ok(())
}

pub async fn state_receiver() -> Result<()> {
    let socket_path = OARA_SM_DOMAIN_SOCKET;

    if tokio::fs::metadata(&socket_path).await.is_ok() {
        tokio::fs::remove_file(&socket_path).await?;
    }

    let listener = UnixListener::bind(&socket_path)?;
    let (mut stream, _) = listener.accept().await?;

    let mut buffer = vec![0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(len) => {
                assert!(len > 0);
                let request_command =
                    bincode::deserialize::<SmClientCommand>(&buffer).unwrap();
                match request_command {
                    SmClientCommand::GetInitialState => {
                        let response = if get_intial_state() == false {
                            SmResponse::GetInitialState(Err(InitialStateError::FailedInitializeInitialState.into()))
                        } else {
                            SmResponse::GetInitialState(Ok(()))
                        };
                        let serialized_resonse = bincode::serialize(&response).unwrap();
                        stream.write(&serialized_resonse).await.unwrap();
                    }
                    SmClientCommand::SetState(_fg_state) => {
                        // FIXME
                        //let _ = set_state(fg_state);
                    }
                }
            }
            Err(error) => {
                panic!("error on read with '{:?}'", error);
            }
        }
        break;
    }

    Ok(())
}
