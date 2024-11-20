use std::sync::Arc;

use anyhow::Result;
use std::error::Error;
use std::future::Future;
use thiserror::Error;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/*
/// [SWS_EM_02000] Definition of API enum `ara::exec::ExecutionState`
///
/// Defines the internal states of a Process (see 7.3.1).
/// Scoped enumeration of `uint8_t`.
///
/// # Values:
/// - `RUNNING = 0`: After a Process has been started by Execution Management,
///    it reports the `ExecutionState::RUNNING`.
pub enum ExecutionState {
    RUNNING = 0
}

/// [SWS_EM_02541]{DRAFT} Definition of API type ara::exec::ExecutionError d
/// Kind: type alias
/// Header file: #include "ara/exec/execution_error_event.h"
/// Scope: namespace ara::exec
/// Symbol: ExecutionError
/// Syntax: using ExecutionError = std::uint32_t;
/// Description: Represents the execution error.
type ExecutionError = u32;

/// [SWS_EM_02544] Definition of API class ara::exec::ExecutionErrorEvent
/// Kind: struct
/// Header file: #include "ara/exec/execution_error_event.h"
/// Forwarding header file: #include "ara/exec/exec_fwd.h"
/// Scope: namespace ara::exec
/// Symbol: ExecutionErrorEvent
/// Syntax: struct ExecutionErrorEvent final {...};
/// Description: Represents an execution error event which happens in a Function Group.
struct ExecutionErrorEvent {
    /// [SWS_EM_02545]{DRAFT} Definition of API variable ara::exec::ExecutionErrorEvent::executionError
    /// Symbol: executionError
    /// Type: ExecutionError
    /// Syntax: ExecutionError executionError;
    /// Description: The execution error of the Process which unexpectedly terminated .
    execution_error : ExecutionError,

    /// [SWS_EM_02546]{DRAFT} Definition of API variable ara::exec::ExecutionErrorEvent::functionGroup
    /// Symbol: functionGroup
    /// Type: ara::core::StringView
    /// Syntax: ara::core::StringView functionGroup;
    /// Description: The function group in which the error occurred .
    function_group: String,
}

#[derive(Error, Debug)]
enum ExecutionClientError {
    #[error("Communication error occurred.")]
    kCommunicationError,
    #[error("Given terminationHandler doesn’t contain a callable function.")]
    kInvalidArgument,
    #[error("Invalid transition request")]
    kInvalidTransition,
}

/// [SWS_EM_02001] Definition of API class ara::exec::ExecutionClient
/// Symbol: ExecutionClient
/// Syntax: class ExecutionClient final {...};
/// Description: Class to implement operations on Execution Client.
/// Notes: To eventually implement the Named Constructor Idiom, the developer may either make the
/// default constructor private or delete it and define a non-default constructor.
pub struct ExecutionClient {
    service_id : u16,
    method_id : u16,
    client_id : u16,

    termination_handler: Box<dyn Fn()>
}

impl Default for ExecutionClient {
    fn default() -> Self {
        Self {
            service_id: 0,
            method_id: 0,
            client_id: 0,
            termination_handler: Box::new(|| {}),
        }
    }
}

impl ExecutionClient {
    /// Symbol: ExecutionClient(std::function< void()> terminationHandler)
    /// Syntax: ExecutionClient (std::function< void()> terminationHandler);
    /// Parameters (in): terminationHandler Callback which is called if ExecutionClient receives SIGTERM
    /// signal. The callback is executed in a background thread. A typical
    /// implementation of this callback will set a global flag (and potentially
    /// unblock other threads) to perform a graceful termination. The
    /// lifetime of the given function has to exceed the lifetime of the
    /// ExecutionClient object.
    /// Exception Safety: not exception safe
    /// Errors: ara::exec::ExecErrc::kCommunicationError : Communication error occurred.
    ///         ara::exec::ExecErrc::kInvalidArgument : Given terminationHandler doesn’t contain a callable function.
    /// Description: Regular constructor for ExecutionClient.
    ///
    /// TBD : don't know what to do.. and how return errors
    pub fn new(termination_handler: Box<dyn Fn()>) -> Result<Self> {
        Ok(Self {
            termination_handler,
            ..Default::default()
        })
    }

    /// Symbol: Create(std::function< void()> terminationHandler)
    /// Syntax: static ara::core::Result< ExecutionClient > Create (std::function<void()> terminationHandler) noexcept;
    /// Parameters (in): terminationHandler Callback which is called if ExecutionClient receives SIGTERM
    /// signal. The callback is executed in a background thread. A typical
    /// implementation of this callback will set a global flag (and potentially
    /// unblock other threads) to perform a graceful termination. The
    /// lifetime of the given function has to exceed the lifetime of the
    /// ExecutionClient object.
    /// Return value: ara::core::Result<ExecutionClient >
    /// a result that contains either a ExecutionClient object or an error.
    /// Exception Safety: noexcept
    /// Errors ara::exec::ExecErrc::kCommunicationError : Communication error occurred.
    ///        ara::exec::ExecErrc::kInvalidArgument : Given terminationHandler doesn’t contain a callable function.
    /// Description: Named constructor for ExecutionClient.
    /// Notes: This named constructor may call a constructor defined by the developer.
    #[inline(always)]
    pub fn create(termination_handler: Box<dyn Fn()>) -> Result<Self> {
        ExecutionClient::new(termination_handler)
    }

    /// Symbol: ReportExecutionState(ExecutionState state)
    /// Syntax: ara::core::Result< void > ReportExecutionState (ExecutionState state) const noexcept;
    /// Parameters (in): state Value of the Execution State
    /// Return value: ara::core::Result< void > An instance of ara::core::Result. The instance holds an ErrorCode
    /// containing either one of the specified errors or a void-value.
    /// Exception Safety: noexcept
    /// Errors ara::exec::ExecErrc::kCommunicationError : Communication error between Application and Execution
    ///                                                   Management, e.g. unable to report state for Non-reporting Process.
    ///        ara::exec::ExecErrc::kInvalidTransition : Invalid transition request (e.g. to Running when already in Running state)
    /// Description: Interface for a Process to report its internal state to Execution Management.
    pub fn report_execution_state(state: ExecutionState) -> Result<()> {

        Ok(())
    }

}*/

pub struct ExecutionClient {
    signal_channel: Option<mpsc::Receiver<()>>,
}

impl ExecutionClient {
    fn new() -> Self {
        ExecutionClient {
            //signal_handler: None,
            signal_channel: None,
        }
    }

    /*async fn run_with_signal_handler(self, signal_handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to create SIGTERM handler");

        tokio::spawn(async move {
            sigterm.recv().await;
            signal_handler();
        });

        self
    }*/

    async fn run_with_signal_handler<F, Fut>(self, handler: F) -> Self
    where
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to create SIGTERM handler");

        tokio::spawn(async move {
            sigterm.recv().await;
            handler().await;
        });

        self
    }

    async fn run_with_channel(mut self) -> Self {
        let (tx, rx) = mpsc::channel::<()>(1);
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to create SIGTERM handler");
        self.signal_channel = Some(rx);

        tokio::spawn(async move {
            sigterm.recv().await;
            let _ = tx.send(()).await;
        });

        self
    }
}

mod tests {
    use super::*;

    async fn sigterm_handler() {
        println!("called ");
    }

    #[test]
    fn signal_handler() {
        // not sure how to test SIGTERM
        let execution_client = ExecutionClient::new().run_with_signal_handler(sigterm_handler);
    }

    #[test]
    fn signal_channel() {
        // not sure how to test SIGTERM
        let execution_client = ExecutionClient::new().run_with_channel();
    }
}
