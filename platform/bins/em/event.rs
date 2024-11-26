pub mod execution_manager;
pub mod state_manager;

use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
pub enum ChangeStateError {
    #[error("Unknown Function group: {0}")]
    UnknownFunctionGroup(String),
    // TBD..
}

pub struct RequestChangeState {
    #[allow(unused)]    // FIXME
    function_group: String,
    #[allow(unused)]    // FIXME
    function_group_state: String,
    #[allow(unused)]    // FIXME
    response_channel: mpsc::Sender<Option<ChangeStateError>>,
}

impl RequestChangeState {
    pub fn new(
        function_group: String,
        function_group_state: String,
        response_channel: mpsc::Sender<Option<ChangeStateError>>,
    ) -> Self {
        Self {
            function_group,
            function_group_state,
            response_channel,
        }
    }
}
